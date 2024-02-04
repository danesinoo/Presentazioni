class Decoration {
	constructor(id, toggle_fn) {
		this.id = id
		this.toggle_fn = toggle_fn
	}

	this_content() {
		return document.getElementById(this.id)
	}

	toggle_visibility() {
		if (this.toggle_fn()) {
			this.this_content().style.display = "block"
		} else {
			this.this_content().style.display = "none"
		}
	}
}

export { Decoration }
