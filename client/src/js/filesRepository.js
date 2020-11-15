export default class FilesRepository{
	/**
	 * For what project this repository may manage files
	 *
	 * @type {number}
	 */
	projectId;

	async create(file){

	}

	async update(file){

	}

	async delete(file){

	}

	constructor(projectId) {
		this.projectId = projectId;
	}
}

export class File{
	/**
	 * Id of file in database. Set automatically during creation
	 *
	 * @type {number}
	 */
	id;

	/**
	 * Name of file. Name must be unique in project scope
	 *
	 * @type {string}
	 */
	name;
}