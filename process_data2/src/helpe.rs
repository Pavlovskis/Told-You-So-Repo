#![allow(dead_code)]
use chrono::{NaiveDate, Datelike};

pub struct Date (
    pub NaiveDate
);

impl Date {

    pub fn add(self, val:i32) -> Option<NaiveDate> {
        let total:i32 = self.0.num_days_from_ce() + val;
        NaiveDate::from_num_days_from_ce_opt(total)
    }
    pub fn sub(self, val:i32) -> Option<NaiveDate> {
        let total:i32 = self.0.num_days_from_ce() - val;
        NaiveDate::from_num_days_from_ce_opt(total)
    }
    pub fn div(self, val:i32) -> Option<NaiveDate> {
        let total:i32 = self.0.num_days_from_ce() / val;
        NaiveDate::from_num_days_from_ce_opt(total)
    }
    pub fn mul(self, val:i32) -> Option<NaiveDate> {
        let total:i32 = self.0.num_days_from_ce() * val;
        NaiveDate::from_num_days_from_ce_opt(total)
    }

}

pub fn check_string(val:impl Into<String>) -> (bool, bool){
    let val:String = val.into();

    let mut has_point:bool = false;
    let mut has_slash:bool = false;
    for c in val.chars() {
        if c.is_alphabetic() { return (false, false); }
        else if c == '.' { has_point = true; }
        else if c == '/' { has_slash = true; }
        else if c.is_numeric() {
            continue;
        }
    }
    
    //floating point
    if has_point { return (true, true) }
    //date type
    else if has_slash && !has_point { return (false, true) }
    //just numeric
    else { return (true, false) }
}

pub fn is_na(val:impl Into<String>) -> bool{
    let val:String = val.into();
    if val.to_lowercase().contains("na") {
        return true;
    }
    false
}

pub fn type_of<T>(_: &T) -> &str{
    std::any::type_name::<T>()
}

pub fn find_some_val<T>(v:&Vec<Option<T>>) -> &T{

    let mut sample:&T = v[0].as_ref().unwrap();
    for value in v {
        if let Some(val) = value {
            sample = val;
            break;
        }
    }
    sample
}

pub mod printer {
    use colored::Colorize;

    const SPACE_LIM:i32 = 18;

    pub fn print_names(v:&Vec<String>) {
        print_clr_cell(&String::from("  # "));
        for i in 0..v.len() {
            let name:&String = &v[i];
            print_clr_cell(name);
        }
        println!();
        print_dash(v);
    }
    pub fn print_dash(v:&Vec<String>) {
        print_clr_cell(&String::from(" ---"));
        for i in 0..v.len() {
            let len:usize = v[i].len();
            let mut s:String = String::with_capacity(len);
            for _ in 0..len {
                s.push('-');
            }
            print_clr_cell(&s);
        }
        println!();
    }
    
    pub fn print_names_without(v:&Vec<String>) {
        
        for i in 0..v.len() {
            let name:&String = &v[i];
            print_clr_cell(name);
        }
        println!();
    }
    pub fn print_dash_without(v:&Vec<String>) {
        for i in 0..v.len() {
            let len:usize = v[i].len();
            let mut s:String = String::with_capacity(len);
            for _ in 0..len {
                s.push('-');
            }
            print_clr_cell(&s);
        }
        println!();
    }

    pub fn print_cell(s:&String) {
        let len:i32 = s.len() as i32;
        print!("{} ", s);
        if SPACE_LIM - len <= 0 { return; }
        
        for _ in 0..(SPACE_LIM-len-1) { print!(" "); }
    }
    pub fn print_clr_cell(s:&String) {
        let len:i32 = s.len() as i32;
        print!("{} ", s.blue().bold());
        if SPACE_LIM - len <= 0 { return; }
        
        for _ in 0..(SPACE_LIM-len-1) { print!(" "); }
    }

    pub fn print_none() {
        print!("{}","None".red().bold());
        for _ in 0..SPACE_LIM-4 { print!(" "); }
    }
}