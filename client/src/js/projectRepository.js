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
		return await fetch(request).then(response => {
			if (response.status === 401) {
				throw NOT_LOGGED_IN_ERROR;
			} else if (response.status === 200) {
				return response.json();
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
				return response.json();
			}
		}).catch(e => console.error(e))

	}

	async get(id) {

	}

	async grantAccess(project, user) {

	}

	async revokeAccess(project, user) {

	}

	/**
	 *
	 * @param project
	 * @return {Promise<Project>} string jeżeli był error
	 */
	async create(project) {
		const request = new Request(URI_TO_SERVER + "/projects", {
			method: "POST",
			body: JSON.stringify(project),
			credentials: "include",
			headers: new Headers({
				'content-type': 'application/json'
			})
		});
		const response = await fetch(request);
		if (response.status === 401) {
			throw NOT_LOGGED_IN_ERROR;
		}
		const body = await response.json();
		if (response.status === 200) {
			return body;
		} else {
			throw body;
		}
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

	constructor(id, name, description, owner) {
		this.id = id;
		this.name = name;
		this.description = description;
		this.owner = owner;
	}
}