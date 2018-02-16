#![allow(unused_imports)] // FIXME too annoying right now

crate use crate::{
    cards::{self, Card, CardLine, LineKind, Language},
    db::{Database, User, CardRecord, QuestionRecord, QuestionKind, QuestionResult},
    errors::{Fallible, MathemaError, MathemaErrorKind},
    git::MathemaRepository,
    line_parser::LineParser,
    throw,
    uuid_ext::UuidExt,
};

crate use extern::{
    atomicwrites::{self, AtomicFile, OverwriteBehavior},
    chrono::{DateTime, Utc, prelude::*},
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
    uuid::Uuid,
    walkdir::WalkDir,
};

crate type UtcDateTime = DateTime<Utc>;
