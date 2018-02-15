//! Manages the git repository and files that stores our data.

use crate::prelude::*;

crate struct MathemaRepository {
    directory_path: PathBuf,
    repository: git2::Repository,
    database: Database,
    cards: HashMap<Uuid, Card>,
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
            cards: HashMap::new(),
        };
        repository.write_database()?;

        Ok(repository)
    }

    crate fn open(directory: impl AsRef<Path>) -> Fallible<MathemaRepository> {
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
            cards: HashMap::new(),
        })
    }

    crate fn database(&self) -> &Database {
        &self.database
    }

    crate fn database_mut(&mut self) -> &mut Database {
        &mut self.database
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

    /// Returns a path relative to the database directory. Returns
    /// `None` if `absolute_path` is not within database directory.
    fn relative_path(&self, absolute_path: impl AsRef<Path>) -> Option<PathBuf> {
        let absolute_path = absolute_path.as_ref();
        assert!(absolute_path.is_absolute());
        match absolute_path.strip_prefix(&self.directory_path) {
            Ok(p) => Some(p.to_owned()),
            Err(_) => None,
        }
    }

    fn db_path(&self) -> PathBuf {
        self.absolute_path(RELATIVE_DB_PATH)
    }

    fn signature() -> Fallible<git2::Signature<'static>> {
        Ok(git2::Signature::now("mathema", "mathema@example.com")?)
    }

    crate fn write_file(
        file_name: &Path,
        op: impl FnMut(&mut File) -> Fallible<()>,
    ) -> Fallible<()> {
        match AtomicFile::new(file_name, OverwriteBehavior::AllowOverwrite).write(op) {
            Ok(()) => Ok(()),
            Err(::atomicwrites::Error::Internal(e)) => return Err(e.into()),
            Err(::atomicwrites::Error::User(e)) => return Err(e),
        }
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
        let db_path = self.db_path();
        Self::write_file(&self.db_path(), |f| self.database.write_to(f)).with_context(|_| {
            MathemaErrorKind::AccessingFile {
                file: db_path.display().to_string(),
            }
        })?;

        let mut index = self.repository.index()?;
        index.add_path(Path::new(RELATIVE_DB_PATH))?;
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

        for entry in ::walkdir::WalkDir::new(&self.directory_path) {
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
}
