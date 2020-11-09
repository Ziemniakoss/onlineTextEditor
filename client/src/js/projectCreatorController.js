import ProjectCreatorView from "./projectCreatorView.js";
import ProjectRepository from "./projectRepository.js";
import {Project} from "./projectRepository.js";


export default class ProjectCreatorController {
	/**
	 * @type {ProjectCreatorView}
	 */
	view;

	/**
	 * @type {ProjectRepository}
	 */
	projectRepository;

	constructor(view) {
		this.view = view;
		this.projectRepository = new ProjectRepository();
	}

	/**
	 * Creates project
	 *
	 * @param name {string} name of project
	 * @param description {string} description of project, can be null
	 */
	async create(name, description){
		let project = new Project();
		project.name = name;
		project.description = description;
		await this.projectRepository.create(project).then(project =>{
			console.log("Created project " + JSON.stringify(project));
			window.location.href = "projects-list.html";
		}).catch(error =>{
			console.error(error);
			this.view.showError(error);
		})
	}
}