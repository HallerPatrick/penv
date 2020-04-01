use std::env;

use ansi_term::Colour::{Blue, Green};
use clap::{App, Arg};

#[macro_use] extern crate prettytable;
use prettytable::Table;

fn main() {

    let matches = App::new("penv")
                        .version("0.1.0")
                        .author("Patrick Haller <patrickhaller40@googlemail.com>")
                        .about("Pretty print environment variables")
                        .arg(Arg::with_name("name")
                             .value_name("PATH")
                             .help("Environment variable name"))
                        .get_matches();



    let var_name: String;
    if let Some(var_name_arg) = matches.value_of("name") {
        var_name = var_name_arg.to_string();
    } else {
        var_name = String::from("PATH");
    }

    let mut path_values: Option<String> = None;
    for (key, value) in env::vars() {
        if key.trim() == var_name.to_uppercase() {
            path_values = Some(value.clone());
        }
    }

    match path_values {
        Some(paths_string) => {
            let paths = paths_string.split(":");

            let mut table = Table::new();

            println!("\n{}", Blue.paint(format!("ENVIRONMENT VARIABLE: {}", Blue.bold().paint(var_name))));

            for (i, path) in paths.enumerate() {
                table.add_row(row![Green.bold().paint((i+1).to_string()), path]);
            }

            table.printstd();

        }

        None => eprintln!("Could not find path variable")
    }
}
