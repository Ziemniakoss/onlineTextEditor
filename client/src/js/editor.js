import EditorView from "./editorView.js";


const view = new EditorView();

document.getElementById("test-replace-json").value =
	`{
	\"start\": {
		\"row\": 0,
		\"column\": 0
	},
	\"end\": {
		\"row\": 0,
		\"column\":1
	},
	\"lines\": [\"a\"]
}`;

document.getElementById("test-replace-butt").onclick = e =>{
	const change = JSON.parse(document.getElementById("test-replace-json").value)
	console.log(change);
	view.replaceText(new ace.Range(change.start.row, change.start.column, change.end.row,change.end.column), change.lines.join("\n"))
}


