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
		this.view.showFilesList([
			new File(1, "File 1"),
			new File(2, "File 2"),
			new File(3, "File 3"),
			new File(4, "File 4"),
			new File(5, "File 5")
		])
		this.view.showProjectInfo(new Project(1, "Testowy projekt","Testowy opis", {id:1,name:"Ala"}))
	}

	async loadProject() {
		//TODO
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
		}catch (e){
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

		this.webosocket.onerror = (e) =>{
			this.view.showError("Please make sure you are logged in and you have access to this project");
		}
	}
	parseMessage = (message) => {
		console.log(`Received message: '${message}'`)

	}
	disconnect = () => {
		if (this.webosocket) {
			console.log('Disconnecting...')
			this.webosocket.close()
			this.webosocket = null
		}
	}
}