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

	async loadProjects() {
		const myProjectsRequest = this.projectsRepository.getAllOwned();
		const sharedProjectsRequest = await this.projectsRepository.getAllSharedTo();
		this.view.showMyProjects(await  myProjectsRequest);
		this.view.showSharedProjects(await sharedProjectsRequest);
	}
}