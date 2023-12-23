use std::collections::HashMap;

use chrono::{NaiveDate, Datelike};
use colored::Colorize;

use crate::helpe::printer;

#[derive(Debug, Clone)]
pub struct Column {
    pub pos:usize,
    pub name:String,
    pub datatype:DataType,
    // positions that need operations
    pub op_pos:Option<Vec<usize>>,
    pub values:ColumnType
}

pub struct ColumnBuilder {
    pub pos:Option<usize>,
    pub name:Option<String>,
    pub datatype:DataType,
    pub op_pos:Option<Vec<usize>>,
    pub values:Option<ColumnType>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    StringVec(Vec<Option<String>>),
    IntVec(Vec<Option<i32>>),
    FloatVec(Vec<Option<f64>>),
    BoolVec(Vec<Option<bool>>),
    DateVec(Vec<Option<NaiveDate>>),
    // ...
    Empty()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    NaiveDate,
    // ...
    None,
}

pub trait ConvertibleVal {
    fn cvrt_to_i32(&self) -> Option<i32>;
    fn cvrt_to_f64(&self) -> Option<f64>;
    fn cvrt_to_str(&self) -> Option<&str>;    
    fn cvrt_to_bool(&self) -> Option<bool>;
    fn cvrt_to_date(&self) -> Option<NaiveDate>;
}
impl ConvertibleVal for i32 {
    fn cvrt_to_i32(&self) -> Option<i32> { Some(*self) }
    fn cvrt_to_f64(&self) -> Option<f64> { Some(*self as f64) }
    fn cvrt_to_str(&self) -> Option<&str> { None }
    fn cvrt_to_bool(&self) -> Option<bool> {
        if *self == 1 { Some(true) }
        else if *self == 0 { Some(false) }
        else { None }
    }
    fn cvrt_to_date(&self) -> Option<NaiveDate> { NaiveDate::from_num_days_from_ce_opt(*self) }

}
impl ConvertibleVal for f64 {
    fn cvrt_to_i32(&self) -> Option<i32> { Some(*self as i32) }
    fn cvrt_to_f64(&self) -> Option<f64> { Some(*self) }
    fn cvrt_to_str(&self) -> Option<&str> { None }
    fn cvrt_to_bool(&self) -> Option<bool> {
        if *self == 1.0 { Some(true) }
        else if *self == 0.0 { Some(false) }
        else { None }
    }
    fn cvrt_to_date(&self) -> Option<NaiveDate> { NaiveDate::from_num_days_from_ce_opt(*self as i32) }
}
impl ConvertibleVal for &str {
    fn cvrt_to_i32(&self) -> Option<i32> { self.parse().ok() }
    fn cvrt_to_f64(&self) -> Option<f64> { self.parse().ok() }
    fn cvrt_to_str(&self) -> Option<&str> { Some(self) }
    fn cvrt_to_bool(&self) -> Option<bool> {
        if self.to_lowercase() == "true" { Some(true) }
        else if self.to_lowercase() == "false" { Some(false) }
        else { None }
    }
    fn cvrt_to_date(&self) -> Option<NaiveDate> { 
        if let Ok(date) = NaiveDate::parse_from_str(&self, "%d/%m/%Y") {
            Some(date) }
        else { None }
    }
}
impl ConvertibleVal for NaiveDate {
    fn cvrt_to_i32(&self) -> Option<i32> { 
        Some(NaiveDate::num_days_from_ce(&self)) 
    }
    fn cvrt_to_f64(&self) -> Option<f64> { 
        Some(NaiveDate::num_days_from_ce(&self) as f64) 
    }
    fn cvrt_to_str(&self) -> Option<&str> { // need to find something 
        todo!()
    }    
    fn cvrt_to_bool(&self) -> Option<bool> { None }
    fn cvrt_to_date(&self) -> Option<NaiveDate> {
        Some(*self)
    }
}
impl ConvertibleVal for bool {
    fn cvrt_to_i32(&self) -> Option<i32> { 
        if *self { return Some(1); }
        else if *self == false {
            return Some(0);    
        }else { None }
    }
    fn cvrt_to_f64(&self) -> Option<f64> { 
        if *self { return Some(1.0); }
        else if *self == false {
            return Some(0.0);
        }else { None }    
    }
    fn cvrt_to_str(&self) -> Option<&str> {
        if *self { return Some("true"); }
        else if *self == false {
            return Some("false");
        }else { None }    
    }
    fn cvrt_to_bool(&self) -> Option<bool> {
        Some(*self)
    }
    fn cvrt_to_date(&self) -> Option<NaiveDate> { None }
}

