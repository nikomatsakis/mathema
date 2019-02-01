#![allow(unused_imports)] // FIXME too annoying right now

crate use crate::{
    MathemaOptions,
    cards::{self, Card, CardLine, LineKind},
    db::{Database, User, CardRecord, QuestionRecord, QuestionResult, QuestionKind},
    errors::{Fallible, MathemaError, MathemaErrorKind},
    git::MathemaRepository,
    language::Language,
    quiz::SUITABLE_QUESTIONS,
    quiz::presentation::{Presentation, PresentationMode, Prompt},
    quiz::presentation::text::{TextPresentation, TextDelegate},
    selection,
    status::Status,
    line_parser::LineParser,
    throw,
    uuid_ext::UuidExt,
};

crate use atomicwrites::{self, AtomicFile, OverwriteBehavior};
crate use chrono::{DateTime, Duration, Utc, prelude::*};
crate use cursive::Cursive;
crate use cursive::views::{Dialog, TextView};
crate use failure::{self, bail, Error, Fail, ResultExt};
crate use git2;
crate use itertools::Itertools;
crate use rand::{self, Rng, ThreadRng};
crate use regex::Regex;
crate use serde_derive::{Serialize, Deserialize};
crate use std::char;
crate use std::collections::{BTreeSet, BTreeMap, HashMap, HashSet};
crate use std::env;
crate use std::fmt;
crate use std::mem;
crate use std::io::{self, prelude::*};
crate use std::fs::{self, File};
crate use std::path::{self, Path, PathBuf};
crate use std::str::FromStr;
crate use std::u64;
crate use uuid::Uuid;
crate use walkdir::WalkDir;

crate type UtcDateTime = DateTime<Utc>;
