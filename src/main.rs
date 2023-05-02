use clap::{Arg, Command};
use std::{fs::{self, create_dir, FileType, OpenOptions}, path::PathBuf, process, fmt::format, collections::HashSet};

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
        // let result = recursive_copy(input,output);
        // if let Err(error) = result {
        //     eprintln!("Error: {}", error);
        // }
        test_recursive_copy(input, output);
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
fn recursive_copy(input: &str, output: &str){
    //checks if output folder exists if not creates it
    if does_folder_exist(output) == false{
        my_create_dir(output);
    }
    //prints if output exists, it should at this point.
    println!("{}",does_folder_exist(output));

    let input_path = std::path::Path::new(input);
    let output_path = std::path::Path::new(output);

    if input_path.is_dir() && output_path.is_dir() {
        if let Ok(element) = fs::read_dir(input) {
            for res in element {
                if let Ok(item) = res {
                    //println!("{:?}", item.path());
                    if let Ok(meta) = fs::metadata(item.path()) {
                         if meta.is_dir() {
                            // create folder in output
                            //println!("copy file 1{:?}", output_path.display());
                            //println!("copy file 2{:?}",item.path().file_name().unwrap().to_str().unwrap());
                            let copy_output = format!("{}{}/",output_path.display(), item.path().file_name().unwrap().to_str().unwrap());
                            fs::create_dir(&copy_output);
                            //recursively call on input folder
                            //println!("!!! {:?}",item.path().to_str().unwrap());
                            //println!("!!!? {:?}", &copy_output.as_str());
                            recursive_copy(item.path().to_str().unwrap(), &copy_output.as_str())
                         }

                         if meta.is_file() {
                            // add output_path + file name
                            let copy_output = format!("{}{}",output_path.display(), item.path().file_name().unwrap().to_str().unwrap());

                            //println!("copy file 1{:?}", output_path.display());
                            //println!("copy file 2{:?}",item.path().file_name().unwrap().to_str().unwrap());
                            // copy file 
                            //println!("copy output {:?}", &copy_output);
                            fs::copy(item.path(), &copy_output);
                         }
                    }
                }
            }
        }

    }
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

//creates the folder
fn my_create_dir(input: &str) {
    if let Ok(_) = fs::create_dir(input) {
        println!("created dir");
    }
}

/// This function returns a bool if the given input string is found or if there is an err.
fn does_folder_exist(input: &str) -> bool{
    let file = OpenOptions::new().read(true).open(input);
    println!("{:?}",file);
    if let Err(..) = file {
        return false
    }
    true
}

