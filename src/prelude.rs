#![allow(unused_imports)] // FIXME too annoying right now

crate use crate::{
    cards::{self, Card, CardLine, LineKind, Language},
    db::{Database, User, CardRecord, QuestionRecord, QuestionResult, QuestionKind},
    errors::{Fallible, MathemaError, MathemaErrorKind},
    git::MathemaRepository,
    status::Status,
    line_parser::LineParser,
    throw,
    uuid_ext::UuidExt,
};

crate use extern::{
    atomicwrites::{self, AtomicFile, OverwriteBehavior},
    chrono::{DateTime, Duration, Utc, prelude::*},
    failure::{self, bail, Error, Fail, ResultExt},
    git2,
    itertools::Itertools,
    serde_derive::{Serialize, Deserialize},
    std::collections::{BTreeSet, BTreeMap, HashMap},
    std::env,
    std::mem,
    std::io,
    std::fs::{self, File},
    std::path::{self, Path, PathBuf},
    std::u64,
    uuid::Uuid,
    walkdir::WalkDir,
};

crate type UtcDateTime = DateTime<Utc>;
