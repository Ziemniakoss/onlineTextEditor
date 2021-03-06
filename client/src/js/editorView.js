import EditorController from "./editorController.js";
import {Project} from "./projectRepository.js";
import {File} from "./filesRepository.js";


export default class EditorView {
	/**
	 * Controller for this view
	 *
	 * @type {EditorController}
	 */
	controller;

	editor;

	editedFileId = null;

	constructor() {
		this.controller = new EditorController(this);
		this.editor = ace.edit("editor");
		//TODO uncoment when fixed document.getElementById("editor").id = "editor-disabled"
		this.init()
	}

	init() {
		document.getElementById("theme-selector").addEventListener("change", (event) => {
			const newTheme = event.target.value
			this.editor.setTheme(`ace/theme/${newTheme}`);
			localStorage.setItem("theme", newTheme);
		})
		this._selectInitialTheme();
		this.editor.session.setMode("ace/mode/text");

		this.editor.session.on("change", (c) => {
			if(c.cancelSignal){
				return;
			}
			this.controller.handleChange(c);
		})

		document.getElementById("new-file-button").addEventListener("click", (_) => {
			const newName = document.getElementById("new-file-name").value;
			this.controller.createNewFile(newName);
		})
	}

	simulateChange(range, content){
		this.editor.session.replace(range, content)
	}

	_selectInitialTheme() {
		let theme = localStorage.getItem("theme");
		if (theme == null) {
			theme = "dracula";
			localStorage.setItem("theme", theme);
		}
		this.editor.setTheme(`ace/theme/${theme}`);
		document.getElementById("theme-selector").value = theme;
	}

	/**
	 * Shows error as alert and prints it to console.
	 * Later wh can change it to show notification
	 *
	 * @param message {string}
	 */
	showError(message) {
		console.error(message);
		alert(message);
	}

	/**
	 * @typedef Session
	 * @property {string} name name of user in this session
	 * @property {number} id id of session
	 */

	/**
	 * Show session list
	 *
	 * @param {IterableIterator<Session>} sessions
	 */
	showSessions(sessions) {
		let sessionListElement = document.getElementById("users-list");
		while (sessionListElement.firstChild) {
			sessionListElement.removeChild(sessionListElement.firstChild);
		}
		for (const session of sessions) {
			const sessionDomElement = document.createElement("li");
			sessionDomElement.textContent = session.name;
			sessionListElement.appendChild(sessionDomElement);
		}
	}

	/**
	 * Displays files in left panel
	 *
	 * @param files {File []}
	 */
	showFilesList(files) {
		const listElement = document.getElementById("project-files-list");
		while (listElement.firstChild) {
			listElement.removeChild(listElement.firstChild);
		}

		files.forEach(file => {
			const fileListElement = document.createElement("li");
			fileListElement.classList.add("file-list-element")

			/** @type {HTMLDivElement} */
			const fileNameDiv = document.createElement("div");
			fileNameDiv.textContent = file.name;
			fileNameDiv.title = file.name;
			fileNameDiv.onclick = (_) => this._handleFileClick(file.id);
			fileNameDiv.classList.add("file-name-label")
			fileListElement.appendChild(fileNameDiv);


			/** @type {HTMLButtonElement} */
			const renameButton = document.createElement("button");
			renameButton.classList.add("option-button");
			renameButton.textContent = "R";
			renameButton.title = "Rename(not implemented)"
			renameButton.dataset["id"] = `${file.id}`
			fileListElement.appendChild(renameButton);

			/** @type {HTMLButtonElement} */
			const deleteButton = document.createElement("button");
			deleteButton.classList.add("red-button");
			deleteButton.textContent = "D";
			deleteButton.title = "Delete file";
			fileListElement.appendChild(deleteButton);
			deleteButton.onclick = (_) => this.controller.deleteFile(file.id);

			listElement.appendChild(fileListElement);
		})
	}

	/**
	 * Sends "file was selected signal" signal to controller
	 * @param {number} fileId id of file to load
	 * @private
	 */
	_handleFileClick(fileId) {
		const possibleDisabledEditor = document.getElementById("editor-disabled");
		if (possibleDisabledEditor != null) {
			possibleDisabledEditor.id = "editor";
		}
		this.controller.fileSelectionChanged(fileId);
	}

	/**
	 *
	 * @param project {Project}
	 */
	showProjectInfo(project) {
		document.getElementById("project-name").innerText = project.name
		document.getElementById("project-description").innerText = project.description

	}

	/**
	 *
	 * @param {string} content file content as string
	 */
	showFileContent = (content) => {
		const cursorPreChange = this.editor.selection.getCursor();
		this.editor.setValue(content, null, true);
		this.editor.gotoLine(cursorPreChange.row + 1, cursorPreChange.column)
	}

	/**
	 * @param {EditorMode} mode requested mode for editor. If null it will be set to plaintext mode
	 */
	setEditorMode(mode) {
		if (mode == null) {
			mode = EditorModes.TEXT;
		}
		console.log("SETTING MODE TO " + mode);
		this.editor.session.setMode(`ace/mode/${mode}`);

	}

	replaceText(range, text){
		this.editor.session.replace(range, text, true, false);
	}

	/**
	 * Hides editor. Useful when for example edited file was deleted;
	 */
	hideEditor() {
		const editor = document.getElementById("editor");
		if (editor != null) {
			editor.id = "editor-disabled";
		}
	}
}