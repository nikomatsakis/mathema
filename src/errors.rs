use crate::prelude::*;
use failure::Fail;

pub(crate) type Fallible<T> = Result<T, MathemaError>;

#[derive(Fail, Debug)]
pub(crate) enum MathemaError {
    #[fail(display = "line {} contains an unrecognized line kind `{}`", source_line, kind)]
    UnrecognizedLineKind { source_line: u64, kind: String },

    #[fail(display = "Error accessing `{}`: {}", file, error)]
    AccessingFile {
        file: String,
        /*#[cause]*/ error: Error,
    },

    #[fail(display = "Failed to create directory `{}`: {}", directory_path, error)]
    CreatingDir {
        directory_path: String,
        /*#[cause]*/ error: Error,
    },

    #[fail(display = "No mathema database found in `{}`", directory_path)]
    NoDatabaseFileFound { directory_path: String },

    #[fail(display = "No git repository found in `{}`: {}", directory_path, error)]
    NoGitRepositoryFound {
        directory_path: String,
        error: Error,
    },

    #[fail(display = "Unexpected error encountered `{:?}`", error)]
    Unexpected { error: Error, backtrace: ::failure::Backtrace }
}

macro_rules! link_unexpected {
    ($($t:ty),* $(,)*) => {
        $(
            impl From<$t> for MathemaError {
                fn from(value: $t) -> MathemaError {
                    MathemaError::Unexpected {
                        error: Error::from(value),
                        backtrace: ::failure::Backtrace::new(),
                    }
                }
            }
        )*
    }
}

link_unexpected! {
    ::std::io::Error,
    ::git2::Error,
}
