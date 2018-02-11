//! New subcommand.

use crate::cards;
use crate::errors::ErrorAccessingFile;
use failure::Error;

crate fn add(file: String) -> Result<(), Error> {
    let cards = cards::parse_cards_file(&file).map_err(|error| ErrorAccessingFile { error, file })?;
    println!("cards = {:#?}", cards);
    Ok(())
}
