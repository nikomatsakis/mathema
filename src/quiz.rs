use crate::prelude::*;

crate fn quiz(directory: &Path, force: bool) -> Fallible<()> {
    let repo = &mut MathemaRepository::open(directory)?;
    let status = repo.load_cards()?;
    if status.warn_if_needed(force) {
        return Ok(());
    }

    Ok(())
}
