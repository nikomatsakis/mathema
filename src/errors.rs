use failure::Fail;

#[derive(Fail, Debug)]
#[fail(display = "error reading `{}`", source_file)]
pub(crate) struct ErrorReading {
    pub(crate) source_file: String,
}

#[derive(Fail, Debug)]
#[fail(display = "{}:{}: unrecognized line kind `{}`", source_file, source_line, kind)]
pub(crate) struct UnrecognizedLineKind {
    pub(crate) source_file: String,
    pub(crate) source_line: u64,
    pub(crate) kind: String
}

