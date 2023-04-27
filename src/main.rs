use clap::{Arg, Command};
use std::fs;

fn main() {
    let matches = Command::new("rmc")
        .version("1.0.0")
        .author("Todd McIntire <mail@toddmcintire.com>")
        .about("command line program to move and copy files")
        .arg(
            Arg::new("choice")
                .short('c')
                .long("choice")
                .help("move or copy")
                .num_args(1)
                .required(true)
        )
        .arg(
            Arg::new("input")
                .value_name("FILE")
                .help("file input")
                .num_args(1)
                .required(true)
        )
        .arg(
            Arg::new("output")
                .value_name("FILE")
                .help("file output")
                .num_args(1)
                .required(true)

        )
        .get_matches();

    let choice = matches.get_one::<String>("choice").unwrap();
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output").unwrap();

    println!(
            "choice {:?} input file {:?} output file {:?}", choice, input, output
    );

    if choice == "c" {
        match fs::copy(input,output) {
            Ok(bytes) => println!("{} bytes copied", bytes),
            Err(err) => println!("Error: {}", err),
        }
    } else if choice == "m" {
        //copy file like above then delete file with fs::remove_file()
        match fs::copy(input,output) {
            Ok(bytes) => println!("{} bytes copied", bytes),
            Err(err) => println!("Error: {}", err),
        }
        match fs::remove_file(input) {
            Ok(()) => println!("original deleted"),
            Err(err) => println!("Error: {}", err),
        }
    } else {
        panic!("incorrect choice aborting")
    }

    
}