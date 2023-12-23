#![allow(dead_code)]
use std::{
    io::{prelude::*, self, BufReader}, 
    fs::File, collections::HashMap
};

pub struct Frame {
    pub size:(usize, usize),
    pub ColumnType:Vec<Column>
}
pub struct FrameBuilder {
    pub size:Option<(usize, usize)>,
    pub columns:Option<Vec<ColumnType>>
}

pub struct Column {
    pub name:String,
    pub data:ColumnType
}

pub enum ColumnType {
    IntVec(Vec<i32>),
    FloatVec(Vec<f64>),
    StringVec(Vec<String>),
    BoolVec(Vec<bool>)
}

pub struct ColumnBuilder {
    pub name:Option<String>,
    pub data:Option<ColumnType>
}

pub fn open_file(path:impl Into<String>) -> BufReader<File>{
    let file = std::fs::File::open(path.into())
        .expect("File not found");

    BufReader::new(file)
}

fn get_rows(reader:&mut BufReader<File>) -> usize {
    let mut rows:usize = 0;

    for _ in reader.lines() {
        rows += 1;
    }

    //offset the pointer to the beggining of the buffer iterator
    let _ = reader.seek(io::SeekFrom::Start(0));
    
    rows - 1
}
fn get_cols(reader:&mut BufReader<File>) -> usize {
    let mut line:String = String::new();
    reader.read_line(&mut line)
        .expect("Reading from cursor size shouldn't fail");

    let mut cols:usize = 1;
    for c in line.chars() {
        if c == ',' { cols += 1; }
    }

    let _ = reader.seek(io::SeekFrom::Start(0));

    cols
}
pub fn size(reader:&mut BufReader<File>) -> (usize, usize) {
    (get_cols(reader), get_rows(reader))
}

fn numeric_type(val:impl Into<String>) -> (bool, bool){
    let val:String = val.into();

    let mut has_point:bool = false;
    for c in val.chars() {
        if c.is_alphabetic() { return (false, false); }
        else if c == '.' { has_point = true; }
        else if c.is_numeric() {
            continue;
        }
    }
    
    //floating point
    if has_point { return (true, true) }
    //just numeric
    else { return (true, false) }
}

impl Frame {
    // pub fn new(reader:&mut BufReader<File>) -> Frame {
    //     Frame { size: (get_cols(reader), get_rows(reader)),
    //             ColumnType: 
    //         }
    // }

    pub fn col_names(reader: &mut BufReader<File>) -> Vec<String> {
        print!("ColumnType:\n");
        let names:Vec<String> = Self::get_col_names(reader);
        for name in &names {
            println!(" -> {}", name);
        }

        names
    }
    pub fn get_col_names(reader: &mut BufReader<File>) -> Vec<String> {
        let mut raw_names:String = String::new();

        reader.read_line(&mut raw_names)
            .expect("Reading from cursor names shouldn't fail");

        let names:String = raw_names.chars().map(|c| 
            if c == '\"' {' '}
            else if c == ',' {' '}
            else {c}
        ).collect();

        let v:Vec<String> = names.split_whitespace()
            .map(|n| n.to_string()).collect();

        let _ = reader.seek(io::SeekFrom::Start(0));
        v
    }

    pub fn retrieve_data(reader:&mut BufReader<File>) -> Vec<Vec<String>> {
        //cols, rows
        let size = size(reader);

        let mut raw_cols:Vec<Vec<String>> = Vec::with_capacity(size.0);

        for _ in 0..size.0 {
            let col:Vec<String> = Vec::with_capacity(size.1);
            raw_cols.push(col)
        }

        let mut discard:String = String::new();
        let _ = reader.read_line(&mut discard);

        for line in reader.lines() {
            if let Ok(line) = line {
                for l in line.split(',').enumerate() {
                    raw_cols[l.0].push(l.1.to_string());
                }
            }else {
                panic!("Line not found");
            }

        }

        let _ = reader.seek(io::SeekFrom::Start(0));

        raw_cols
    }

    pub fn convert_data(cols:Vec<Vec<String>>) {
        let mut new_cols:Vec<ColumnType> = Vec::with_capacity(cols.len());

        for c in cols {
            let f = numeric_type(&c[0]);

            match f {
                (true, true) => {
                    let result:Result<Vec<f64>, _> = c.iter().map(|s| s.parse()).collect();

                    if let Ok(res) = result { 
                        new_cols.push(ColumnType::FloatVec(res));
                    }

                },
                (true,false) => {
                    let result:Result<Vec<i32>, _> = c.iter().map(|s| s.parse()).collect();

                    if let Ok(res) = result {
                        new_cols.push(ColumnType::IntVec(res));
                    }
                },
                _ => {
                    new_cols.push(ColumnType::StringVec(c));
                }
            }
        }
    }

    // fn convert_data<U, V>(val:String)
    //     where U:From<String> + Into<V> + Debug
    // { 

    //     let mut is_alpha:bool = false;
    //     let mut is_numeric:bool = false;
    //     let mut has_dot:bool = false;
    //     for c in val.chars() {
    //         if c.is_numeric() { is_numeric = true; }
    //         else if c.is_alphabetic() { is_alpha = true; }
    //         else if c == '.' { has_dot = true; }
    //     }

    //     let some:T = val.into();
    // }

}

impl Column {
    pub fn count_values(v:&Vec<String>) -> HashMap<String, usize> {
        let mut map:HashMap<String,usize> = HashMap::new();

        for i in 0..v.len() {
            if map.contains_key(&v[i]) {
                *map.entry(v[i].clone()).or_insert(0) += 1;
            }else {
                map.insert(v[i].clone(), 1);
            }
        }
        map
    }

}

impl ColumnBuilder {
    // pub fn new() -> Self {
    //     ColumnBuilder { name: None, data: None }
    // }

    // pub fn name(mut self, n:&str) -> Self {
    //     self.name = Some(n.to_string());
    //     self
    // }
    // pub fn data(mut self, d:Vec<ColumnType>) -> Self {
    //     self.data = ;
    //     self
    // }

    // pub fn build(self) -> Column {
    //     Column { 
    //         name: self.name.expect("Name Required"), 
    //         data: self.data.expect("Data Required") 
    //     }
    // }
}