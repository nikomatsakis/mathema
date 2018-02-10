use ::uuid::

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
crate struct Uuid {
    data: String
}

impl Uuid {
    pub fn fresh() -> Uuid {
        Uuid {
            data: format!("{}", ::uuid::Uuid::new_v4())
        }
    }
}
