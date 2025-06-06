window.handle_theme_control = function () {
	const html = document.documentElement;
	const checkbox = document.getElementById("oj-theme-toggle-checkbox");
	const storedTheme = localStorage.getItem("oro-jackson-theme");

	const defaultTheme = "oj-dark";

	// checked = oj-light
	// not checked = oj-dark
	if (storedTheme) {
		html.setAttribute("data-theme", storedTheme);
		if (storedTheme == "oj-dark" && checkbox) {
			checkbox.checked = false;
		} else if (storedTheme == "oj-light" && checkbox) {
			checkbox.checked = true;
		}
	} else {
		html.setAttribute("data-theme", defaultTheme);
		localStorage.setItem("oro-jackson-theme", defaultTheme);
		checkbox.checked = false;
	}

	if (checkbox) {
		checkbox.addEventListener("change", function () {
			if (this.checked == true) {
				const newTheme = "oj-light";
				html.setAttribute("data-theme", newTheme);
				localStorage.setItem("oro-jackson-theme", newTheme);
			} else {
				const newTheme = "oj-dark";
				html.setAttribute("data-theme", newTheme);
				localStorage.setItem("oro-jackson-theme", newTheme);
			}
		});
	}
};

window.handle_theme_control();

// Fix for issue #2: Theme not updating correctly on back navigation
// See: https://github.com/lovedeepsingh-07/oro-jackson/issues/2
window.addEventListener("pageshow", function () {
	window.handle_theme_control();
});
