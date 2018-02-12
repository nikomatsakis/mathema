use crate::prelude::*;

crate struct MathemaRepository {
    directory_path: PathBuf,
    repository: git2::Repository,
}

const RELATIVE_DB_PATH: &str = ".mathema-v1.json";

impl MathemaRepository {
    crate fn create_on_disk(directory: impl AsRef<Path>) -> Fallible<MathemaRepository> {
        let directory_path = directory.as_ref().to_owned();

        fs::create_dir(&directory_path).map_err(|e| MathemaError::CreatingDir {
            directory_path: directory_path.display().to_string(),
            error: Error::from(e),
        })?;

        let repository = git2::Repository::init(&directory_path)?;

        Ok(MathemaRepository { directory_path, repository })
    }

    crate fn open(&self, directory: impl AsRef<Path>) -> Fallible<MathemaRepository> {
        let directory_path = directory.as_ref().to_owned();
        let db_path = directory_path.join(RELATIVE_DB_PATH);
        if !db_path.exists() {
            return Err(MathemaError::NoDatabaseFileFound {
                directory_path: directory_path.display().to_string(),
            });
        }

        let repository = git2::Repository::open(&directory_path).map_err(|e| {
            MathemaError::NoGitRepositoryFound {
                directory_path: directory_path.display().to_string(),
                error: Error::from(e),
            }
        })?;

        Ok(MathemaRepository { directory_path, repository })
    }

    fn db_path(&self) -> PathBuf {
        self.directory_path.join(RELATIVE_DB_PATH)
    }

    fn signature() -> Fallible<git2::Signature<'static>> {
        Ok(git2::Signature::now("mathema", "mathema@example.com")?)
    }

    crate fn write_database(&mut self, db: &Database) -> Fallible<()> {
        let db_path = self.db_path();
        db.write_to_path(&db_path).map_err(|e| MathemaError::AccessingFile {
            file: db_path.display().to_string(),
            error: Error::from(e),
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
}
