//! Status subcommand.

use crate::prelude::*;

crate fn status(directory: &Path) -> Fallible<()> {
    let repo = MathemaRepository::open(&directory)?;
    let mut needs_separator = false;

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
        needs_separator = true;
    }

    // Load the card files that are registered.
    let mut cards_with_missing_uuids = vec![];
    let mut valid_cards = 0;
    for card_file in &repo.database().card_files {
        let cards = repo.parse_card_file_from_repo(card_file)?;
        for card in cards {
            if card.uuid.is_none() {
                cards_with_missing_uuids.push(card);
            } else {
                valid_cards += 1;
            }
        }
    }

    if !cards_with_missing_uuids.is_empty() {
        if mem::replace(&mut needs_separator, true) {
            println!("");
        }
        println!("Files containing cards with missing UUIDs (try `mathema add`):");
        let mut map: BTreeMap<&Path, Vec<u64>> = BTreeMap::default();
        for card in &cards_with_missing_uuids {
            map.entry(&card.source_file).or_insert(vec![]).push(card.start_line);
        }
        for (filename, mut lines) in map {
            lines.sort();
            if lines.len() == 1 {
                println!("  {} (on line {})", filename.display(), lines[0]);
            } else if lines.len() == 2 {
                println!("  {} (on lines {} and {})", filename.display(), lines[0], lines[1]);
            } else {
                let (tail, prefix) = lines.split_last().unwrap();
                let lines_str: String = prefix.iter()
                                              .map(|line| format!("{}, ", line))
                                              .collect();
                println!("  {} (on lines {}, and {})", filename.display(), lines_str, tail);
            }
        }
    }

    if mem::replace(&mut needs_separator, true) {
        println!("");
    }
    if repo.database().card_files.is_empty() {
        println!("No card files added so far.");
    } else {
        println!(
            "{} valid cards found amongst {} files.",
            valid_cards,
            repo.database().card_files.len(),
        );
    }

    Ok(())
}
