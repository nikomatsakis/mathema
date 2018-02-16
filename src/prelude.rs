#![allow(unused_imports)] // FIXME too annoying right now

crate use crate::throw;
crate use crate::cards::{self, Card, CardLine, LineKind, Language};
crate use crate::db::{Database, User, CardRecord, QuestionRecord, QuestionKind, QuestionResult};
crate use crate::errors::{Fallible, MathemaError, MathemaErrorKind};
crate use crate::git::MathemaRepository;
crate use crate::line_parser::LineParser;

crate use atomicwrites::{AtomicFile, OverwriteBehavior};
crate use chrono::{DateTime, Utc, prelude::*};
crate use failure::{bail, Error};
crate use git2;
crate use std::collections::{BTreeSet, BTreeMap, HashMap};
crate use std::env;
crate use std::mem;
crate use std::io;
crate use std::fs::{self, File};
crate use std::path::{self, Path, PathBuf};
crate use uuid::Uuid;

#[allow(unused_imports)] // FIXME rustc bug
crate use itertools::Itertools;

#[allow(unused_imports)] // FIXME rustc bug
crate use failure::{ResultExt};

#[allow(unused_imports)] // FIXME rustc bug
crate use crate::uuid_ext::UuidExt;

crate type UtcDateTime = DateTime<Utc>;
