import EditorView from "./editorView.js";
import FilesRepository from "./filesRepository.js";
import File from "./filesRepository.js";
import ProjectRepository from "./projectRepository";

const EDITOR_STATES = Object.freeze({
	LOADING_PROJECT_STRUCTURE: 0,
	NO_FILE_OPENED: 1,
	FILE_OPENED:2,
	LOADING_FILE_CONTENT:3,
	EDITING_FILE: 4,

})

export default class EditorController {
	/**
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
	 * Ace editor.
	 */
	editor;

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
	}

	async loadProject() {
		//TODO
	}

	/**
	 * Will try to create new file in project. Operation will fail if there already is file with same name
	 * in this projct
	 * @param name {string}
	 * @return {Promise<void>}
	 */
	async createNewFile(name) {
		//TODO

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
		//TODO
	}

	/**
	 * Try to delete file with given id. Will fail if file with given id
	 * does not exist in given project
	 *
	 * @param id {number} id of file to delete
	 * @return {Promise<void>}
	 */
	async deleteFile(id) {

	}

	/**
	 * Sends message in current project context. All users curently
	 * editing this project will see this message
	 *
	 * @param message {string}
	 * @return {Promise<void>}
	 */
	async sendMessage(message) {
		//TODO*
	}
}