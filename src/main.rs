#![feature(crate_in_paths, crate_visibility_modifier, extern_absolute_paths, decl_macro,
           termination_trait, use_nested_groups, universal_impl_trait)]

use structopt_derive::StructOpt;
use structopt::StructOpt;

mod data;

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
}

fn main() {
    match Mathema::from_args() {
        Mathema::Quiz {} => {
            println!("Don't you feel smarter?");
        }

        Mathema::New { directory } => {
            println!("Loading file: `{}`", directory);
        }

        Mathema::Dump {} => {
            println!("Dumping cards");
        }
    }
}
