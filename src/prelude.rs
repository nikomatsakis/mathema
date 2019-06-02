#![allow(unused_imports)] // FIXME too annoying right now

crate use crate::{
    cards::{self, Card, CardLine, LineKind},
    db::{CardRecord, Database, QuestionKind, QuestionRecord, QuestionResult, User},
    errors::{Fallible, MathemaError, MathemaErrorKind},
    git::MathemaRepository,
    language::Language,
    line_parser::LineParser,
    quiz::presentation::text::{TextDelegate, TextPresentation},
    quiz::presentation::{Presentation, PresentationMode, Prompt},
    quiz::SUITABLE_QUESTIONS,
    selection,
    status::Status,
    throw,
    uuid_ext::UuidExt,
    MathemaOptions,
};

crate use atomicwrites::{self, AtomicFile, OverwriteBehavior};
crate use chrono::{prelude::*, DateTime, Duration, Utc};
#[cfg(feature = "console")]
crate use cursive::views::{Dialog, TextView};
#[cfg(feature = "console")]
crate use cursive::Cursive;
crate use failure::{self, bail, Error, Fail, ResultExt};
crate use git2;
crate use itertools::Itertools;
crate use rand::{self, Rng, ThreadRng};
crate use regex::Regex;
crate use serde_derive::{Deserialize, Serialize};
crate use std::char;
crate use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
crate use std::env;
crate use std::fmt;
crate use std::fs::{self, File};
crate use std::io::{self, prelude::*};
crate use std::mem;
crate use std::path::{self, Path, PathBuf};
crate use std::str::FromStr;
crate use std::u64;
crate use uuid::Uuid;
crate use walkdir::WalkDir;

crate type UtcDateTime = DateTime<Utc>;
