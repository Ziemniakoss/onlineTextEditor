import ProjectCreatorView from "./projectCreatorView.js";

const view = new ProjectCreatorView();

document.getElementById("submit-button").onclick =(e) =>{
	view.create();
}