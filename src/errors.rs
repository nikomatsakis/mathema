use failure::{Context, Fail};
use std::fmt;

pub(crate) type Fallible<T> = Result<T, MathemaError>;

#[derive(Debug, Fail)]
pub(crate) struct MathemaError {
    error: Context<MathemaErrorKind>,
}

#[derive(Fail, Debug)]
pub(crate) enum MathemaErrorKind {
    #[fail(display = "line {} contains an unrecognized line kind `{}`", source_line, kind)]
    UnrecognizedLineKind {
        source_line: u64,
        kind: String,
    },

    #[fail(display = "Error accessing `{}`", file)] AccessingFile {
        file: String,
    },

    #[fail(display = "Failed to create directory `{}`", directory_path)]
    CreatingDir {
        directory_path: String,
    },

    #[fail(display = "Cannot load Mathema database from `{}`", database_path)]
    CannotLoadDatabase {
        database_path: String,
    },

    #[fail(display = "No git repository found in `{}`", directory_path)]
    NoGitRepositoryFound {
        directory_path: String,
    },

    #[fail(display = "Unexpected error encountered")]
    Unexpected,
}

impl From<Context<MathemaErrorKind>> for MathemaError {
    fn from(error: Context<MathemaErrorKind>) -> MathemaError {
        MathemaError { error }
    }
}

impl From<MathemaErrorKind> for MathemaError {
    fn from(error: MathemaErrorKind) -> MathemaError {
        MathemaError {
            error: Context::new(error),
        }
    }
}

impl fmt::Display for MathemaError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.error)?;
        match self.error.get_context() {
            MathemaErrorKind::Unexpected => {
                if let Some(cause) = self.error.cause() {
                    write!(fmt, ": {:?}", cause)?;
                }
            }
            _ => {
                if let Some(cause) = self.error.cause() {
                    write!(fmt, ": {}", cause)?;
                }
            }
        }
        Ok(())
    }
}

macro_rules! link_unexpected {
    ($($t:ty),* $(,)*) => {
        $(
            impl From<$t> for MathemaError {
                fn from(value: $t) -> MathemaError {
                    MathemaError::from(value.context(MathemaErrorKind::Unexpected))
                }
            }
        )*
    }
}

link_unexpected! {
    ::std::io::Error,
    ::walkdir::Error,
    ::git2::Error,
    ::serde_json::Error,
}
