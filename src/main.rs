mod cli;
mod document;
mod editor;
mod syntax;
mod theme;
mod tui;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = cli::parse()?;
    let mut document = document::load(&args.file)?;

    if args.print {
        document::print_document(&document);
        return Ok(());
    }

    tui::run(&mut document, args.edit)
}
