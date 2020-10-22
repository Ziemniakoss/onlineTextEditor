const editor = ace.edit("editor");
editor.setTheme("ace/theme/dracula");
editor.session.setMode("ace/mode/html");
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

function init() {
    document.getElementById("theme-selector").addEventListener("change", (event) => {
        const newTheme = event.target.value
        editor.setTheme(`ace/theme/${newTheme}`);
        localStorage.setItem("theme", newTheme);
    })
    selectInitialTheme();
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

function selectInitialTheme() {
    let theme = localStorage.getItem("theme");
    if (theme == null) {
        theme = "dracula";
        localStorage.setItem("theme", theme);
    }
    editor.setTheme(`ace/theme/${theme}`);
    document.getElementById("theme-selector").value = theme;
}

init();