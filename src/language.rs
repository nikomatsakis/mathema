use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub(crate) enum Language {
    English,
    Greek,
}

impl fmt::Display for Language {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Language::English => write!(fmt, "en"),
            Language::Greek => write!(fmt, "gr"),
        }
    }
}

impl FromStr for Language {
    type Err = MathemaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gr" => Ok(Language::Greek),
            "en" => Ok(Language::English),
            _ => Err(MathemaErrorKind::UnrecognizedLanguage { text: s.to_string() }.into()),
        }
    }
}
