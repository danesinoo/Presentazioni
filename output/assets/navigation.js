import { Observable } from './lib/observable.js'

const HEADINGS = ['H1', 'H2', 'H3', 'H4', 'H5', 'H6']

class Navigation extends Observable {
	constructor() {
		super()

		this.slide = document.createElement('section')
		this.comment = document.createElement('section')
		this.change_slide(document.getElementsByTagName('div')[0])

	}

	copy_slide(new_slide) {
		this.slide.remove()
		this.slide = document.createElement('section')
		this.slide.id = 'slide'
		this.actual = new_slide

		let content = document.createElement('section')
		let title = document.createElement('header')

		for (let node of Array.from(new_slide.childNodes)) {
			if (node.tagName === 'DIV' || node.tagName === "SECTION") {
				continue
			} else if (HEADINGS.includes(node.tagName)) {
				title.appendChild(node.cloneNode(true))
			} else { content.appendChild(node.cloneNode(true)) }
		}

		title.id = 'title'
		content.id = 'content'

		this.slide.appendChild(title)
		this.slide.appendChild(content)
		document.body.appendChild(this.slide)
	}

	copy_comment(new_slide) {
		this.comment.remove()

		let comment = Array.from(new_slide.children)
			.find(node => node.tagName === "SECTION")

		if (!comment) {
			this.comment = document.createElement('section')
		} else {
			this.comment = comment.cloneNode(true)
			console.log(this.comment)
		}

		this.comment.id = 'comment'
		document.body.appendChild(this.comment)
	}

	change_slide(new_slide) {
		this.copy_slide(new_slide)
		this.copy_comment(new_slide)
		this.update_decorations_visibility()
		this.update()
	}

	has_next() {
		let siblings = Array.from(this.actual.parentElement.childNodes)
			.filter(node => node.tagName === 'DIV')
		let i = siblings.indexOf(this.actual)

		if (i >= siblings.length - 1)
			return null
		return siblings[i + 1]
	}

	has_previous() {
		let siblings = Array.from(this.actual.parentElement.childNodes)
			.filter(node => node.tagName === 'DIV')

		let i = siblings.indexOf(this.actual)
		if (i < 1)
			return null
		return siblings[i - 1]
	}

	has_subsection() {
		let subsection = Array.from(this.actual.childNodes).filter(node => node.tagName === 'DIV')
		if (subsection.length > 0)
			return subsection[0]
		return null
	}

	has_supersection() {
		if (this.actual.parentElement.tagName === 'DIV')
			return this.actual.parentElement
		return null
	}

	next() {
		let next = this.has_next()
		if (next)
			this.change_slide(next)
	}

	previous() {
		let previous = this.has_previous()
		if (previous)
			this.change_slide(previous)
	}

	subsection() {
		let subsection = this.has_subsection()
		if (subsection)
			this.change_slide(subsection)
	}

	supersection() {
		let supersection = this.has_supersection()
		if (supersection)
			this.change_slide(supersection)
	}

	update_decorations_visibility() {
		let next = this.has_next()
		let previous = this.has_previous()
		let subsection = this.has_subsection()
		let supersection = this.has_supersection()

		if (next) {
			document.getElementById('bottom').style.display = 'block'
		} else {
			document.getElementById('bottom').style.display = 'none'
		}

		if (previous) {
			document.getElementById('top').style.display = 'block'
		} else {
			document.getElementById('top').style.display = 'none'
		}

		if (subsection) {
			document.getElementById('right').style.display = 'block'
		}
		else {
			document.getElementById('right').style.display = 'none'
		}
		if (supersection) {
			document.getElementById('left').style.display = 'block'
		}
		else {
			document.getElementById('left').style.display = 'none'
		}
	}
}

export { Navigation }
