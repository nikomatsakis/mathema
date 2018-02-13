//! New subcommand.

use crate::prelude::*;

crate fn new(directory: String) -> Fallible<()> {
    match new_atomic(&directory) {
        Ok(()) => Ok(()),
        Err(e) => {
            let _ = fs::remove_dir_all(&directory); // if this fails, oh well
            Err(e)
        }
    }
}

fn new_atomic(directory: impl AsRef<Path>) -> Fallible<()> {
    let _ = MathemaRepository::create_on_disk(&directory)?;

    Ok(())
}
