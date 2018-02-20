//! Manages the git repository and files that stores our data.

use crate::prelude::*;

crate struct MathemaRepository {
    dry_run: bool,
    directory_path: PathBuf,
    repository: git2::Repository,
    database: Database,
    cards: HashMap<Uuid, Card>,
}

#[derive(Default)]
crate struct Status {
    crate unknown_card_files: Vec<PathBuf>,
    crate card_files_with_missing_uuids: BTreeMap<PathBuf, Vec<u64>>,
    crate duplicate_uuids: Vec<Uuid>,
    crate valid_cards: usize,
    crate valid_card_files: usize,
}

const RELATIVE_DB_PATH: &str = ".mathema-v1.json";

impl MathemaRepository {
    crate fn create_on_disk(directory: impl AsRef<Path>) -> Fallible<MathemaRepository> {
        let directory_path = directory.as_ref().to_owned();

        fs::create_dir(&directory_path).with_context(|_| MathemaErrorKind::CreatingDir {
            directory_path: directory_path.display().to_string(),
        })?;

        let repository = git2::Repository::init(&directory_path)?;

        let database = Database::empty();

        let mut repository = MathemaRepository {
            directory_path,
            repository,
            database,
            dry_run: false,
            cards: HashMap::new(),
        };
        repository.write_database()?;

        Ok(repository)
    }

    crate fn open(options: &MathemaOptions) -> Fallible<MathemaRepository> {
        Self::open_full(options.dry_run, options.directory()?)
    }

    crate fn open_full(dry_run: bool, directory: impl AsRef<Path>) -> Fallible<MathemaRepository> {
        let directory_path = directory.as_ref().to_owned();
        let db_path = directory_path.join(RELATIVE_DB_PATH);
        let database = Self::read_from(&db_path, |f| Database::load_from(f)).with_context(|_| {
            MathemaErrorKind::CannotLoadDatabase {
                database_path: db_path.display().to_string(),
            }
        })?;

        let repository = git2::Repository::open(&directory_path).with_context(|_| {
            MathemaErrorKind::NoGitRepositoryFound {
                directory_path: directory_path.display().to_string(),
            }
        })?;

        Ok(MathemaRepository {
            directory_path,
            repository,
            database,
            dry_run,
            cards: HashMap::new(),
        })
    }

    crate fn database(&self) -> &Database {
        &self.database
    }

    crate fn database_mut(&mut self) -> &mut Database {
        &mut self.database
    }

    crate fn cards(&self) -> &HashMap<Uuid, Card> {
        &self.cards
    }

