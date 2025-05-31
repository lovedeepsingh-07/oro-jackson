window.render_mermaid = async function () {
	document.querySelectorAll(".language-mermaid").forEach((el) => {
		el.classList.remove("language-mermaid");
		el.classList.add("mermaid");
	});

	const mermaid = await import("https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs");
	mermaid.initialize({ startOnLoad: true });
};

window.render_mermaid();
window.addEventListener("pageshow", function () {
	window.render_mermaid();
});
