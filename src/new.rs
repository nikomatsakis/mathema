//! New subcommand.

use ::failure::{Error, Fail};
use ::git2::Repository;
use ::std::fs;

crate fn new(directory: String) -> Result<(), Error> {
    fs::create_dir(&directory).map_err(|e| {
        e.context(format!(
            "failed to create directory `{}`, maybe it already exists?",
            directory
        ))
    })?;

    Repository::init(&directory).map_err(|e| {
        e.context(format!(
            "failed to initialize git repo in `{}`",
            directory
        ))
    })?;

    Ok(())
}
