// FIXME rust-lang/rust#47075

use crate::prelude::*;

crate trait UuidExt {
    fn fresh() -> Self;
}

impl UuidExt for Uuid {
    fn fresh() -> Self {
        Uuid::new_v4()
    }
}
