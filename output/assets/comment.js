import { Panel } from './lib/panel.js'

/** # A class to represent the comment panel
* @extends Panel
*/
class Comment extends Panel {
	constructor() {
		super('comment')
	}

	/** # Replace the content of the comment panel
	* @param {HTMLElement} new_content - The new content to be shown
	*/
	replace(new_content) {
		let comment = Array.from(new_content.children)
			.filter(node => node.tagName === "SECTION")
			.reduce((acc, node) => {
				Array.from(node.children).forEach(child => {
					acc.appendChild(child.cloneNode(true))
				})
				return acc
			}, document.createElement('section'))

		super.replace(comment)
	}

}

export { Comment }
