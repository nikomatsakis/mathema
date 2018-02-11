// FIXME rust-lang/rust#47075

extern crate uuid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
crate struct Uuid {
    data: uuid::Uuid,
}

impl Uuid {
    pub fn fresh() -> Uuid {
        Uuid {
            data: uuid::Uuid::new_v4()
        }
    }
}