    crate fn card_uuids(&self) -> impl Iterator<Item = Uuid> + '_ {
        self.cards.keys().cloned()
    }

    /// Makes a "database-relative" path into an absolute path.
    fn absolute_path(&self, relative_path: impl AsRef<Path>) -> PathBuf {
        self.directory_path.join(relative_path)
    }

    /// Open a relative path found in the repo.
    fn open_file(&self, relative_path: impl AsRef<Path>) -> Fallible<File> {
        let absolute_path = self.absolute_path(relative_path);
        Ok(
            File::open(&absolute_path).with_context(|_| MathemaErrorKind::AccessingFile {
                file: absolute_path.display().to_string(),
            })?,
        )
    }

    /// Returns a path relative to the database directory. Returns an
    /// error if `path` is not within database directory (or if it is,
    /// e.g., a symlink to something outside the directory).
    pub fn path_in_repo(&self, path: impl AsRef<Path>) -> Fallible<PathBuf> {
        let original_path = path.as_ref();
        let absolute_path = original_path.canonicalize()?;
        match absolute_path.strip_prefix(&self.directory_path) {
            Ok(p) => Ok(p.to_owned()),
            Err(_) => {
                throw!(MathemaErrorKind::NotInRepo {
                    file: original_path.display().to_string(),
                });
            }
        }
    }

    fn db_path(&self) -> PathBuf {
        self.absolute_path(RELATIVE_DB_PATH)
    }

    fn signature() -> Fallible<git2::Signature<'static>> {
        Ok(git2::Signature::now("mathema", "mathema@example.com")?)
    }

    fn write_file(
        &self,
        file_name: &Path,
        op: impl FnMut(&mut File) -> Fallible<()>,
    ) -> Fallible<()> {
        assert!(!self.dry_run);
        AtomicFile::new(file_name, OverwriteBehavior::AllowOverwrite).write(op)?;
        Ok(())
    }

    crate fn read_from<R>(
        file_name: &Path,
        mut op: impl FnMut(&mut File) -> Fallible<R>,
    ) -> Fallible<R> {
        let mut file = File::open(file_name)?;
        let r = op(&mut file)?;
        Ok(r)
    }

    crate fn write_database(&mut self) -> Fallible<()> {
        if !self.dry_run {
            let db_path = self.db_path();
            self.write_file(&self.db_path(), |f| self.database.write_to(f)).with_context(|_| {
                MathemaErrorKind::AccessingFile {
                    file: db_path.display().to_string(),
                }
            })?;

            self.git_commit()?;
        }

        Ok(())
    }

    /// Creates a new git commit, adding in the changes from all of
    /// the registered `cards` files as well as the index.
    fn git_commit(&mut self) -> Fallible<()> {
        assert!(!self.dry_run);
        let mut index = self.repository.index()?;
        index.add_path(Path::new(RELATIVE_DB_PATH))?;
        for card_file in &self.database.card_files {
            index.add_path(card_file)?;
        }
        index.write()?;

        let tree_id = self.repository.index()?.write_tree()?;
        let tree = self.repository.find_tree(tree_id)?;
        let signature = Self::signature()?;
        let parents = match self.repository.head() {
            Ok(head_ref) => {
                let head_oid = head_ref.target().unwrap();
                Some(self.repository.find_commit(head_oid)?)
            }
            _ => None,
        };
        let parents: Vec<_> = parents.iter().collect();
        self.repository.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "mathema: write_database",
            &tree,
            &parents,
        )?;
        self.repository.checkout_head(None)?;

        Ok(())
    }

    /// Returns database-relative paths to all the card files.
    crate fn all_card_files(&self) -> Fallible<Vec<PathBuf>> {
        // Ergonomic hits:
        // - no `Path::has_extension`?
        // - working with errors in an iterator is awful
        // - WalkDir has no "filter files"?
        //
        // as a result, switched to for loop :(

        let mut results = vec![];

        for entry in WalkDir::new(&self.directory_path) {
            let entry = entry?;

            if !entry.file_type().is_file() {
                continue;
            }

            if entry
                .path()
                .extension()
                .map(|e| e != "cards")
                .unwrap_or(true)
            {
                continue;
            }

            // My assumption here is that walkdir will always yield up
            // paths relative to its starting point. I wonder if
            // that's true. =)

            match entry.path().strip_prefix(&self.directory_path) {
                Ok(p) => results.push(p.to_owned()),
                Err(_) => panic!(
                    "expected `{:?}` to have a prefix of `{:?}`",
                    entry.path(),
                    self.directory_path,
                ),
            }
        }

        Ok(results)
    }

    crate fn parse_card_file_from_repo(&self, relative_path: &Path) -> Fallible<Vec<Card>> {
        let file = self.open_file(relative_path)?;
        Ok(cards::parse_cards_file_from(relative_path, file)?)
    }

    crate fn load_cards(&mut self) -> Fallible<Status> {
        let mut status = Status::default();

        // Compare all the card files that are registered
        // with those that exist in the directory.
        let mut all_card_files: BTreeSet<PathBuf> = self.all_card_files()?.into_iter().collect();
        for card_file in &self.database.card_files {
            all_card_files.remove(card_file);
        }
        status.unknown_card_files.extend(all_card_files);

        // Load the card files that are registered.
        let mut cards_with_missing_uuids = vec![];
        for card_file in &self.database.card_files {
            status.valid_card_files += 1;

            let cards = self.parse_card_file_from_repo(card_file)?;
            for card in cards {
                if let Some(uuid) = card.uuid {
                    if !self.cards.contains_key(&uuid) {
                        self.cards.insert(uuid, card);
                        status.valid_cards += 1;
                    } else {
                        status.duplicate_uuids.push(uuid);
                    }
                } else {
                    cards_with_missing_uuids.push(card);
                }
            }
        }

        Ok(status)
    }
}

impl Status {
    crate fn contains_warnings(&self) -> bool {
        !self.unknown_card_files.is_empty() || self.contains_fatal()
    }

    crate fn contains_fatal(&self) -> bool {
        !self.card_files_with_missing_uuids.is_empty() ||
            !self.duplicate_uuids.is_empty()
    }

    /// Issues warnings. Returns true if fatal warnings were emitted,
    /// and hence execution should not continue.
    crate fn warn_if_needed(&self, force: bool) -> bool {
        if !self.contains_warnings() {
            return false;
        }

        self.warn();

        // Cannot continue with missing or duplicate UUIDs.
        if self.contains_fatal() {
            return true;
        }

        // But otherwise, if the user said to "force", we can keep
        // going.
        if force {
            return false;
        }

        true
    }

    fn warn(&self) {
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

        if !self.duplicate_uuids.is_empty() {
            if mem::replace(&mut needs_separator, true) {
                println!("");
            }
            println!("Duplicate UUIDs found (try `grep` to find them):");
            for uuid in &self.duplicate_uuids {
                println!("  {}", uuid);
            }
        }
    }
}
