//! New subcommand.

use crate::prelude::*;

crate fn add(file: String) -> Fallible<()> {
    let cards = cards::parse_cards_file(&file)
        .with_context(|_| MathemaErrorKind::AccessingFile { file })?;
    println!("cards = {:#?}", cards);
    Ok(())
}
