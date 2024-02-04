class Panel {
	/** # Creates a new panel
	* @param {string} id - The id of the panel
	* @returns {Panel} - The new panel
	*/
	constructor(id) {
		this.id = id
		this.content = document.createElement('section')
		this.visible = false
	}

	this_content() {
		return document.getElementById(this.id)
	}

	change_visibility() {
		this.visible = !this.visible
		this.toggle_visibility()
	}

	toggle_visibility() {
		if (this.visible) {
			this.content.style.display = 'flex'
		} else {
			this.content.style.display = 'none'
		}
	}

	/** # Replaces the content of the panel with the new content
	* @param {HTMLElement} new_content - The new content to be displayed
	* @returns {void}
	*/
	replace(new_content) {
		this.content.remove()
		this.content = new_content.cloneNode(true)
		document.body.appendChild(this.content)
		this.content.id = this.id
		this.toggle_visibility()
	}
}

export { Panel }
