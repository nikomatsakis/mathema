use crate::prelude::*;

crate fn dump(options: &MathemaOptions) -> Fallible<()> {
    let _repo = &mut MathemaRepository::open(options)?;

    Ok(())
}
