#![allow(dead_code)]
#![deny(unused_must_use)] // always a bug
#![feature(decl_macro)]
#![feature(in_band_lifetimes)]
#![feature(proc_macro_hygiene)]
#![feature(try_blocks)]
#![feature(crate_visibility_modifier)]
#![feature(async_await)]
#![warn(rust_2018_idioms)]

// FIXME can't use this because of format!
//#![deny(elided_lifetime_in_path)]

// FIXME: File an issue...had some problems due to crazy internal
// macros; really needs hygiene I guess?
//
// FIXME: RLS bug about `extern crate lazy_static` here.
#[macro_use]
extern crate lazy_static;

use crate::prelude::*;
use {structopt::StructOpt, structopt_derive::StructOpt};

macro throw($t: expr) {
    return Err($t.into());
}

mod add;
mod cards;
mod db;
mod dump;
mod errors;
mod git;
mod language;
mod line_parser;
mod new;
mod prelude;
mod quiz;
mod selection;
mod serve;
mod status;
mod test;
mod uuid_ext;

/// Do fancy things
#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "mathema", about = "a CLI for flashcards")]
struct MathemaOptions {
    #[structopt(name = "directory", help = "where your existing cards can be found")]
    directory: Option<String>,

    #[structopt(
        short = "f",
        long = "force",
        help = "continue despite ignorable errors"
    )]
    force: bool,

    #[structopt(long = "dry-run", help = "do not write changes to disk")]
    dry_run: bool,

    #[structopt(subcommand)]
    command: MathemaCommand,
}

#[derive(StructOpt, Clone, Debug)]
enum MathemaCommand {
    #[structopt(name = "quiz", about = "test yourself")]
    Quiz {
        #[structopt(help = "what language do you want to learn")]
        language: String,

        #[structopt(long = "mode", help = "presentation mode (basic or ncurses)")]
        mode: Option<PresentationMode>,

        #[structopt(
            short = "d",
            long = "duration",
            help = "maximum duration in minutes",
            default_value = "10"
        )]
        duration: i64,
    },

    #[structopt(name = "dump", about = "dump info about cards")]
    Dump {
        #[structopt(help = "substring to match against dumped cards")]
        filter: Option<String>,

        #[structopt(
            long = "expired",
            help = "dump only expired cards",
            default_value = "false"
        )]
        expired: bool,
    },

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

    #[structopt(name = "serve", about = "serve information about your cards over JSON")]
    Serve {},
}

fn main() {
    env_logger::init();
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
        MathemaCommand::Quiz {
            language,
            mode,
            duration,
        } => {
            quiz::quiz(args, language, *mode, *duration)?;
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

        MathemaCommand::Dump { filter, expired } => {
            dump::dump(args, filter, *expired)?;
        }

        MathemaCommand::Serve {} => {
            serve::serve(args)?;
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
