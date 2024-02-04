class Observable {
	constructor() {
		this.obs = []
	}

	/** Add an observer to the list of observers
	* @param {Observer} observer - The observer to be added
	*/
	add_observer(observer) {
		this.obs.push(observer)
	}

	/** Remove an observer from the list of observers
	* @param {Observer} observer - The observer to be removed
	* @return {undefined}
	*/
	remove_observer(observer) {
		this.obs = this.obs.filter(o => o !== observer)
	}

	/** Notify all observers
	* @return {undefined}
	* */
	update() {
		for (let observer of this.obs)
			observer.notify()
	}
}

export { Observable }
