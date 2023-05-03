use clap::{Arg, Command};
mod functions;

fn main() {
    let matches = Command::new("rmc")
        .version("1.1.0")
        .author("Todd McIntire <mail@toddmcintire.com>")
        .about("command line program to move and copy files")
        .arg(
            Arg::new("choice")
                .short('c')
                .long("choice")
                .help("m to move, c to copy, rc to recursively copy, rm to recursively move")
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

    if choice == "c" || choice == "copy"{
        functions::copy_file(input, output);
    } else if choice == "m" || choice == "move"{
        functions::move_file(input, output);
    }else if choice == "rc" {
        functions::recursive_copy(input, output);
    }else if choice == "rm" {
        functions::recursive_move(input, output);
    }else {
        panic!("incorrect choice aborting")
    }

    
}