mod cli;
mod document;
mod theme;
mod tui;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::parse()?;
    let document = document::load(&args.file)?;

    if args.print {
        document::print_document(&document);
        return Ok(());
    }

    tui::run(&document)
}
