use std::fs;

/// creates folder from given input string
/// 
/// # Arguments
/// 
/// * `input` - input path to file
/// 
/// # Examples
/// 
/// ```
/// my_create_dir("test_dir");
/// ```
pub fn my_create_dir(input: &str) -> std::io::Result<()>{
    if input.is_empty() {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Empty directory name"));
    }
    fs::create_dir(input)?;
    //println!("created dir");
    Ok(())
}

/// This function returns a bool if the given input string is found or if there is an err.
///
/// # Arguments
/// 
/// * `input` - input path to folder
/// 
/// # Examples
/// 
/// ```
/// does_folder_exist("test_dir");
/// ```
pub fn does_folder_exist(input: &str) -> bool{
    let file = fs::OpenOptions::new().read(true).open(input);
    //println!("{:?}",file);
    if let Err(..) = file {
        return false
    }
    true
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
pub fn move_file(input: &str, output: &str) {
    copy_file(input, output);
    match fs::remove_file(input) {
        Ok(()) => println!("original deleted"),
        Err(err) => println!("Error: {}", err),
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
pub fn copy_file(input: &str, output: &str) {
    match fs::copy(input, output) {
        Ok(bytes) => println!("{} bytes copied", bytes),
            Err(err) => println!("Error: {}", err),
    }
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
pub fn recursive_copy(input: &str, output: &str){
    //checks if output folder exists if not creates it
    if does_folder_exist(output) == false{
        if let Ok(_) = my_create_dir(output){
            //println!("dir created");
        } // cannot use else clause here, produces an infinite loop error in windows.
    }

    let input_path = std::path::Path::new(input);
    let output_path = std::path::Path::new(output);

    if input_path.is_dir() && output_path.is_dir() {
        if let Ok(element) = fs::read_dir(input) {
            for res in element {
                if let Ok(item) = res {
                    if let Ok(meta) = fs::metadata(item.path()) {
                         if meta.is_dir() {
                            // create folder in output
                            let copy_output = output_path.join(item.path().file_name().unwrap());
                            if let Ok(_) = my_create_dir(copy_output.to_str().unwrap()) {
                                //println!("dir created");
                            } else if let Err(error) = my_create_dir(copy_output.to_str().unwrap()) {
                                eprintln!("error creating dir {}",error);
                            }
                            // recursively call on input folder
                            recursive_copy(item.path().to_str().unwrap(), copy_output.to_str().unwrap())
                         }

                         if meta.is_file() {
                            // add output_path + file name
                            let copy_output = output_path.join(item.path().file_name().unwrap());

                            // copy file 
                            if let Ok(_) = fs::copy(item.path(), &copy_output) {
                                //println!("copy successful");
                            } else {
                                eprintln!("error copying");
                            }
                         }
                    }
                }
            }
        }

    }
}

/// recursively moves files and folders 
/// 
/// # Arguments
/// 
/// * `input` - input path to file
/// * `output` - output path to file
/// 
/// # Examples
/// 
/// ```
/// recursive_move("folder/file.txt","another_folder/");
/// ```
pub fn recursive_move(input: &str, output: &str){
    //calls 
    recursive_copy(input, output);
    if let Ok(_) = fs::remove_dir_all(input) {
        println!("original removed");
    } else {
        eprintln!("error in removing original");
    }
}