use std::{env, error::Error};

type DynError = Box<dyn Error>;

pub mod tasks {
    pub fn print_help() {
        println!(
            "
Usage: Run with `cargo xtask <task>`, eg. `cargo xtask xxx`.

    Tasks:
        docgen: Generate files to be included in the mdbook output.
        themelint <theme>: Report errors for <theme>, or all themes if no theme is specified.
        query-check: Check that tree-sitter queries are valid.
"
        );
    }
}

fn main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task {
        None => tasks::print_help(),
        Some(t) => match t.as_str() {
            _ => unreachable!(),
        },
    };
    Ok(())
}
