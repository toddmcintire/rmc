use clap::{Arg, Command};
use std::{fs::{self, create_dir, FileType, OpenOptions}, path::PathBuf, process};

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
        let result = recursive_folder_check(input);
            if let Err(error) = result {
                eprintln!("Error: {}", error);
            }
    }else if choice == "rc" {
        //list_files_in_dir(output);
        // let result = recursive_copy(input,output);
        //     if let Err(error) = result {
        //         eprintln!("Error: {}", error);
        //     }
        println!("{}",does_folder_exist(output));
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
    list_files_in_dir(output);
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
            create_dir(appended)?;
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

//function currently used to see if anything is in the output folder, if not then create it.
fn list_files_in_dir(dir_path: &String) {
    //creates the folder
    fn my_create_dir(input: &String) -> std::io::Result<()> {
        fs::create_dir(input)?;
        Ok(())
    }

    if let Ok(entries) = fs::read_dir(dir_path) {
        for item in entries {
            if let Ok(item) = item {
                if let Some(file_name) = item.file_name().to_str() {
                    println!("{}",file_name);
                    
                }
                
                if let Ok(meta) = fs::metadata(item.path()) {
                    println!("file? {:?} -- folder? {:?}",meta.is_file(), meta.is_dir());
                }

                if let Ok(meta) = fs::metadata(item.path()) {
                    let file_type = meta.file_type();
                    println!("file type: {:?}", file_type);
                    if file_type.is_dir() == false && file_type.is_file() == false && file_type.is_symlink() == false {
                        println!("what im looking for");
                    }
                }
            }
        }
    } else {
        println!("output directory does not exist creating");
        my_create_dir(dir_path);
    }
}

/// This function returns a bool if the given input string is found or if there is an err.
fn does_folder_exist(input: &String) -> bool{
    let file = OpenOptions::new().read(true).open(input);
    println!("{:?}",file);
    if let Err(..) = file {
        return false
    }
    true
}