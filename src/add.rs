//! New subcommand.

use crate::prelude::*;

crate fn add(file: String) -> Fallible<()> {
    let cards = cards::parse_cards_file(&file)
        .map_err(|error| MathemaError::AccessingFile { error: error.into(), file })?;
    println!("cards = {:#?}", cards);
    Ok(())
}
