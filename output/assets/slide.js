import { Panel } from './lib/panel.js'
import { ContentManager } from './lib/content_manager.js'

const HEADINGS = ['H1', 'H2', 'H3', 'H4', 'H5', 'H6']

/** # A class to represent the slide panel
* @extends Panel
*/
class Slide extends Panel {
	constructor() {
		super('slide')
		this.change_visibility()
		this.header = new ContentManager('title')
		this.body = new ContentManager('content')
	}

	/** # Replaces the content of the panel with the new content
	* @param {HTMLElement} new_content - The new content to be displayed
	* @returns {void}
	*/
	replace(new_content) {
		let header = document.createElement('header')
		let body = document.createElement('section')

		for (let node of Array.from(new_content.childNodes)) {
			if (node.tagName === 'DIV' || node.tagName === "SECTION") {
				continue
			} else if (HEADINGS.includes(node.tagName)) {
				header.appendChild(node.cloneNode(true))
			} else { body.appendChild(node.cloneNode(true)) }
		}

		let slide = document.createElement('section')
		header.id = 'title'
		body.id = 'content'
		slide.appendChild(header)
		slide.appendChild(body)
		super.replace(slide)
		this.notify()
	}

	/** # Notifies the observers of the slide panel to resize the header and body
	* @returns {void}
	*/
	notify() {
		this.header.notify()
		this.body.notify()
	}
}

export { Slide }
