import EditorController from "./editorController.js";
import {Project} from "./projectRepository.js";
import {File} from "./filesRepository.js";


export default class EditorView {
	/**
	 * Controller for this view
	 *
	 * @type {EditorController}
	 */
	controller;

	editor;

	constructor() {
		this.controller = new EditorController(this);
		this.editor = ace.edit("editor");
		this.init()
	}

	init() {
		document.getElementById("theme-selector").addEventListener("change", (event) => {
			const newTheme = event.target.value
			this.editor.setTheme(`ace/theme/${newTheme}`);
			localStorage.setItem("theme", newTheme);
		})
		this.selectInitialTheme();
		this.editor.session.setMode("ace/mode/text");

		this.editor.session.on("change", (e) => {
			console.log("a")
			t.insert({row: 0, column: 0}, ";;;");
		})

		document.getElementById("new-file-button").addEventListener("click", (_) => {
			const newName = document.getElementById("new-file-name").value;
			this.controller.createNewFile(newName);
		})
	}

	selectInitialTheme() {
		let theme = localStorage.getItem("theme");
		if (theme == null) {
			theme = "dracula";
			localStorage.setItem("theme", theme);
		}
		this.editor.setTheme(`ace/theme/${theme}`);
		document.getElementById("theme-selector").value = theme;
	}

	/**
	 * Shows error as alert and prints it to console.
	 * Later wh can change it to show notification
	 *
	 * @param message {string}
	 */
	showError(message) {
		console.error(message);
		alert(message);
	}


	/**
	 * Show session list
	 *
	 * @param {IterableIterator} sessions
	 * @param {string} sessions[].id id of session
	 * @param {string} sessions[].name name of user in this session
	 */
	showSessions(sessions) {
		let sessionListElement = document.getElementById("users-list");
		while(sessionListElement.firstChild) {
			sessionListElement.removeChild(sessionListElement.firstChild);
		}
		for (const session of sessions) {
			console.log(JSON.stringify(session))
			const sessionDomElement = document.createElement("li");
			sessionDomElement.textContent = session.name;
			sessionListElement.appendChild(sessionDomElement);
		}
	}

	/**
	 * Displays files in left panel
	 *
	 * @param files {File []}
	 */
	showFilesList(files) {
		const listElement = document.getElementById("project-files-list");
		while (listElement.firstChild) {
			listElement.removeChild(listElement.firstChild);
		}

		files.forEach(file => {
			/** @type {HTMLLIElement}*/
			const fileListElement = document.createElement("li");
			fileListElement.innerText = file.name;
			fileListElement.dataset["id"] = "" + file.id;
			fileListElement.onclick = this.handleFileClick
			listElement.appendChild(fileListElement);
		})
	}

	handleFileClick(event) {
		console.log(event.target.dataset.id)
	}

	/**
	 *
	 * @param project {Project}
	 */
	showProjectInfo(project) {
		document.getElementById("project-name").innerText = project.name
		document.getElementById("project-description").innerText = project.description

	}
}

const extensionToLanguageMap = {
	// "abap"
// "abc",
//     "actionscript",
	"ada": "ada",
	// "alda",
	// "apache_conf",
	"cls": "apex",
	// "applescript",
	// "aql",
	// "asciidoc",
	// "asl",
	// "assembly_x86",
	// "autohotkey",
	// "batchfile",
	// "c9search",
	"c": "c_cpp",
	"cpp": "c_cpp",
	// "cirru",
	// "clojure",
	// "cobol",
	// "coffee",
	// "coldfusion",
	// "crystal",
	"cs": "csharp",
	// "csound_document",
	// "csound_orchestra",
	// "csound_score",
	// "csp",
	"css": "css",
	// "curly",
	// "dart",
	// "diff",
	// "django",
	// "d",
	"DOCKERFILE": "dockerfile",
	// "dot",
	// "drools",
	// "edifact",
	// "eiffel",
	// "ejs",
	// "elixir",
	// "elm",
	// "erlang",
	// "forth",
	// "fortran",
	// "fsharp",
	// "fsl",
	// "ftl",
	// "gcode",
	// "gherkin",
	"gitignore": "gitignore",
	// "glsl",
	// "gobstones",
	// "golang",
	// "graphqlschema",
	"groovy": "groovy",
	"haml": "haml",
	// "handlebars",
	// "haskell_cabal",
	// "haskell",
	// "haxe",
	// "hjson",
	// "html_elixir",
	"html": "html",
	"htm": "html",
	// "html_ruby",
	// "ini",
	// "io",
	// "jack",
	// "jade",
	"java": "java",
	"js": "javascript",
	// "json5",
	// "jsoniq",
	"josn": "json",
	// "jsp",
	// "jssm",
	// "jsx",
	// "julia",
	"kt": "kotlin",
	"tex": "latex",
	// "less",
	// "liquid",
	// "lisp",
	// "livescript",
	// "logiql",
	// "logtalk",
	// "lsl",
	// "lua",
	// "luapage",
	// "lucene",
	"MAKEFILE": "makefile",
	"md": "markdown",
	// "mask",
	// "matlab",
	// "maze",
	// "mediawiki",
	// "mel",
	// "mixal",
	// "mushcode",
	// "mysql",
	// "nginx",
	// "nim",
	// "nix",
// "nsis",
// "nunjucks",
// "objectivec",
// "ocaml",
// "pascal",
// "perl6",
// "perl",
// "pgsql",
	"php": "php",
// "php_laravel_blade",
// "pig",
	"txt": "plain_text",
// "powershell",
// "praat",
// "prisma",
// "prolog",
	"properties": "properties",
// "protobuf",
// "puppet",
	"py": "python",
// "qml",
// "razor",
// "rdoc",
// "red",
// "redshift",
// "rhtml",
// "r",
// "rst",
// "ruby",
	"rs": "rust",
// "sass",
// "scad",
// "scala",
// "scheme",
// "scss",
	"sh": "sh",
// "sjs",
// "slim",
// "smarty",//
// "snippets",
// "soy_template",
// "space",
// "sparql",
	"sql": "sql",
// "sqlserver",
// "stylus",
// "svg",
// "swift",
// "tcl",
// "terraform",
// "tex",
// "textile",
// "text",
	"toml": "toml",
// "tsx",
// "turtle",
// "twig",
	"ts": "typescript",
// "vala",
// "vbscript",
// "velocity",
// "verilog",
// "vhdl",
// "visualforce",
// "wollok",
	"xml": "xml",
// "xquery",
	"yaml": "yaml",
// "zeek"
}

const specialFileTypesToMode = {
	"DOCKERFILE": "dockerfile"
}

function getModeForFileExtension(fileExtension) {
	if (!fileExtension) {
		return "text";
	}
	const specialFileMode = specialFileTypesToMode[fileExtension];
	if (specialFileMode) {
		return specialFileMode;
	}
	const mode = extensionToLanguageMap[fileExtension];
	return mode ? mode : "text";
}