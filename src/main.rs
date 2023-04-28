use clap::{Arg, Command};
use std::{fs::{self, create_dir_all}, path::PathBuf};

fn main() {
    let matches = Command::new("rmc")
        .version("1.1.0")
        .author("Todd McIntire <mail@toddmcintire.com>")
        .about("command line program to move and copy files")
        .arg(
            Arg::new("choice")
                .short('c')
                .long("choice")
                .help("m or move to move file & c or copy to copy file")
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
        copy_file(input, output);
    } else if choice == "m" || choice == "move"{
        move_file(input, output);
    } else if choice == "r" {
        recursive_folder_check(input);
    }else if choice == "rc" {
        recursive_copy(input, output);
    }else {
        panic!("incorrect choice aborting")
    }

    
}

/// recursively checks if input is folder or file
/// 
/// # Arguments
/// 
/// * `input` - input path to file
/// 
/// # Examples
/// 
/// ```
/// recursive_folder_check("file.txt");
/// ```
fn recursive_folder_check (input: &String) -> std::io::Result<()>{
    for element in  fs::read_dir(input)? {
        let dir = element?;
        let meta = fs::metadata(dir.path())?;
        let file_type = meta.file_type();
        let path_buf = PathBuf::from(dir.path());
        let path_str = path_buf.to_str().unwrap();
        let path_string = String::from(path_str);
        println!("path: {:?} & is dir: {:?}",dir.path(),file_type.is_dir());
        if file_type.is_dir() {
            recursive_folder_check(&path_string)?;
        } else if  file_type.is_file(){
            println!("{} is a file", path_str);
        } else {
            println!("unknown file type");
        }
        
    }
    Ok(())
}

/// recursively copies files and folders 
/// 
/// # Arguments
/// 
/// * `input` - input path to file
/// * `output` - output path to file
/// 
/// # Examples
/// 
/// ```
/// recursive_copy("folder/file.txt","another_folder/");
/// ```
fn recursive_copy(input: &String, output: &String) -> std::io::Result<()>{
    for element in fs::read_dir(input)?  {
        let dir = element?;
        let meta = fs::metadata(dir.path())?;
        let file_type = meta.file_type();   //sets file type
        let path_buf = PathBuf::from(dir.path());   //sets path buffer
        let path_str = path_buf.to_str().unwrap();  //sets path str
        let path_string = String::from(path_str);   //sets path string
        println!("dir.path: {:?} & file_type.is_dir: {:?} & path_str {:?} & path_string {:?}",dir.path(),file_type.is_dir(), path_str, path_string);
        if file_type.is_dir() {
            let appended = format!("{}/{}",output,path_str);
            println!("{}",appended);
            create_dir_all(appended)?;
            recursive_copy(&path_string, output)?;
        } else if  file_type.is_file(){
            let appended = format!("{}/{}",output,path_str);
            println!("{} is a file", path_str);
            copy_file(&path_string, &appended);
        } else {
            println!("unknown file type");
        }
    }
    Ok(())
}

/// copies a single file from one location to another
/// 
/// # Arguments
/// 
/// * `input` - input path to file
/// * `output` - output path for desired file
/// 
/// # Examples
/// 
/// ```
/// copy_file("file.txt","folder/copy.txt");
/// ```
/// 
/// This function copies a file from one location to another with all the original permissions.
fn copy_file(input: &String, output: &String) {
    match fs::copy(input, output) {
        Ok(bytes) => println!("{} bytes copied", bytes),
            Err(err) => println!("Error: {}", err),
    }
}

/// moves a single file from one location to another
/// 
/// # Arguments
/// 
/// * `input` - input path to file
/// * `output` - output path for desired file
/// 
/// # Examples
/// 
/// ```
/// move_file("file.txt","folder/");
/// ```
/// 
/// This function moves a file from one location to another with all the original permissions.
fn move_file(input: &String, output: &String) {
    //copy file like above then delete file with fs::remove_file()
    match fs::copy(input, output) {
        Ok(bytes) => println!("{} bytes copied", bytes),
            Err(err) => println!("Error: {}", err),
    }
    match fs::remove_file(input) {
        Ok(()) => println!("original deleted"),
        Err(err) => println!("Error: {}", err),
    }
}