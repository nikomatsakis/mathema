use crate::prelude::*;

use ::failure::Context;
use ::std::fmt;

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

    #[fail(display = "`{}` is not a recognized language", text)]
    UnrecognizedLanguage {
        text: String,
    },

    #[fail(display = "`{}` is not a recognized presentation mode (try basic or ncurses)", text)]
    UnrecognizedPresentationMode {
        text: String,
    },

    #[fail(display = "the option `{}` is not compatible with the command `{}`", option, command)]
    IncompatibleOption {
        option: &'static str,
        command: &'static str,
    },

    #[fail(display = "don't know how to quiz you in `{}`", language)]
    DontKnowHowToQuiz {
        language: &'static str,
    },

    #[fail(display = "the cards file `{}` does not appear to be in the repository directory",
           file)]
    NotInRepo {
        file: String,
    },

    #[fail(display = "the card on line {} of `{}` already has a UUID assigned", line, file)]
    PreexistingUUID {
        file: String,
        line: u64,
    },

    #[fail(display = "the card on line {} of `{}` has an invalid UUID", line, file)]
    InvalidUUID {
        file: String,
        line: u64,
    },

    #[fail(display = "card on line {} of `{}` does not have a UUID; re-run `mathema add`?",
           source_file, source_line)]
    CardWithNoUuid {
        source_file: String,
        source_line: u64,
    },

    #[fail(display = "Error accessing `{}`", file)] AccessingFile {
        file: String,
    },

    #[fail(display = "Error loading card file")] ErrorLoadingCardFile,

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

    #[fail(display = "Unexpected error encountered")] Unexpected,
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
    ($($t:path),* $(,)*) => {
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
    ::ron::de::Error,
    ::ron::ser::Error,
}

impl<E> From<atomicwrites::Error<E>> for MathemaError
where
    E: Into<MathemaError>,
{
    fn from(value: atomicwrites::Error<E>) -> MathemaError {
        match value {
            atomicwrites::Error::Internal(e) => e.into(),
            atomicwrites::Error::User(e) => e.into(),
        }
    }
}
