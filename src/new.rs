//! New subcommand.

use crate::prelude::*;

crate fn new(options: &MathemaOptions, new_directory: &str) -> Fallible<()> {
    let MathemaOptions {
        directory,
        force,
        dry_run,
        command: _,
    } = options;

    if *dry_run {
        throw!(MathemaErrorKind::IncompatibleOption {
            option: "--dry-run",
            command: "new",
        });
    }

    if directory.is_some() {
        throw!(MathemaErrorKind::IncompatibleOption {
            option: "--directory",
            command: "new",
        });
    }

    if *force {
        throw!(MathemaErrorKind::IncompatibleOption {
            option: "--force",
            command: "new",
        });
    }

    match new_atomic(new_directory) {
        Ok(()) => Ok(()),
        Err(e) => {
            let _ = fs::remove_dir_all(&new_directory); // if this fails, oh well
            Err(e)
        }
    }
}

fn new_atomic(directory: impl AsRef<Path>) -> Fallible<()> {
    let _ = MathemaRepository::create_on_disk(&directory)?;

    Ok(())
}
