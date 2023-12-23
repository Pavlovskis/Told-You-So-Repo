use std::{
    io::{SeekFrom, prelude::*,BufReader}, 
    fs::File
};

pub struct Line(pub String);

pub fn open_file(path:String) -> BufReader<File>{
    let file = std::fs::File::open(path)
        .expect("File not found");

    BufReader::new(file)
}

pub fn buffer_size(reader:&mut BufReader<File>) -> usize {
    let mut rows:usize = 0;
    for _ in reader.lines() {
        rows += 1;
    }

    let _ = reader.seek(SeekFrom::Start(0));

    rows + 1
}

impl Line {
    pub fn to_bin(self) -> String {
        let mut bin_str = String::new();
        
        bin_str = self.0.chars().map( |c|
            format!("{:b}", c as u8)
        ).collect();


        bin_str
    }
}