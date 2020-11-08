import ProjectsListController from "./projectsListController.js";

export default class ProjectsListView{
	showMyProjects(projects){

	}

	showSharedProjects(projects){

	}

	constructor() {
		this.controller = new ProjectsListController(this);
		this.controller.loadProjects();
	}
}