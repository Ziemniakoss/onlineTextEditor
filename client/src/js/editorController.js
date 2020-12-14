import EditorView from "./editorView.js";
import FilesRepository from "./filesRepository.js";
import {File} from "./filesRepository.js";
import ProjectRepository from "./projectRepository.js";
import {Project} from "./projectRepository.js";

export default class EditorController {
	/**
	 * View for editor
	 *
	 * @type {EditorView}
	 */
	view;

	/**
	 * Websocket to server allowing to edit file and get
	 * events about currently loaded project
	 *
	 * @type {WebSocket}
	 */
	websocket;

	/**
	 * List of files in edited project
	 *
	 * @type {File []}
	 */
	files;

	/**
	 * Curently edited project
	 *
	 * @type {Project}
	 */
	project;

	/**
	 * State of editor. Values are taken from const EDITOR_STATES
	 *
	 * @type {number}
	 */
	state;

	/**
	 * Currently edited/loaded file.
	 *
	 * @type {File}
	 */
	openedFile;

	/**
	 * Repository of projects. Used only to get basic data about project like name and description
	 *
	 * @type {ProjectRepository}
	 */
	projectsRepository;

	/**
	 * Repository for files
	 */
	filesRepository;

	/**
	 * Map of all active sessions editing this project. Keys are session ids and
	 * values are objects with 2 fields: id and name.
	 *
	 * @type {Map.<string, object>}
	 */
	sessions

	/**
	 * Id of last applied change send by remote server. Used for Operation Transformation
	 * https://en.wikipedia.org/wiki/Operational_transformation
	 * @type {number} id of last applied change. Id is generated by remote server.
	 */
	lastAppliedChangeId;

	/**
	 * Creates new controller for editor
	 *
	 * @param view {EditorView}
	 */
	constructor(view) {
		this.view = view;
		this.projectsRepository = new ProjectRepository();
		this.filesRepository = new FilesRepository();
		let _ = this.init();
	}

	/**
	 * Tries to load project data and begin project editor session
	 * @return {Promise<void>}
	 */
	async init() {
		const projectId = new URL(window.location).searchParams.get("project_id")
		this.connect(projectId)
	}

	/**
	 * Will try to create new file in project. Opertaion will fal if network error occurs.
	 *
	 * @param name {string}
	 * @return {Promise<void>}
	 */
	async createNewFile(name) {
		this.webosocket.send(`1${name}`);
	}

	/**
	 * Try to rename file with given id. If there is already
	 * file with this name in this project operation will fail and
	 * error will be shown.
	 *
	 * @param id {number} id of file
	 * @param name {string} new name for file with this id
	 */
	async renameFile(id, name) {
		this.webosocket.send(`3${name}`);
	}

	/**
	 * Try to delete file with given id. Will fail if file with given id
	 * does not exist in given project
	 *
	 * @param id {number} id of file to delete
	 * @return {Promise<void>}
	 */
	async deleteFile(id) {
		console.log("Deleteing file " + id);
		this.webosocket.send(`2${id}`);
	}

	connect = (projectId) => {
		const wsUri =
			(window.location.protocol === 'https:' ? 'wss://' : 'ws://') +
			"localhost:5000" +
			'/projects/' + projectId + "/edit"
		console.log("Logging to project session " + projectId)
		this.webosocket = new WebSocket(wsUri)
		console.log('Connecting...')

		const t = this;
		this.webosocket.onopen = function () {
			console.log('Connected.')
		}
		this.webosocket.onmessage = function (e) {
			t.parseMessage(e.data);
		}

		this.webosocket.onclose = function (e) {
			console.log('Disconnected.');
			console.log(e);
			t.webosocket = null;
		}

		this.webosocket.onerror = (e) => {
			t.view.showError("Please make sure you are logged in and you have access to this project");
			t.webosocket = null;
		}
	}

	/**
	 * Converts change to format acceptable by remote server
	 *
	 * @param {Change} change
	 * @returns {string} change converted to server readable string
	 * @private
	 */
	_convertChangeToMessage = (change) => {
		return `5${this.openedFile.id} ${change.start.row} ${change.start.column} ${change.end.row} ${change.end.column} ${change.lastChangeApplied} ${change.lines.join("\n")}`
	}

	/**
	 * Parses message from remote server and invokes special handler for message with
	 * @param message
	 */
	parseMessage = (message) => {
		switch (message[0]) {
			case "1":
				this._handleNewSessionPackage(message.substring(1));
				break;
			case "2":
				this._handleSessionDisconnectedPackage(message.substring(1));
				break;
			case "3":
				this._handleNewFilePackage(message.substring(1));
				break;
			case "4":
				this._handleFileDeletedPackage(message.substring(1));
				break;
			case "5":
				this._handleFileContentPackage(message.substring(1));
				break;
			case "6":
				this._handleChangeInFilePackage(message.substring(1));
				break;
			case "9":
				this._handleProjectData(JSON.parse(message.substring(1)));
				break;
			case "a":
				this._handleErrorPackage(message.substring(1));
				break;
		}
	}

