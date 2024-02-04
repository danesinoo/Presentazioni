import { Decoration } from './lib/decoration.js'

class Decorations {
	constructor(presentazione) {
		this.decorations = [
			new Decoration("top", () => presentazione.has_previous()),
			new Decoration("bottom", () => presentazione.has_next()),
			new Decoration("right", () => presentazione.has_sub()),
			new Decoration("left", () => presentazione.has_super()),
		]
	}

	toggle_all() {
		this.decorations.forEach(decoration => decoration.toggle_visibility())
	}
}

export { Decorations }
