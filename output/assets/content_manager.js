class ContentManager {
	constructor(id) {
		this.id = id
		this.notify()
	}

	/** Get the content element to be managed */
	get_content() {
		this.content = document.getElementById(this.id)
	}

	/** Get the width of the parent
	* @returns {number} The width of the parent
	*/
	width() {
		return this.content.parentNode.clientWidth
	}

	/** Get the height of the parent 
	* @returns {number} The height of the parent
	*/
	height() {
		return this.content.parentNode.clientHeight
	}

	max_width() {
		let s = getComputedStyle(this.content).maxWidth
		return parseFloat(s) / 100
	}

	/** Get the maximum height of the content */
	max_height() {
		let s = getComputedStyle(this.content).maxHeight
		return parseFloat(s) / 100
	}

	/** Scale the content to fit the window */
	scale() {
		let element_w = Array.from(this.content.children)
			.map(child => child.clientWidth)
			.reduce((acc, w) => Math.max(acc, w))
		let element_h = this.content.clientHeight

		let scale_x = this.width() / element_w * this.max_width()
		let scale_y = this.height() / element_h * this.max_height()

		this.scala = Math.min(scale_x, scale_y)
		this.content.style.transform = `scale(${this.scala})`
	}

	/** To be called when something in the window changes */
	notify() {
		this.get_content()
		this.scale()
	}
}

export { ContentManager }
