CREATE TABLE users
(
	id            SERIAL PRIMARY KEY,
	name          CHAR(200) NOT NULL UNIQUE,
	password_hash CHAR(32)  NOT NULL
);

CREATE TABLE projects
(
	id          SERIAL PRIMARY KEY,
	name        CHAR(200) NOT NULL,
	description CHAR(400),
	owner_id    INT       NOT NULL REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE files
(
	id         SERIAL PRIMARY KEY,
	name       CHAR(100) NOT NULL CHECK (length(name) > 0),
	project_id INT       NOT NULL REFERENCES projects (id) ON DELETE CASCADE ON UPDATE CASCADE,
	unique (name, project_id)
);

create TABLE projects_shared_for_users
(
	user_id    INT NOT NULL REFERENCES users (id) ON DELETE CASCADE ON UPDATE CASCADE,
	project_id INT NOT NULL REFERENCES projects (id) ON DELETE CASCADE ON UPDATE CASCADE,
	PRIMARY KEY (user_id, project_id)
);

CREATE TABLE files_lines
(
	file_id     INT NOT NULL REFERENCES files (id) ON DELETE CASCADE ON UPDATE CASCADE,
	line_number INT NOT NULL CHECK ( line_number >= 0 ),
	content     TEXT DEFAULT '',
	PRIMARY KEY (file_id, line_number)
);

CREATE OR REPLACE FUNCTION create_user(_username varchar, _password_plain varchar) RETURNS INT
	language plpgsql AS
$body$
DECLARE
	_password_hash CHAR;
BEGIN
	IF EXISTS(SELECT name FROM users) THEN
		RETURN -1;
	END IF;
	IF length(_username) < 5 THEN
		RETURN -2;
	END IF;
	_password_hash = md5(_password_plain);
	INSERT INTO users (name, password_hash) VALUES (_username, _password_hash) RETURNING id;
	RETURN 0;
END;
$body$;

CREATE OR REPLACE FUNCTION try_login(_username CHAR, _password_plain CHAR) RETURNS INT
	LANGUAGE plpgsql AS
$body$
DECLARE
	_id            INT;
	_password_hash CHAR;
BEGIN
	_password_hash = md5(_password_plain);
	SELECT id FROM users WHERE name = _username AND password_hash = _password_hash INTO _id;
	IF _id IS NOT NULL THEN
		RETURN _id;
	END IF;
	RETURN -1;
END;
$body$;

CREATE OR REPLACE FUNCTION create_project(_name CHAR, _description CHAR, _owner_id INT) RETURNS INT
	LANGUAGE plpgsql AS
$body$
DECLARE
	_id INT;
BEGIN
	IF NOT EXISTS(SELECT id FROM users WHERE id = _owner_id) THEN
		RETURN -1;
	end if;
	IF _name IS NULL OR length(_name) = 0 THEN
		RETURN -2;
	END IF;
	IF EXISTS(SELECT id FROM projects WHERE name = _name AND owner_id = _owner_id) THEN
		RETURN -3;
	END IF;
	INSERT INTO projects (name, description, owner_id) VALUES (_name, _description, _owner_id) RETURNING id INTO _id;
	RETURN _id;
END;
$body$;

CREATE OR REPLACE FUNCTION grant_access_to_project(_project_id INT, _user_id INT) RETURNS INT
	LANGUAGE plpgsql AS
$body$
DECLARE
	_owner_id INT;
BEGIN
	SELECT owner_id FROM projects WHERE id = _project_id INTO _owner_id;
	IF _owner_id IS NULL THEN
		RETURN -1;
	END IF;
	IF _owner_id = _user_id OR
	   EXISTS(SELECT user_id FROM projects_shared_for_users WHERE user_id = _user_id AND project_id = _project_id) THEN
		RETURN 0;
	END IF;
	INSERT INTO projects_shared_for_users (user_id, project_id) VALUES (_user_id, _project_id);
	RETURN 0;
END;
$body$;

CREATE OR REPLACE FUNCTION revoke_access_to_project(_project_id INT, _user_id INT) RETURNS INT
	LANGUAGE plpgsql AS
$body$
BEGIN
	IF EXISTS(SELECT id FROM projects WHERE id = _project_id AND owner_id = _user_id) THEN
		RETURN -1;
	END IF;
	DELETE FROM projects_shared_for_users WHERE project_id = _project_id AND user_id = _user_id;
	RETURN 0;
END;
$body$;

CREATE OR REPLACE FUNCTION has_access_to_project(_project_id INT, _user_id INT) RETURNS BOOLEAN
	LANGUAGE plpgsql
AS
$body$
BEGIN
	IF EXISTS(SELECT id FROM projects WHERE id = _project_id AND owner_id = _user_id) THEN
		RETURN TRUE;
	END IF;
	RETURN EXISTS(
			SELECT project_id FROM projects_shared_for_users WHERE project_id = _project_id AND user_id = _user_id);
END;
$body$;

CREATE OR REPLACE FUNCTION create_file(_project_id INT, _name CHAR) RETURNS INT
	LANGUAGE plpgsql AS
$body$
DECLARE
    _file_id INT;
BEGIN
	IF _name IS NULL OR length(_name) = 0 THEN
		RETURN -1;
	END IF;
	IF NOT EXISTS(SELECT id FROM projects WHERE id = _project_id) THEN
		RETURN -2;
	END IF;
	IF EXISTS(SELECT id FROM files WHERE project_id = _project_id AND name = _name) THEN
		RETURN -3;
	END IF;
	INSERT INTO files (name, project_id) VALUES (_name, _project_id) RETURNING id INTO _file_id;
	RETURN _file_id;
END;
$body$;

CREATE OR REPLACE FUNCTION update_file(_id INT, _new_name CHAR) RETURNS INT
	LANGUAGE plpgsql AS
$body$
DECLARE
	_old_name   CHAR;
	_project_id INT;
BEGIN
	IF _new_name IS NULL OR length(_new_name) = 0 THEN
		RETURN -1;
	END IF;
	SELECT name FROM files WHERE id = _id INTO _old_name;
	IF _old_name IS NULL THEN
		RETURN -2;
	end if;
	If _old_name = _new_name THEN
		RETURN 0;
	END IF;
	SELECT project_id FROM files WHERE id = _id INTO _project_id;
	IF EXISTS(SELECT id FROM files WHERE id <> _id AND name = _new_name AND project_id = _project_id) THEN
		RETURN -3;
	END IF;
	UPDATE files SET name = _new_name WHERE id = _id;
	RETURN 0;
END;
$body$;


CREATE OR REPLACE FUNCTION files_lines_after_delete() RETURNS TRIGGER
	LANGUAGE plpgsql AS
$bodt$
DECLARE
	_line_number INT;
BEGIN
	IF NOT EXISTS(SELECT file_id FROM files_lines WHERE file_id = OLD.file_id AND line_number > OLD.line_number) THEN
		RETURN NEW;
	END IF;
	FOR _line_number IN
		SELECT line_number
		FROM files_lines
		WHERE file_id = OLD.file_id
		  AND line_number > OLD.line_number
		ORDER BY line_number
		LOOP
			UPDATE files_lines
			SET line_number = (_line_number - 1)
			WHERE line_number = _line_number
			  AND file_id = OLD.file_id;
		END LOOP;
	RETURN NEW;
END;
$bodt$;

COMMENT ON FUNCTION files_lines_after_delete() IS 'Move lines if deleted line was not at the end of file';

CREATE TRIGGER files_lines_after_delete_trigger
	AFTER DELETE
	ON files_lines
	FOR EACH ROW
EXECUTE PROCEDURE files_lines_after_delete();

CREATE OR REPLACE FUNCTION insert_line_in_file(_file_id INT, _line_number INT, _content CHAR) RETURNS INT
	LANGUAGE plpgsql
AS
$body$
DECLARE
	_current_line_number INT;
BEGIN
	IF _line_number < 0 THEN
		RETURN -1;
	END IF;
	IF NOT EXISTS(SELECT id FROM files WHERE id = _file_id) THEN
		RETURN -2;
	END IF;
	--- add lines before this line if they dont exist
	_current_line_number := _line_number - 1;
	WHILE _current_line_number >= 0 AND NOT EXISTS(
			SELECT *
			FROM files_lines
			WHERE file_id = _file_id
			  AND line_number = _current_line_number
		)
		LOOP
			INSERT INTO files_lines (file_id, line_number, content) VALUES (_file_id, _current_line_number, '');
			_current_line_number := _current_line_number - 1;
		END LOOP;
	--- move following lines by one
	FOR _current_line_number IN
		SELECT line_number
		FROM files_lines
		WHERE line_number >= _line_number
		  AND file_id = _file_id
		ORDER BY line_number DESC
		LOOP
			UPDATE files_lines
			SET line_number = _current_line_number + 1
			WHERE file_id = _file_id
			  AND line_number = _current_line_number;
		END LOOP;
	INSERT INTO files_lines (file_id, line_number, content) VALUES (_file_id, _line_number, _content);
	RETURN 0;
END;
$body$;