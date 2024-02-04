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
			.find(node => node.tagName === "SECTION")

		if (!comment) {
			comment = document.createElement('section')
		}

		comment.id = 'comment'
		super.replace(comment)
	}

}

export { Comment }
