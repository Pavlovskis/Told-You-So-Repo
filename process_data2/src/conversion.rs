use std::{
    io::{self, prelude::*, BufReader}, 
    fs::File
};
use chrono::NaiveDate;

use crate::{
    column::{ColumnType, Column, DataType, ColumnBuilder}, 
    helpe::{check_string, is_na}, Frame
};

pub struct FromCSV ( Frame );

impl FromCSV {
    pub fn csv(reader:&mut BufReader<File>) -> Frame {
        let fin:Vec<ColumnType> = refine_data(get_raw_data(reader));
        let names:Vec<String> = csv_names(reader);

        let mut cols:Vec<Column> = Vec::with_capacity(names.len());

        let mut i:usize = 0;
        for col in fin {
            #[allow(unused_assignments)]
            let mut dt:DataType = DataType::None;
            match col {
                ColumnType::StringVec(_) => {
                    dt = DataType::String;
                },ColumnType::IntVec(_) => {
                    dt = DataType::Int;
                },ColumnType::FloatVec(_) => {
                    dt = DataType::Float;
                },ColumnType::BoolVec(_) => {
                    dt = DataType::Bool;
                },ColumnType::DateVec(_) => {
                    dt = DataType::NaiveDate;
                },ColumnType::Empty() => continue,
            }

            let new_col = ColumnBuilder::new()
                .pos(i)
                .name(names[i].clone())
                .datatype(dt)
                .values(col)
            .build();

            cols.push(new_col);
            i += 1;
        }

        Frame { 
            size:(csv_cols(reader), csv_rows(reader)),
            data:cols,
            col_indexed:Vec::new()
        }

    }

}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

//Categorizes the Vec<String> into the respectives datatypes.
pub fn refine_data(str_cols:Vec<Vec<String>>) -> Vec<ColumnType> {

    let mut data_cols:Vec<ColumnType> = Vec::with_capacity(str_cols.len());

    for c in str_cols {
        let mut i:usize = 0;
        let mut val:String = c[0].to_lowercase();
        while is_na(&val) && i < c.len() {
            i += 1;
            val = c[i].to_lowercase();
        }
        let string_type = check_string(c[i].clone());

        match string_type {
            (true, true) => {
                let mut f_vec:Vec<Option<f64>> = Vec::with_capacity(c.len());
                for val in c {
                    if is_na(&val) {
                        f_vec.push(None);
                    }else {
                        let int_val:Result<f64, _> = val.parse();
                        if let Ok(n) = int_val { f_vec.push(Some(n)); }
                        else { f_vec.push(None); }
                    }
                }
                data_cols.push(ColumnType::FloatVec(f_vec));
            },(true, false) => {
                let mut int_vec:Vec<Option<i32>> = Vec::with_capacity(c.len());
                for val in c {
                    if is_na(&val) {
                        int_vec.push(None);
                    }else {
                        let int_val:Result<i32, _> = val.parse();
                        if let Ok(n) = int_val { int_vec.push(Some(n)); }
                        else { int_vec.push(None); }
                    }
                }
                data_cols.push(ColumnType::IntVec(int_vec));
            },(false, true) => {
                let mut date_vec:Vec<Option<NaiveDate>> = Vec::with_capacity(c.len());
                for val in c {
                    if is_na(&val) { date_vec.push(None); }
                    else {
                        if let Ok(date) = NaiveDate::parse_from_str(&val, "%d/%m/%Y"){
                            date_vec.push(Some(date));
                        }else { date_vec.push(None); }
                    }
                }
                data_cols.push(ColumnType::DateVec(date_vec));
            }
            _ => {
                let mut new_vec:Vec<Option<String>> = Vec::with_capacity(c.len());
                for val in c {
                    if is_na(&val) { new_vec.push(None); }
                    else { new_vec.push(Some(val)); }
                }
                data_cols.push(ColumnType::StringVec(new_vec));
            },
        }
    }
    data_cols
}

pub fn csv_rows(reader:&mut BufReader<File>) -> usize {
    let mut rows:usize = 0;
    for _ in reader.lines() {
        rows += 1;
    }

    //offset the pointer to the beggining of the buffer iterator
    let _ = reader.seek(io::SeekFrom::Start(0));
    rows - 1
}

pub fn csv_cols(reader:&mut BufReader<File>) -> usize {
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

pub fn csv_names(reader: &mut BufReader<File>) -> Vec<String> {
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

pub fn csv_size(reader:&mut BufReader<File>) -> (usize, usize) {
    (csv_cols(reader), csv_rows(reader))
}

//puts each col into Vec<String>
pub fn get_raw_data(reader:&mut BufReader<File>) -> Vec<Vec<String>> {
    //cols, rows
    let size = csv_size(reader);

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