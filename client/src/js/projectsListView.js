import ProjectsListController from "./projectsListController.js";
import {Project} from "./projectRepository.js";

export default class ProjectsListView {
	showMyProjects(projects) {
		const myProjectsWrapper = document.getElementById("my-projects-tbody");
		while (myProjectsWrapper.firstChild) {
			myProjectsWrapper.removeChild(myProjectsWrapper.firstChild);
		}
		projects.forEach(project => {
			myProjectsWrapper.appendChild(this.createTableRow(project, true))
		})
	}

	showSharedProjects(projects) {
		if(projects == null || projects.length() == 0) {
			return;
		}
		console.log("shared projects");
		console.log(projects);
		const mySharedProjectsWrapper = document.getElementById("shared-projects-tbody")
		while (mySharedProjectsWrapper.firstChild) {
			mySharedProjectsWrapper.removeChild(mySharedProjectsWrapper.firstChild);
		}
		projects.forEach(project => {
			mySharedProjectsWrapper.appendChild(this.createTableRow(project, false))
		})
	}

	constructor() {
		this.controller = new ProjectsListController(this);
		this.controller.loadProjects();
	}

	/**
	 *
	 * @param project {Project}
	 * @param createShareButton {boolean}
	 * @return {HTMLTableRowElement}
	 */
	createTableRow(project, createShareButton) {
		const row = document.createElement("tr");
		const projectNameCell = document.createElement("td");
		projectNameCell.innerText = project.name;
		row.appendChild(projectNameCell);

		const projectDescriptionCell = document.createElement("td");
		projectDescriptionCell.innerText = project.description;
		row.appendChild(projectDescriptionCell);

		const buttonsCell = document.createElement("td");
		buttonsCell.classList.add("my-project-buttons-wrapper");
		row.appendChild(buttonsCell);

		const openButton = document.createElement("button");
		openButton.classList.add("option-button")
		openButton.innerText = "Open";
		openButton.dataset["id"] = project.id + "";
		buttonsCell.appendChild(openButton);

		if (createShareButton) {
			const shareButton = document.createElement("button");
			shareButton.classList.add("option-button");
			shareButton.innerText = "Share";
			buttonsCell.appendChild(shareButton);
		}
		const deleteButton = document.createElement("button");
		deleteButton.classList.add("red-button");
		deleteButton.innerText = "Delete";
		buttonsCell.appendChild(deleteButton);

		return row;
	}
}