impl Column {

    pub fn count_values(&self) -> HashMap<String, usize> {
        match &self.values {
            ColumnType::StringVec(v) => {
                let mut map:HashMap<String, usize> = HashMap::with_capacity(10);
                map.insert(String::from("None"), 0);
                for val in v {
                    if let Some(s) = val {
                        *map.entry(s.to_string()).or_insert(0) += 1;
                    }else {
                        map.entry(String::from("None")).and_modify(|v| *v += 1);
                    }
                }
                print_count(&map, self.name.clone());
                map

            }, _ => return HashMap::new()
        }
    }

    pub fn describe(&self) -> Column {
        let new_col = ColumnBuilder::new().pos(self.pos.clone()).name(&self.name.clone()).op_pos(None);

        match &self.values {
            ColumnType::IntVec(v) => { 
                let col = new_col.datatype(DataType::Float).values(ColumnType::FloatVec(Self::desc_num(v))).build();
                col
            },ColumnType::FloatVec(v) => { 
                let col = new_col.datatype(DataType::Float).values(ColumnType::FloatVec(Self::desc_num(v))).build();
                col
            },ColumnType::StringVec(v) => { 
                let col = new_col.datatype(DataType::String).values(ColumnType::StringVec(Self::desc_str(v))).build();
                col
            },ColumnType::DateVec(v) => {
                let col = new_col.datatype(DataType::NaiveDate).values(ColumnType::DateVec(Self::desc_date_num(v))).build();
                col
            }, _ => ColumnBuilder::empty(),
        }
    }



    fn desc_num<T> (v:&Vec<Option<T>>) -> Vec<Option<f64>>
        where T:Into<f64> + Copy
    {
        fn round_dec(value: f64) -> f64 { (value * 1000.0).round() / 1000.0 }
        
        let mut f_vec:Vec<f64> = Vec::with_capacity(v.len());
        let mut max:f64 = f64::MIN;
        let mut min:f64 = f64::MAX;
        let mut sum:f64 = 0.0;
        for value in v {
            if let Some(val) = value {
                let new_val = *val;
                let fin:f64 = new_val.into();
                max = max.max(fin); 
                min = min.min(fin);
                sum += fin;
                f_vec.push(fin);
            }
        }

        let mean:f64 = sum / v.len() as f64;

        let pow_cnt:f64 = f_vec.iter().map(|x| 
            (x - mean).abs().powi(2)
        ).sum();

        let std_dev:f64 = (pow_cnt / f_vec.len() as f64).sqrt();

        f_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut quarts:Vec<f64> = Vec::with_capacity(3);
        let mut i:f64 = 1.0;
        while i < 4.0 {
            let q:f64 = f_vec[((i/4.0) * f_vec.len() as f64).round() as usize];
            quarts.push(q);
            i += 1.0;
        }

        vec![None, None, None, Some(round_dec(mean)), Some(round_dec(std_dev)), Some(min), Some(quarts[0]), Some(quarts[1]), Some(quarts[2]), Some(max)]
    }

    fn desc_str(v:&Vec<Option<String>>) -> Vec<Option<String>> {
        let mut map:HashMap<String, usize> = HashMap::with_capacity(25);

        map.insert(String::from("None"), 0);
        for val in v {
            if let Some(s) = val {
                if map.contains_key(s) {
                    map.entry(s.to_string()).and_modify(|x| *x += 1);
                }else { map.insert(s.to_string(), 1); }
            }else {
                map.entry(String::from("None")).and_modify(|x| *x += 1); 
            }
        }

        let mut top:String = String::new();
        let mut freq:usize = 0;
        let unique:usize = map.len() - 1;
        for (k, v) in map {
            if v > freq {
                freq = v; top = k;
            }
        }
        vec![Some(unique.to_string()), Some(top), Some(freq.to_string()), None, None, None, None, None, None, None]
    }

