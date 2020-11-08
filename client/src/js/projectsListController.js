import ProjectsListView from "./projectsListView.js";
import ProjectRepository from "./projectRepository.js";

export default class ProjectsListController {
	/**
	 * View to control
	 *
	 * @type {ProjectsListView}
	 */
	view;

	/**
	 * Creates new controller for view
	 *
	 * @param view {ProjectsListView}
	 */
	constructor(view) {
		this.view = view;
		this.projectsRepository = new ProjectRepository();
	}

	loadProjects() {
		console.log("Loading projects");
		this.projectsRepository.getAllOwned();
		this.projectsRepository.getAllSharedTo();
	}
}