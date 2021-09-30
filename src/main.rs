mod card;
use std::path::Path;

use clap::{crate_authors, crate_version, App, Arg};

fn main() {
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
                .validator(is_number),
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
                .validator(is_number)
                .default_value("1")
                .default_value_if("default", None, "2"),
        )
        .arg(
            Arg::with_name("default")
                .short("d")
                .help("Use the default profil")
                .long_help(
                    "Use the default profil :\nrandom = true\ntry = 2\nall_cases = true\npass = 2",
                ),
        )
        .get_matches();

    let input = Path::new(matches.value_of("card_path").unwrap());
}

#[inline]
fn is_number(string: String) -> Result<(), String> {
    string
        .parse::<u32>()
        .map(|_| ())
        .map_err(|_| String::from("The value must be a natural number"))
}
