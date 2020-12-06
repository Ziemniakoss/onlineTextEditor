export default class FileChangesCache {
	/**
	 * @typedef Change
	 * @property {Range} start
	 * @property {Range} end
	 * @property {string []} lines Content of lines modified in this change. If length of
	 * this array is smaller than end.columne - start.column that means that we removed some lines from
	 * file
	 * @property {number} lastChangeApplied
	 *
	 * @typedef Range
	 * @property {number} row starts at 0
	 * @property {number} column starts at 0
	 */
	localChanges = [];

	lastRemoteChange;


	/**
	 * Add change applied by user
	 * @param {Change} change
	 */
	addLocalChange(change) {
		//TODO merging local changes to reduce bandwidth consumption
		this.localChanges.push(change);
	}

	applyRemoteChange(change) {
		//TODO
	}

	/**
	 * Removes all local changes from memory and returns them
	 * @returns {Change []}
	 */
	removeAll() {
		const changes = this.localChanges;
		this.localChanges = [];
		return changes;
	}
}