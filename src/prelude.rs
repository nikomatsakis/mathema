#![allow(unused_imports)] // FIXME too annoying right now

crate use crate::cards::{Card, CardLine, LineKind, Language};
crate use crate::db::{Database, User, CardRecord, QuestionRecord, QuestionKind, QuestionResult};
crate use crate::errors;
crate use crate::line_parser::LineParser;
crate use chrono::{DateTime, Utc, prelude::*};
crate use failure::{bail, Error};
crate use git2::Repository;
crate use std::collections::HashMap;
crate use std::fs::{self, File};
crate use std::path::{self, Path, PathBuf};
crate use uuid::Uuid;

#[allow(unused_imports)] // FIXME rustc bug
crate use failure::{ResultExt};

#[allow(unused_imports)] // FIXME rustc bug
crate use crate::uuid_ext::UuidExt;

crate type UtcDateTime = DateTime<Utc>;
