#![allow(dead_code)]
#![deny(unused_must_use)] // always a bug
#![feature(crate_in_paths, conservative_impl_trait, crate_visibility_modifier, decl_macro,
           dyn_trait, /*FIXME(rust-lang/rust#47075) extern_absolute_paths,*/
           in_band_lifetimes, match_default_bindings, nll,
           termination_trait, underscore_lifetimes, universal_impl_trait)]

// FIXME can't use this because of format!
//#![deny(elided_lifetime_in_path)]

// FIXME rust-lang/rust#47075
#[cfg(test)]
extern crate assert_cli;
extern crate atomicwrites;
extern crate chrono;
extern crate failure;
extern crate git2;
extern crate itertools;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate structopt_derive;
extern crate structopt;
#[cfg(test)]
extern crate tempdir;
extern crate uuid;
extern crate walkdir;

use structopt_derive::StructOpt;
use structopt::StructOpt;
use failure::Error;
use std::process;

mod add;
mod cards;
mod db;
mod errors;
mod git;
mod line_parser;
mod new;
mod status;
mod prelude;
mod test;
mod uuid_ext;

/// Do fancy things
#[derive(StructOpt, Debug)]
#[structopt(name = "mathema", about = "a CLI for flashcards")]
enum Mathema {
    #[structopt(name = "quiz", about = "test yourself")] Quiz {},

    #[structopt(name = "dump", about = "dump info about cards")] Dump {},

    #[structopt(name = "new", about = "create a new deck of cards")]
    New {
        #[structopt(help = "where to create your cards")]
        directory: String,
    },

    #[structopt(name = "status", about = "create a new deck of cards")]
    Status {
        #[structopt(help = "directory containing your cards")]
        directory: Option<String>,
    },

    #[structopt(name = "add", about = "add new cards from file")]
    Add {
        #[structopt(help = "new card file")]
        file: String,
    },
}

fn main() {
    match main1() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

fn main1() -> Result<(), Error> {
    match Mathema::from_args() {
        Mathema::Quiz {} => {
            println!("Don't you feel smarter?");
        }

        Mathema::New { directory } => {
            new::new(directory)?;
        }

        Mathema::Status { directory } => {
            status::status(directory)?;
        }

        Mathema::Add { file } => {
            add::add(file)?;
        }

        Mathema::Dump {} => {
            println!("Dumping cards");
        }
    }
    Ok(())
}
