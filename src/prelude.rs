crate use crate::errors;
crate use crate::line_parser::LineParser;
crate use failure::{bail, Error};
crate use git2::Repository;
crate use std::fs::{self, File};
crate use uuid::Uuid;

#[allow(unused_imports)] // FIXME rustc bug
crate use failure::{ResultExt};

#[allow(unused_imports)] // FIXME rustc bug
crate use crate::uuid_ext::UuidExt;
