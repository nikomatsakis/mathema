//! New subcommand.

use crate::prelude::*;

crate fn new(directory: String) -> Result<(), Error> {
    fs::create_dir(&directory).with_context(|_| {
        format!(
            "failed to create directory `{}`, maybe it already exists?",
            directory
        )
    })?;

    Repository::init(&directory).with_context(|_| {
        format!(
            "failed to initialize git repo in `{}`",
            directory
        )
    })?;

    Ok(())
}
