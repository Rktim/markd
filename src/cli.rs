use std::env;
use std::error::Error;

pub struct CliArgs {
    pub file: String,
    pub print: bool,
    pub edit: bool,
}

pub fn parse() -> Result<CliArgs, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "-v" || arg == "--version") {
        println!("markd {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    if args.len() < 2 || args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        std::process::exit(0);
    }

    let print = args.iter().any(|arg| arg == "-p" || arg == "--print");
    let edit = args.iter().any(|arg| arg == "-e" || arg == "--edit");

    let file = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| "untitled.md".to_string());

    Ok(CliArgs { file, print, edit })
}

fn print_help() {
    println!("markd - Universal Terminal Document Viewer & Editor\n");

    println!("USAGE:");
    println!("  markd <file>          View document in terminal");
    println!("  markd -e <file>       Open document directly in built-in editor");
    println!("  markd -p <file>       Print converted/rendered content to stdout");
    println!("  markd -v, --version   Print version information\n");

    println!("EXAMPLES:");
    println!("  markd README.md");
    println!("  markd -e notes.md");
    println!("  markd index.html");
    println!("  markd style.css");
    println!("  markd report.pdf\n");

    println!("VIEWER CONTROLS:");
    println!("  e             Enter built-in interactive editor");
    println!("  t / m         Open theme menu");
    println!("  j / ↓         Scroll down");
    println!("  k / ↑         Scroll up");
    println!("  f / Space     Page down");
    println!("  b             Page up");
    println!("  g / Home      Top");
    println!("  G             Bottom");
    println!("  q / Esc       Quit\n");

    println!("EDITOR CONTROLS:");
    println!("  Ctrl+S        Save changes to disk");
    println!("  Ctrl+Z        Undo change");
    println!("  Ctrl+Y        Redo change");
    println!("  Ctrl+K        Delete line");
    println!("  Ctrl+D        Duplicate line");
    println!("  Tab           Indent (4 spaces)");
    println!("  Esc / Ctrl+Q  Return to viewer");
}
