use std::env;
use std::error::Error;

pub struct CliArgs {
    pub file: String,
    pub print: bool,
}

pub fn parse() -> Result<CliArgs, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        std::process::exit(0);
    }

    let print = args.iter().any(|arg| arg == "-p" || arg == "--print");

    let file = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with('-'))
        .ok_or("No input file specified.")?
        .clone();

    Ok(CliArgs { file, print })
}

fn print_help() {
    println!("markd - Universal Terminal Document Viewer\n");

    println!("USAGE:");
    println!("  markd <file>");
    println!("  markd -p <file>\n");

    println!("EXAMPLES:");
    println!("  markd README.md");
    println!("  markd report.pdf");
    println!("  markd document.docx");
    println!("  markd slides.pptx");
    println!("  markd data.xlsx\n");

    println!("TUI:");
    println!("  Hover ⚙       Open theme menu");
    println!("  Click theme   Apply theme");
    println!("  t / m         Open theme menu");
    println!("  j / ↓         Scroll down");
    println!("  k / ↑         Scroll up");
    println!("  f / Space     Page down");
    println!("  b             Page up");
    println!("  g / Home      Top");
    println!("  q / Esc       Quit");
}
