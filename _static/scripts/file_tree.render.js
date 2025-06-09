document.addEventListener("DOMContentLoaded", function () {
	fetch("/_static/_file_tree.json")
		.then((response) => {
			if (!response.ok) {
				throw new Error("Network response was not ok");
			}
			return response.json();
		})
		.then((data) => {
			render_file_tree_data(data, "oj-file-explorer-list");
		})
		.catch((error) => {
			console.error("There was a problem with the fetch operation:", error);
		});
});

render_file_tree_data = function (input_data, parent_id) {
	let parent_element = document.getElementById(parent_id);
	for (let i = 0; i < input_data.length; i++) {
		if (input_data[i].hasOwnProperty("File")) {
			let curr_file = input_data[i]["File"];
			parent_element.innerHTML += `
				<li>
					<a href="${curr_file.href}">
						${curr_file.name}
					</a>
				</li>
			`;
		}
		if (input_data[i].hasOwnProperty("Folder")) {
			let curr_folder = input_data[i]["Folder"];
			let curr_folder_id = crypto.randomUUID();
			parent_element.innerHTML += `
			<li>
				<details>
					<summary><a href="${curr_folder.href}">${curr_folder.name}</a></summary>
					<ul id="${curr_folder_id}">
					</ul>
				</details>
			</li>
			`;
			render_file_tree_data(curr_folder.children, curr_folder_id);
		}
	}
};
