//! Status subcommand.

use crate::prelude::*;

crate fn status(directory: Option<String>) -> Fallible<()> {
    let directory = &match directory {
        Some(s) => Path::new(&s).to_owned(),
        None => env::current_dir()?,
    };

    let repo = MathemaRepository::open(&directory)?;

    // Compare all the card files that are registered
    // with those that exist in the directory.
    let mut all_card_files: BTreeSet<PathBuf> =
        repo.all_card_files()?
            .into_iter()
            .collect();
    for card_file in &repo.database().card_files {
        all_card_files.remove(card_file);
    }
    if !all_card_files.is_empty() {
        println!("Unknown card files (try `mathema add`):");
        for unregistered_card_file in all_card_files {
            println!("  {}", unregistered_card_file.display());
        }
    }

    Ok(())
}
