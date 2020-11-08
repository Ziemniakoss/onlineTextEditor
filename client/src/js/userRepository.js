import {URI_TO_SERVER} from "./constants"

export default class UserRepository {
}

export class User {
	/**
	 * Id of user in database
	 * @type {number}
	 */
	id;

	/**
	 * Name of user. Case sensitive and unique.
	 * @type {string}
	 */
	name;

	/**
	 * Creates new user
	 *
	 * @param id {number}
	 * @param name {string}
	 */
	constructor(id, name) {
		this.id = id;
		this.name = name;
	}
}