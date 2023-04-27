use clap::{Arg, Command};
use std::{fs, path::PathBuf};

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
    } else if choice == "r" {
        recursive_folder_check(input);
    }else {
        panic!("incorrect choice aborting")
    }

    
}


fn recursive_folder_check (input: &String) -> std::io::Result<()>{
    for element in  fs::read_dir(input)? {
        let dir = element?;
        let meta = fs::metadata(dir.path())?;
        let file_type = meta.file_type();
        let path_buf = PathBuf::from(dir.path());
        let path_str = path_buf.to_str().unwrap();
        let path_string = String::from(path_str);
        println!("path: {:?} & type: {:?}",dir.path(),file_type.is_dir());
        if file_type.is_dir() {
            recursive_folder_check(&path_string);
        } else if  file_type.is_file(){
            println!("{} is a file", path_str);
        } else {
            println!("unknown file type");
        }
        
    }
    Ok(())
}