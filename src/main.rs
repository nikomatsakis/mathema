#![allow(dead_code)]
#![deny(unused_must_use)] // always a bug
#![feature(crate_in_paths, conservative_impl_trait, crate_visibility_modifier, decl_macro,
           dyn_trait, /*FIXME(rust-lang/rust#47075) extern_absolute_paths,*/ extern_in_paths,
           in_band_lifetimes, match_default_bindings, nll,
           termination_trait, underscore_lifetimes, universal_impl_trait)]

// FIXME can't use this because of format!
//#![deny(elided_lifetime_in_path)]

use crate::prelude::*;
use extern::{
    structopt_derive::StructOpt,
    structopt::{self, StructOpt},
};

macro throw($t:expr) {
    return Err($t.into());
}

mod add;
mod cards;
mod db;
mod errors;
mod git;
mod line_parser;
mod new;
mod status;
mod prelude;
mod quiz;
mod selection;
mod test;
mod uuid_ext;

/// Do fancy things
#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "mathema", about = "a CLI for flashcards")]
struct MathemaOptions {
    #[structopt(name = "directory", help = "where your existing cards can be found")]
    directory: Option<String>,

    #[structopt(short = "f", long = "force", help = "continue despite ignorable errors")]
    force: bool,

    #[structopt(long = "dry-run", help = "do not write changes to disk")]
    dry_run: bool,

    #[structopt(subcommand)]
    command: MathemaCommand,
}

#[derive(StructOpt, Clone, Debug)]
enum MathemaCommand {
    #[structopt(name = "quiz", about = "test yourself")] Quiz {
        #[structopt(help = "what language do you want to learn")]
        language: String
    },

    #[structopt(name = "dump", about = "dump info about cards")] Dump,

    #[structopt(name = "new", about = "create a new deck of cards")]
    New {
        #[structopt(help = "where to create your cards")]
        directory: String,
    },

    #[structopt(name = "status", about = "check on the status of your cards")]
    Status,

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
            ::std::process::exit(1);
        }
    }
}

fn main1() -> Result<(), Error> {
    let args = &MathemaOptions::from_args();

    match &args.command {
        MathemaCommand::Quiz { language } => {
            quiz::quiz(args, language)?;
        }

        MathemaCommand::New { directory } => {
            new::new(args, directory)?;
        }

        MathemaCommand::Status => {
            status::status(args)?;
        }

        MathemaCommand::Add { file } => {
            add::add(args, file)?;
        }

        MathemaCommand::Dump {} => {
            println!("Dumping cards");
        }
    }
    Ok(())
}

impl MathemaOptions {
    crate fn directory(&self) -> Fallible<PathBuf> {
        Ok(match &self.directory {
            Some(s) => Path::new(s).to_owned(),
            None => env::current_dir()?,
        })
    }
}