	_handleChangeInFilePackage = (message) => {
		let fileIdRangesAndChangeId = message.split(" ", 6);
		if (fileIdRangesAndChangeId.length !== 6) {
			console.error("Could not extract changes position, id and file id from incoming change in file package");
		}
		let [fileId, startRow, startColumn, endRow, endColumn, changeId] = fileIdRangesAndChangeId.map((str) =>parseInt(str));
		let startingIndexOfChangeContent = 6 + fileIdRangesAndChangeId.reduce((total, currentStr) => {return total + currentStr.length}, 0);
		let changeContent = message.substring(startingIndexOfChangeContent);
		if(this.openedFile == null || fileId !== this.openedFile.id){
			return;
		}
		this.lastAppliedChangeId = changeId;
		const range = new ace.Range(startRow, startColumn, endRow, endColumn)
		this.view.replaceText(range, changeContent)
	}


	_handleFileContentPackage(message) {
		const indexOfFirstSpace = message.indexOf(" ");
		const fileId = parseInt(message.substring(0, indexOfFirstSpace));
		if (fileId !== this.openedFile.id) {
			console.log(`Recived contetn of file ${fileId} but currently opened file is ${this.openedFile.id}`);
			return;
		}
		this.realFileContentSession = ace.createEditSession(message.substring(indexOfFirstSpace + 1));
		this.view.showFileContent(this.realFileContentSession.getValue());
	}

	_handleFileDeletedPackage(message) {
		const fileId = parseInt(message);
		console.log(`File ${fileId} was deleted`);
		this.files = this.files.filter(file => file.id !== fileId);
		this.view.showFilesList(this.files);
		if (this.openedFile != null && fileId === this.openedFile.id) {
			this.view.hideEditor();
		}
	}

	/**
	 *
	 * @param {string} message contains error message
	 * @private
	 */
	_handleErrorPackage(message) {
		this.view.showError(message);
	}

	/**
	 * New file was created on server
	 *
	 * @param {string} message string containing id and new file name separated by space
	 * @private
	 */
	_handleNewFilePackage(message) {
		const indexOfFirstSpace = message.indexOf(" ");
		const id = message.substring(0, indexOfFirstSpace);
		const name = message.substring(indexOfFirstSpace + 1);
		this.files.push(new File(id, name))
		this.files.sort(((a, b) => {
			if (a.name > b.name) {
				return 1
			} else if (a.name < b.name) {
				return -1
			}
			return 0;

		}))
		this.view.showFilesList(this.files);
	}

	/**
	 *
	 * @param {string} sessionId
	 * @private
	 */
	_handleSessionDisconnectedPackage(sessionId) {
		this.sessions.delete(sessionId);
		this.view.showSessions(this.sessions.values());
	}

	/**
	 *
	 * @param {string} message Message containing id and name of user separated by space
	 * @private
	 */
	_handleNewSessionPackage(message) {
		const indexOfFirstSpace = message.indexOf(" ");
		const sessionId = message.substring(0, indexOfFirstSpace);
		const name = message.substring(indexOfFirstSpace + 1);
		this.sessions.set(sessionId, {
			id: sessionId,
			name: name
		});
		this.view.showSessions(this.sessions.values());
	}

	/**
	 * Display project data
	 *
	 * @param projectData {object}
	 * @param {Project} projectData.project
	 * @param {object[]} projectData.sessions list of active session(containing current session)
	 * @param {string} projectData.sessions[].id id of session
	 * @param {string} projectData.sessions[].name  Name of user for given session
	 * @param {object []} projectData.files list of files in this project
	 * @param {number} projectData.files[].id id of file
	 * @param {string} projectData.files[].name name of file
	 * @private
	 */
	_handleProjectData(projectData) {
		this.sessions = new Map();
		projectData.sessions.forEach(session => {
			this.sessions.set(session.id, session);
		})
		this.view.showSessions(this.sessions.values());

		this.project = projectData.project;
		this.view.showProjectInfo(this.project);

		if (projectData.files != null) {
			this.files = projectData.files;
		} else {
			this.files = [];
		}
		this.view.showFilesList(this.files);
	}

	/**
	 * If file exists this will load file content
	 * @param {number} fileId
	 */
	fileSelectionChanged(fileId) {
		this.openedFile = null;
		for (const file of this.files) {
			if (file.id === fileId) {
				this.openedFile = file;
				break;
			}
		}
		if (this.openedFile != null) {
			this._loadFileContent();
		}
	}

	/**
	 * Send "get file content" package to server
	 * @private
	 */
	_loadFileContent() {
		this.webosocket.send(`4${this.openedFile.id}`);
	}

	realFileContent

	/**
	 * @typedef FileChange
	 * @property {Range} start
	 * @property {Range} end
	 * @property {"insert" | "remove"} action
	 * @property {string []} lines
	 *
	 *
	 * @typedef Range
	 * @property {number} row
	 * @property {number} column
	 * @param change
	 */

	/**
	 * @param {FileChange} fileChange
	 */
	handleChange(fileChange) {
		/** @type {Change}*/
		const change = {
			start: {
				row: fileChange.start.row,
				column: fileChange.start.column
			},
			end: fileChange.action ==="remove" ? fileChange.end : fileChange.start,
			lines: fileChange.action === "remove" ? [] : fileChange.lines,
			lastChangeApplied: this.lastAppliedChangeId
		}
		const message = this._convertChangeToMessage(change);
		this.webosocket.send(message);
	}

	// disconnect = () => {
	// 	if (this.webosocket) {
	// 		console.log('Disconnecting...')
	// 		this.webosocket.close()
	// 		this.webosocket = null
	// 	}
	// }
}