#![allow(dead_code)]

mod ask;
mod card;
mod load;

use std::error::Error;
use std::num::NonZeroU32;
use std::{convert, path::Path};

use clap::{crate_authors, crate_version, App, Arg};

use crate::{
    card::{collection::Collection, deck::Deck},
    load::load_data_file,
};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Pixo")
        .version(crate_version!())
        .about("Pixo is a CLI fashcard app")
        .author(crate_authors!())
        .arg(
            Arg::with_name("card_path")
                .required(true)
                .takes_value(true)
                .validator(|path| {
                    let path = Path::new(&path);
                    if path.is_file() {
                        if path.extension().unwrap() == "json" {
                            Ok(())
                        } else {
                            Err(format!("The file {:?} is not as json file.", path))
                        }
                    } else if path.is_dir() {
                        path.read_dir()
                            .map(|mut files| {
                                if files.any(|file| {
                                    file.expect(&format!("Error during getting data in {:?}", path))
                                        .file_name()
                                        .into_string()
                                        .map(|string| {
                                            string
                                                .as_str()
                                                .split(".")
                                                .last()
                                                .map(|extention| extention == "json")
                                        })
                                        .ok()
                                        .flatten()
                                        .unwrap_or(false)
                                }) {
                                    Ok(())
                                } else {
                                    Err(format!(
                                        "There don't are any json file in the folder {:?}",
                                        path
                                    ))
                                }
                            })
                            .unwrap_or(Err(format!("Error during getting data in {:?}", path)))
                    } else {
                        Err(format!("The file {:?} does not exist.", path))
                    }
                }),
        )
        .arg(
            Arg::with_name("verso")
                .help("Ask the verso insted of the rerco of each card.")
                .short("v")
                .long("verso")
                .conflicts_with("random"),
        )
        .arg(
            Arg::with_name("random")
                .help("Inverse randomly the recto a verso of each card.")
                .short("r")
                .long("random"),
        )
        .arg(
            Arg::with_name("try")
                .long("try")
                .takes_value(true)
                .help("Set the numbre of try for each question. 0 means infinity of try.")
                .default_value("2")
                .validator(is_number_non_zero),
        )
        .arg(
            Arg::with_name("all_cases")
                .requires_all(&["random"])
                .alias("ac"),
        )
        .arg(
            Arg::with_name("pass")
                .help("Set the nombre of time the deck will be used.")
                .short("p")
                .validator(is_number_non_zero)
                .default_value("1")
                .default_value_if("default", None, "2"),
        )
        .arg(
            Arg::with_name("default")
                .short("d")
                .help("Use the default profil")
                .long_help(
                    "Use the default profil :\nrandom = true\ntry = 2\nall_cases = true\npass = 2\nWARNING : These parametres can be overrided.",
                ),
        )
        .get_matches();

    let input = Path::new(matches.value_of("card_path").unwrap());

    if input.is_dir() {
        panic!("Pixo can not read a folder of data files (.json files) yet.")
    }

    let data_file = load_data_file(input)?;
    let deck = Deck::from(data_file);

    let mut collection = Collection::new(deck, rand::thread_rng());

    if matches.is_present("default") {
        if !matches.is_present("verso") {
            collection.random_mode()
        }
        collection.all_cases_mode();
        collection.pass(NonZeroU32::new(2).unwrap())
    }

    if matches.is_present("verso") {
        collection.verso_mode()
    } else if matches.is_present("random") {
        collection.random_mode();

        if matches.is_present("all_cases") {
            collection.all_cases_mode()
        }
    }

    if let Some(pass) = matches.value_of("pass") {
        let pass = NonZeroU32::new(pass.parse::<u32>().unwrap()).unwrap();

        collection.pass(pass)
    }

    if matches.is_present("default") {
        collection.random_mode();
        collection.all_cases_mode();
        collection.pass(unsafe { NonZeroU32::new_unchecked(2) })
    }

    Ok(())
}

#[inline]
fn is_number_non_zero(string: String) -> Result<(), String> {
    string
        .parse::<u32>()
        .map(|nbr| {
            if nbr == 0 {
                Err(String::from("Need to be at lease 1"))
            } else {
                Ok(())
            }
        })
        .map_err(|_| String::from("The value must be a natural number"))
        .and_then(convert::identity)
}
