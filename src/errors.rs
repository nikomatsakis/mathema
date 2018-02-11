use failure::{Fail, Error};

#[derive(Fail, Debug)]
#[fail(display = "line {} contains an unrecognized line kind `{}`", source_line, kind)]
pub(crate) struct UnrecognizedLineKind {
    pub(crate) source_line: u64,
    pub(crate) kind: String
}

#[derive(Fail, Debug)]
#[fail(display = "Error accessing `{}`: {}", file, error)]
pub(crate) struct ErrorAccessingFile {
    pub(crate) file: String,
    /*#[cause]*/ pub(crate) error: Error,
}

