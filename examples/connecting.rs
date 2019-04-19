extern crate ftp;

use std::str;
use std::io::Cursor;
use ftp::{FtpStream, FtpError};

fn test_ftp(addr: &str, user: &str, pass: &str) -> Result<(), FtpError> {
    let mut ftp_stream = FtpStream::connect((addr, 21)).unwrap();
    ftp_stream.login(user, pass).unwrap();
    println!("current dir: {}", ftp_stream.pwd().unwrap());

    ftp_stream.cwd("test_data").unwrap();

    // An easy way to retrieve a file
    let cursor = ftp_stream.simple_retr("ftpext-charter.txt").unwrap();
    let vec = cursor.into_inner();
    let text = str::from_utf8(&vec).unwrap();
    println!("got data: {}", text);

    // Store a file
    let file_data = "Some awesome file data man!!".to_string();
    let mut reader = Cursor::new(file_data.into_bytes());
    ftp_stream.put("my_random_file.txt", &mut reader).unwrap();

    ftp_stream.quit()
}

fn main() {
    test_ftp("127.0.0.1", "anonymous", "rust-ftp@github.com").unwrap();
    println!("test successful")
}
