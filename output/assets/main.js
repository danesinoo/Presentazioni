import { Navigation } from './navigation.js'

document.addEventListener('DOMContentLoaded', function() {
	const presentation = new Navigation()

	window.addEventListener('resize', () => {
		presentation.update()
	})

	document.addEventListener('keydown', function(event) {
		if (event.key === 'j') {
			presentation.next()
		}

		if (event.key === 'k') {
			presentation.previous()
		}

		if (event.key === 'h') {
			presentation.super()
		}

		if (event.key === 'l') {
			presentation.sub()
		}

		if (event.key === 'c') {
			presentation.comment.change_visibility()
			presentation.update()
		}
	})

	document.getElementById('bottom').addEventListener('click', function() {
		presentation.next()
	})

	document.getElementById('top').addEventListener('click', function() {
		presentation.previous()
	})

	document.getElementById('right').addEventListener('click', function() {
		presentation.sub()
	})

	document.getElementById('left').addEventListener('click', function() {
		presentation.super()
	})
})
