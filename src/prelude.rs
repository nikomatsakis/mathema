#![allow(unused_imports)] // FIXME too annoying right now

crate use crate::{
    MathemaOptions,
    cards::{self, Card, CardLine, LineKind},
    db::{Database, User, CardRecord, QuestionRecord, QuestionResult, QuestionKind},
    errors::{Fallible, MathemaError, MathemaErrorKind},
    git::MathemaRepository,
    language::Language,
    quiz::presentation::{Presentation, Prompt},
    selection,
    status::Status,
    line_parser::LineParser,
    throw,
    uuid_ext::UuidExt,
};

crate use crate::{
    atomicwrites::{self, AtomicFile, OverwriteBehavior},
    chrono::{DateTime, Duration, Utc, prelude::*},
    cursive::Cursive,
    cursive::views::{Dialog, TextView},
    failure::{self, bail, Error, Fail, ResultExt},
    git2,
    itertools::Itertools,
    rand::{self, Rng},
    regex::Regex,
    serde_derive::{Serialize, Deserialize},
    std::collections::{BTreeSet, BTreeMap, HashMap},
    std::env,
    std::fmt,
    std::mem,
    std::io::{self, prelude::*},
    std::fs::{self, File},
    std::path::{self, Path, PathBuf},
    std::str::FromStr,
    std::u64,
    uuid::Uuid,
    walkdir::WalkDir,
};

crate type UtcDateTime = DateTime<Utc>;
