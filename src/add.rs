//! New subcommand.

use ::crate::cards;
use ::failure::Error;

crate fn add(file: String) -> Result<(), Error> {
    let cards = cards::parse_cards_file(&file)?;
    println!("cards = {:#?}", cards);
    Ok(())
}
