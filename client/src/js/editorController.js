import EditorView from "./editorView.js";
import FilesRepository from "./filesRepository.js";
import {File} from "./filesRepository.js";
import ProjectRepository from "./projectRepository.js";
import {Project} from "./projectRepository.js";

const EDITOR_STATES = Object.freeze({
	LOADING_PROJECT_STRUCTURE: 0,
	NO_FILE_OPENED: 1,
	FILE_OPENED: 2,
	LOADING_FILE_CONTENT: 3,
	EDITING_FILE: 4,
	ERROR: 5

})

export default class EditorController {
	/**
	 * View for editor
	 *
	 * @type {EditorView}
	 */
	view;

	/**
	 * Weboscket to server allowing to edit file and get
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
	 * Creates new controller for editor
	 *
	 * @param view {EditorView}
	 */
	constructor(view) {
		this.view = view;
		this.projectsRepository = new ProjectRepository();
		this.filesRepository = new FilesRepository();
		this.init();
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
		this.webosocket.send(`2${id}`);
	}

	connect = (projectId) => {
		this.disconnect()
		const wsUri =
			(window.location.protocol === 'https:' ? 'wss://' : 'ws://') +
			"localhost:5000" +
			'/projects/' + projectId + "/edit"
		console.log("Logging to project session " + projectId)
		try {
			this.webosocket = new WebSocket(wsUri)
		} catch (e) {
			this.view.showError(JSON.stringify(e));
		}
		console.log('Connecting...')

		const t = this;
		this.webosocket.onopen = function () {
			console.log('Connected.')
			t.state = EDITOR_STATES.NO_FILE_OPENED
		}

		this.webosocket.onmessage = function (e) {
			t.parseMessage(e.data);
		}

		this.webosocket.onclose = function () {
			console.log('Disconnected.')
			t.webosocket = null
		}

		this.webosocket.onerror = (e) => {
			this.view.showError("Please make sure you are logged in and you have access to this project");
		}
	}
	parseMessage = (message) => {
		console.log(`Received message: '${message}'`)
		switch (message[0]) {
			case "1":
				this._handleNewSessionPackage(message.substring(1));
				break;
			case "2":
				this._handleSessionDisconnectedPackage(message.substring(1));
				break;
			case "9":
				this._handleProjectData(JSON.parse(message.substring(1)));
				break;
		}

	}

	/**
	 *
	 * @param {string} sessionId
	 * @private
	 */
	_handleSessionDisconnectedPackage(sessionId){
		this.sessions.forEach((val, key) =>{
			console.log(`${key} ses ${val.name} ${key === val.id}`)
		})
		this.sessions.delete(sessionId);
		this.view.showSessions(this.sessions.values());
	}

	/**
	 *
	 * @param {string} message Message containing id and name of user separated by space
	 * @private
	 */
	_handleNewSessionPackage(message){
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
		projectData.sessions.forEach(session =>{
			this.sessions.set(session.id, session);
		})
		this.view.showSessions(this.sessions.values());

		this.project = projectData.project;
		this.view.showProjectInfo(this.project);

		if(projectData.files != null){
			this.files = projectData.files;
		}else{
			this.files = [];
		}
		this.view.showFilesList(this.files);
	}

	disconnect = () => {
		if (this.webosocket) {
			console.log('Disconnecting...')
			this.webosocket.close()
			this.webosocket = null
		}
	}
}