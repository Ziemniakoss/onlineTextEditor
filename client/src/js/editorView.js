import EditorController from "./editorController.js";

export default class EditorView{
	/**
	 * Controller for this view
	 * @type {EditorController}
	 */
	controller;

	constructor() {
		this.controller = new EditorController(this);
	}

	init(){
		this.controller.loadProject();
	}

	/**
	 * Shows error as alert and prints it to console.
	 * Later wh can change it to show notification
	 *
	 * @param message {string}
	 */
	showError(message){
		console.error(message);
		alert(message);
	}

	showFilesList(files){

	}

}