    fn desc_date_num(v:&Vec<Option<NaiveDate>>) -> Vec<Option<NaiveDate>> {
        let mut days:Vec<i32> = Vec::with_capacity(v.len());
        let mut min:i32 = i32::MAX;
        let mut max:i32 = i32::MIN;
        let mut sum:i64 = 0;
        for val in v {
            if let Some(date) = val {
                let d:i32 = date.num_days_from_ce();
                max = max.max(d);
                min = min.min(d);
                sum += d as i64;
                days.push(d);
            }   
        } 
        let mean:f64 = sum as f64 / days.len() as f64;
        let pow_cnt:f64 = days.iter().map(|x|
            (*x as f64 - mean).abs().powi(2)
        ).sum();
        let std_dev:f64 = (pow_cnt / days.len() as f64).sqrt();
        
        days.sort();

        let mut quarts:Vec<i32> = Vec::with_capacity(3);
        let mut i:f64 = 1.0;
        while i < 4.0 {
            let q:i32 = days[((i/4.0) * days.len() as f64).round() as usize];
            quarts.push(q);
            i += 1.0;
        }

        vec![ None, None, None, 
            NaiveDate::from_num_days_from_ce_opt(mean.round() as i32), NaiveDate::from_num_days_from_ce_opt(std_dev.round() as i32), NaiveDate::from_num_days_from_ce_opt(min), 
            NaiveDate::from_num_days_from_ce_opt(quarts[0]), NaiveDate::from_num_days_from_ce_opt(quarts[1]), NaiveDate::from_num_days_from_ce_opt(quarts[2]), NaiveDate::from_num_days_from_ce_opt(max) 
        ]
    }
}

impl ColumnBuilder {

    pub fn empty() -> Column {
        Column {
            pos: 0,
            name: String::from(""),
            datatype:DataType::None,
            op_pos: None,
            values: ColumnType::Empty()
        }
    }
    pub fn new() -> Self {
        ColumnBuilder { 
            pos: None,
            name: None, 
            datatype: DataType::None, 
            op_pos: None,
            values:None
        }
    }
    pub fn pos(mut self, pos:usize) -> Self {
        self.pos = Some(pos);
        self
    }
    pub fn name(mut self, name:impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn datatype(mut self, data_type:DataType) -> Self {
        self.datatype = data_type;
        self
    }
    pub fn op_pos(mut self, pos:Option<Vec<usize>>) -> Self {
        self.op_pos = pos;
        self
    }
    pub fn values(mut self, values:ColumnType) -> Self {
        self.values = Some(values);
        self
    }
    pub fn build(self) -> Column {
        Column {
            pos: self.pos.unwrap(),
            name: self.name.unwrap(),
            datatype: self.datatype, 
            op_pos: self.op_pos,
            values: self.values.unwrap()
        }
    }

}

fn print_desc(c:&Column) {
    match &c.values {
        ColumnType::StringVec(v) => {
            printer::print_names(&vec![c.name.clone()]);
            let t:Vec<&str> = vec!["UNIQUE", "TOP", "FREQ"];
            for i in 0..t.len() {
                print!(" ");
                printer::print_clr_cell(&t[i].to_string());
                print!(" {:.3}\n", v[i].as_ref().unwrap());
            }
            println!(" {}:  {:?}\n","Dtype".red().bold(), c.datatype);
        },ColumnType::FloatVec(v) => {
            printer::print_names(&vec![c.name.clone()]);
            let t:Vec<&str> = vec!["MEAN", "STD", "MIN", "25%", "50%", "75%", "MAX"];
            for i in 3..v.len() {
                print!(" ");
                printer::print_clr_cell(&t[i-3].to_string());
                print!(" {:.3}\n", v[i].as_ref().unwrap());
            }
            println!(" {}:  {:?}\n","Dtype".red().bold(), c.datatype);
        }, _ => {}
    }
}

fn print_count(map:&HashMap<String, usize>, name:String) {
    let names:Vec<String> = vec![name];
    printer::print_names(&names);

    for (k, v) in map {
        printer::print_cell(k);
        printer::print_cell(&v.to_string());
        println!();
    }
    printer::print_dash(&names);
    println!();
}