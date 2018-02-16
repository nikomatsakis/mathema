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
    let repo = &MathemaRepository::open(&directory)?;
    let status = Status::from(repo)?;
    status.print();
    Ok(())
}

impl Status {
    crate fn from(repo: &MathemaRepository) -> Fallible<Self> {
        let mut result = Status::default();

        // Compare all the card files that are registered
        // with those that exist in the directory.
        let mut all_card_files: BTreeSet<PathBuf> = repo.all_card_files()?.into_iter().collect();
        for card_file in &repo.database().card_files {
            all_card_files.remove(card_file);
        }
        result.unknown_card_files.extend(all_card_files);

        // Load the card files that are registered.
        let mut cards_with_missing_uuids = vec![];
        for card_file in &repo.database().card_files {
            result.valid_card_files += 1;

            let cards = repo.parse_card_file_from_repo(card_file)?;
            for card in cards {
                if card.uuid.is_none() {
                    cards_with_missing_uuids.push(card);
                } else {
                    result.valid_cards += 1;
                }
            }
        }

        if !cards_with_missing_uuids.is_empty() {
            let mut map: BTreeMap<PathBuf, Vec<u64>> = BTreeMap::default();
            for card in cards_with_missing_uuids {
                map.entry(card.source_file).or_insert(vec![]).push(card.start_line);
            }
            for (_, lines) in &mut map {
                lines.sort();
            }
            result.card_files_with_missing_uuids.extend(map);
        }

        Ok(result)
    }

    fn print(&self) {
        let mut needs_separator = false;

        if !self.unknown_card_files.is_empty() {
            println!("Unknown card files (try `mathema add`):");
            for unregistered_card_file in &self.unknown_card_files {
                println!("  {}", unregistered_card_file.display());
            }
            needs_separator = true;
        }

        if !self.card_files_with_missing_uuids.is_empty() {
            if mem::replace(&mut needs_separator, true) {
                println!("");
            }
            println!("Files containing cards with missing UUIDs (try `mathema add`):");
            for (filename, lines) in &self.card_files_with_missing_uuids {
                if lines.len() == 1 {
                    println!("  {} (on line {})", filename.display(), lines[0]);
                } else if lines.len() == 2 {
                    println!(
                        "  {} (on lines {} and {})",
                        filename.display(),
                        lines[0],
                        lines[1]
                    );
                } else {
                    let (tail, prefix) = lines.split_last().unwrap();
                    let lines_str: String = prefix.iter().map(|line| format!("{}, ", line)).collect();
                    println!(
                        "  {} (on lines {}, and {})",
                        filename.display(),
                        lines_str,
                        tail
                    );
                }
            }
        }

        if mem::replace(&mut needs_separator, true) {
            println!("");
        }
        if self.valid_card_files == 0 {
            println!("No card files added so far.");
        } else {
            println!(
                "{} valid cards found amongst {} files.",
                self.valid_cards,
                self.valid_card_files,
            );
        }
    }
}
