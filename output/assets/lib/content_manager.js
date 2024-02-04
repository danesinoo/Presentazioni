class ContentManager {
	constructor(id) {
		this.id = id
	}

	/** # Get the content element to be managed */
	get_content() {
		this.content = document.getElementById(this.id)
	}

	/** # Get the width of the parent
	* @returns {number} The width of the parent
	*/
	width() {
		return this.content.parentNode.clientWidth
	}

	/** # Get the height of the parent 
	* @returns {number} The height of the parent
	*/
	height() {
		return this.content.parentNode.clientHeight
	}

	/** # Get the maximum width of the content, from css */
	max_width() {
		let s = getComputedStyle(this.content).maxWidth
		return parseFloat(s) / 100
	}

	/** # Get the maximum height of the content, from css */
	max_height() {
		let s = getComputedStyle(this.content).maxHeight
		return parseFloat(s) / 100
	}

	/** # Scale the content to fit the given space */
	scale() {
		let element_w = Array.from(this.content.children)
			.map(child => child.clientWidth)
			.reduce((acc, w) => Math.max(acc, w), 1)
		let element_h = this.content.clientHeight + 1

		let scale_x = this.width() / element_w * this.max_width()
		let scale_y = this.height() / element_h * this.max_height()

		this.scala = Math.min(scale_x, scale_y)
		this.content.style.transform = `scale(${this.scala})`
	}

	/** # Resize the content */
	notify() {
		this.get_content()
		this.scale()
	}
}

export { ContentManager }
