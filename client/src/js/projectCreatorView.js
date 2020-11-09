import ProjectCreatorController from "./projectCreatorController.js";

export default class ProjectCreatorView{
	constructor() {
		this.controller = new ProjectCreatorController(this)
	}

	create(){
		const name = document.getElementById("name-input").value.trim();
		const description = document.getElementById("description-input").value.trim();
		this.controller.create(name, description);
	}

	showError(message){
		alert(message);//TODO może jakiś popup
	}
}