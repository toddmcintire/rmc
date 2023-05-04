use std::fs;
use assert_cmd::Command;
use file_diff::{self, diff_files};

#[test]
fn copy_file_test() {
    if let Ok(_) = fs::write("test.txt", "to be copied") {
        println!("file created");
    }
    //copy_file("test.txt","test-copy.txt");

    let mut cmd = Command::cargo_bin("rmc").unwrap();
    //cmd.assert().success();
    cmd
        .arg("-c")
        .arg("c")
        .arg("test.txt")
        .arg("test_copy.txt");

    cmd.assert();

    let mut file1 = match fs::File::open("test.txt") {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };
    let mut file2 = match fs::File::open("test_copy.txt") {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    assert!(diff_files(&mut file1, &mut file2), "files not the same");

    match fs::remove_file("test.txt") {
        Ok(()) => println!("original deleted"),
        Err(err) => println!("Error: {}", err),
    }
    match fs::remove_file("test_copy.txt") {
        Ok(()) => println!("original deleted"),
        Err(err) => println!("Error: {}", err),
    }
}

#[test]
fn move_file_test() {
    if let Ok(_) = fs::write("test.txt", "to be copied") {
        println!("file created");
    }
    //copy_file("test.txt","test-copy.txt");

    let mut cmd = Command::cargo_bin("rmc").unwrap();
    //cmd.assert().success();
    cmd
        .arg("-c")
        .arg("m")
        .arg("test.txt")
        .arg("test_moved.txt");

    cmd.assert();

    //check if test_moved.txt exists
    //check if test.txt does not exits
}