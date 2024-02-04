import { Slide } from './slide.js'
import { Comment } from './comment.js'
import { Decorations } from './decorations.js'

class Navigation {
	constructor() {
		this.slide = new Slide()
		this.comment = new Comment()
		this.decorations = new Decorations(this)
		this.change_slide(document.getElementsByTagName('div')[0])
	}

	/** # Resize the slide. 
	* It used to be managed with an observer.
	* Actually, I might prefer to the previous version
	*/
	update() {
		this.slide.notify()
	}

	/** # Change the current slide
	* @param {HTMLElement} new_slide - The new slide to be shown
	*/
	change_slide(new_slide) {
		this.actual = new_slide
		this.slide.replace(new_slide)
		this.comment.replace(new_slide)

		// update the bars in the sides
		this.decorations.toggle_all()
	}

	/** # Check if the current slide has a next sibling
	* @returns {HTMLElement} - The next sibling of the current slide
	* or null if there is no next sibling
	*
	* if the current slide is a section, it will return the next section
	* if the current slide is a subsection, it will return the next subsection
	* ...
	*/
	has_next() {
		let siblings = Array.from(this.actual.parentElement.childNodes)
			.filter(node => node.tagName === 'DIV')
		let i = siblings.indexOf(this.actual)

		if (i >= siblings.length - 1)
			return null
		return siblings[i + 1]
	}

	/** # Check if the current slide has a previous sibling
	* @return {HTMLElement} - The previous sibling of the current slide
	* or null if there is no previous sibling
	*
	* if the current slide is a section, it will return the previous section
	* if the current slide is a subsection, it will return the previous subsection
	* ...
	*/
	has_previous() {
		let siblings = Array.from(this.actual.parentElement.childNodes)
			.filter(node => node.tagName === 'DIV')

		let i = siblings.indexOf(this.actual)
		if (i < 1)
			return null
		return siblings[i - 1]
	}

	/** # Check if the current slide has a subsection
	* @return {HTMLElement} - The first subsection of the current slide
	* or null if there is no subsection
	*
	* if the current slide is a section, it will return the first subsection
	* if the current slide is a subsection, it will return the first subsubsection
	* ...
	*/
	has_sub() {
		let subsection = Array.from(this.actual.childNodes).filter(node => node.tagName === 'DIV')
		if (subsection.length > 0)
			return subsection[0]
		return null
	}

	/** # Check if the current slide has a supersection
	* @return {HTMLElement} - The parent of the current slide
	* or null if there is no supersection
	*
	* if the current slide is a section, it will return null
	* if the current slide is a subsection, it will return its section
	* ...
	*/
	has_super() {
		if (this.actual.parentElement.tagName === 'DIV')
			return this.actual.parentElement
		return null
	}

	/** # If the current slide has a next sibling, change to it */
	next() {
		let next = this.has_next()
		if (next)
			this.change_slide(next)
	}

	/** # If the current slide has a previous sibling, change to it */
	previous() {
		let previous = this.has_previous()
		if (previous)
			this.change_slide(previous)
	}

	/** # If the current slide has a subsection, change to it */
	sub() {
		let subsection = this.has_sub()
		if (subsection)
			this.change_slide(subsection)
	}

	/** # If the current slide has a supersection, change to it */
	super() {
		let supersection = this.has_super()
		if (supersection)
			this.change_slide(supersection)
	}
}

export { Navigation }
