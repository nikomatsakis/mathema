//! Status subcommand.

use crate::prelude::*;

#[derive(Default)]
crate struct Status {
    unknown_card_files: Vec<PathBuf>,
    card_files_with_missing_uuids: BTreeMap<PathBuf, Vec<u64>>,
    valid_cards: usize,
    valid_card_files: usize,
}

crate fn status(directory: &Path) -> Fallible<()> {
    let repo = &mut MathemaRepository::open(&directory)?;
    let status = repo.load_cards()?;

    if status.warn_if_needed(false) {
        println!("");
    }

    if status.valid_card_files == 0 {
        println!("No card files added so far.");
    } else {
        println!(
            "{} valid cards found amongst {} files.",
            status.valid_cards,
            status.valid_card_files,
        );
    }

    Ok(())
}

