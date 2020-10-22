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
    name       CHAR(100),
    project_id INT NOT NULL REFERENCES projects (id) ON DELETE CASCADE ON UPDATE CASCADE
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
    line_number INT NOT NULL CHECK ( line_number > 0 ),
    content     TEXT,
    PRIMARY KEY (file_id, line_number)
);


/**
 * TODO Funkcje do:
 * - do tworzenia użytkownika
 * - tworzenia projektu
 * - tworzenia pliku
 * - sprawdzania czy użytkownik ma dostęp do projektu
 * - dawania dostępu
 * - usuwania dostępu
 * - dodawania n nowych linii w dokumencie
 * - pobrania danych o całym projekcie
 * - zapaisania wartości linijki w bazie danych
 * - pobieranie danych o dostępnych projetach(nazwa, id, nazwa użytkownika, opis)
 */