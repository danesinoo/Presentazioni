import { Navigation } from './navigation.js'
import { ContentManager } from './content_manager.js'

let show_comment = false

function set_comment_visibility() {
	let comment = document.getElementById('comment')
	if (show_comment) {
		comment.style.display = 'flex'
		comment.style.flex = '1 1 3fr'
	} else {
		comment.style.display = 'none'
	}
}

function toggle_comment(title, content) {
	show_comment = !show_comment
	set_comment_visibility()
	title.notify()
	content.notify()
}

document.addEventListener('DOMContentLoaded', function() {
	const presentation = new Navigation()
	const title = new ContentManager('title')
	const content = new ContentManager('content')

	presentation.add_observer(title)
	presentation.add_observer(content)

	window.addEventListener('resize', () => {
		title.notify()
		content.notify()
	})

	document.addEventListener('keydown', function(event) {
		if (event.key === 'j') {
			presentation.next()
			set_comment_visibility()
		}

		if (event.key === 'k') {
			presentation.previous()
			set_comment_visibility()
		}

		if (event.key === 'h') {
			presentation.supersection()
			set_comment_visibility()
		}

		if (event.key === 'l') {
			presentation.subsection()
			set_comment_visibility()

		}

		if (event.key === 'c') {
			toggle_comment(title, content)
		}
	})

	document.getElementById('bottom').addEventListener('click', function() {
		presentation.next()
		set_comment_visibility()
	})

	document.getElementById('top').addEventListener('click', function() {
		presentation.previous()
		set_comment_visibility()
	})

	document.getElementById('right').addEventListener('click', function() {
		presentation.subsection()
		set_comment_visibility()
	})

	document.getElementById('left').addEventListener('click', function() {
		presentation.supersection()
		set_comment_visibility()
	})
})
