document.addEventListener("DOMContentLoaded", function () {
	// Inline math:
	document.querySelectorAll(".math-inline").forEach(function (el) {
		katex.render(
			el.textContent, // the raw TeX inside
			el, // the element to render into
			{ displayMode: false }
		);
	});

	// Display math:
	document.querySelectorAll(".math-display").forEach(function (el) {
		katex.render(el.textContent, el, { displayMode: true });
	});
});
