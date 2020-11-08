import {URI_TO_SERVER, NOT_LOGGED_IN_ERROR} from "./constants.js";

export default class ProjectRepository {
	async getAllOwned() {
		const request = new Request(URI_TO_SERVER + "/projects/my", {
			method: "GET",
			credentials: "include",
			headers: new Headers({
				'content-type': 'application/json'
			})
		})
		return fetch(request).then(response => {
			if (response.status === 401) {
				throw NOT_LOGGED_IN_ERROR;
			} else if (response.status === 200) {
				const projects = response.json();
				console.log(projects)
				console.log("Success");
				console.log(response.body);
				return projects;
			}
		}).catch(e => console.error(e))
	}

	async getAllSharedTo() {
		const request = new Request(URI_TO_SERVER + "/projects/shared-for-me", {
			method: "GET",
			credentials: "include"
		})
		return fetch(request).then(response => {
			if (response.status === 401) {
				throw NOT_LOGGED_IN_ERROR;
			} else if (response.status === 200) {
				const projects = response.json();
				console.log("Shared for me")
				console.log(projects);
				return projects;
			}
		}).catch(e => console.error(e))

	}

	async get(id) {

	}

	async grantAccess(project, user) {

	}

	async revokeAccess(project, user) {

	}
}

export class Project {
	/**
	 * @type {number}
	 */
	id;

	/**
	 * @type {string}
	 */
	name;

	/**
	 * @type {string | null}
	 */
	description;

	/**
	 * @type {User}
	 */
	owner;
}