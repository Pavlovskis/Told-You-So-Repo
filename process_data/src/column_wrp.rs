
use std::{ops, collections::HashMap};

use crate::{column::{Column, DataType, ColumnType, Positions}, helpe::{self, printer}, Frame, FrameBuilder};

//Wrapper for Vec<Column> so you can pass just vecto and don't bother creating frame
#[derive(Debug, Clone)]
pub struct Data {
    positions:Vec<usize>,
    data:Vec<Column>
}

pub struct DataBuilder {
    positions:Option<Vec<usize>>,
    data:Option<Vec<Column>>
}

impl ops::Deref for Data {
    type Target = Vec<Column>;
    fn deref(&self) -> &Self::Target { &self.data }
}
impl ops::DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.data }
}

trait WrapperTrait {
    fn wrap(&self);
}

impl WrapperTrait for Data {
    fn wrap(&self) {
        print!("implementation for wrapper");
    }
}

pub trait ConvertibleVal {
    fn cvrt_to_i32(&self) -> Option<i32>;
    fn cvrt_to_f64(&self) -> Option<f64>;
    fn cvrt_to_str(&self) -> Option<&str>;
}
impl ConvertibleVal for i32 {
    fn cvrt_to_i32(&self) -> Option<i32> { Some(*self) }
    fn cvrt_to_f64(&self) -> Option<f64> { Some(*self as f64) }
    fn cvrt_to_str(&self) -> Option<&str> { None }
}
impl ConvertibleVal for f64 {
    fn cvrt_to_i32(&self) -> Option<i32> { Some(*self as i32) }
    fn cvrt_to_f64(&self) -> Option<f64> { Some(*self) }
    fn cvrt_to_str(&self) -> Option<&str> { None }
}
impl ConvertibleVal for &str {
    fn cvrt_to_i32(&self) -> Option<i32> { self.parse().ok() }
    fn cvrt_to_f64(&self) -> Option<f64> { self.parse().ok() }
    fn cvrt_to_str(&self) -> Option<&str> { Some(*self) }
}

impl Data {

    pub fn describe(&self) -> Vec<Column> {
        let mut res:Vec<Column> = Vec::with_capacity(self.data.len());
        for col in &self.data {
            let desc:Column = Column::describe(col);
            res.push(desc);
        }

        let dt:&DataType = &res[0].datatype;
        let mut is_same:bool = true;
        for col in &res {
            if dt != &col.datatype { is_same = false; break; }
        }

        print_desc(&res, is_same);

        res
    } 

    pub fn count_values(&self) -> Vec<HashMap<String, usize>>{
        let mut maps:Vec<HashMap<String, usize>> = Vec::with_capacity(self.data.len());
        for col in &self.data {
            let map = Column::count_values(col);
            maps.push(map);
        }
        maps
    }

    pub fn eq<T:ConvertibleVal>(&self, val:T) -> Vec<usize> {
        let len = Self::get_len(&self);

        #[allow(unused_assignments)]
        let mut pos:Vec<usize> = Vec::with_capacity(len);
        match &self.data[0].values {
            ColumnType::StringVec(v) => { 
                let new = match val.cvrt_to_str() {
                    Some(x) => x,
                    None => "" };
                pos = v.iter().enumerate()
                    .filter_map(|(i, option)| option.as_deref().filter(|s| s == &new).map(|_| i))
                .collect()
            },ColumnType::IntVec(v) => { 
                pos = Self::get_positions(v, "eq", Self::get_i32(val));
            },ColumnType::FloatVec(v) => { 
                pos = Self::get_positions(v, "eq", Self::get_f64(val));
            }, _ => { return Vec::new(); }
        }
        pos
    }

    pub fn lt<T:ConvertibleVal>(&self, val:T) -> Vec<usize> {
        if helpe::type_of(&val) == "&str" {
            println!("  Can't perform Less Operation on String");
            return Vec::new();
        }
        let len = Self::get_len(&self);

        #[allow(unused_assignments)]
        let mut pos:Vec<usize> = Vec::with_capacity(len);
        match &self.data[0].values {
            ColumnType::StringVec(_) => { 
                println!("  Can't perform Less Operation on String"); // red
                return Vec::new();
            },ColumnType::IntVec(v) => { 
                pos = Self::get_positions(v, "lt", Self::get_i32(val));
            },ColumnType::FloatVec(v) => { 
                pos = Self::get_positions(v, "lt", Self::get_f64(val));
            }, _ => { return Vec::new(); }
        }
        pos
    } 

    pub fn mt<T:ConvertibleVal>(&self, val:T) -> Vec<usize> {
        if helpe::type_of(&val) == "&str" {
            println!("  Can't perform More Operation on String");
            return Vec::new();
        }
        let len = Self::get_len(&self);

        #[allow(unused_assignments)]
        let mut pos:Vec<usize> = Vec::with_capacity(len);
        match &self.data[0].values {
            ColumnType::StringVec(_) => { 
                println!("  Can't perform More Operation on String"); // red
                return Vec::new();
            },ColumnType::IntVec(v) => { 
                pos = Self::get_positions(v, "mt", Self::get_i32(val));
            },ColumnType::FloatVec(v) => { 
                pos = Self::get_positions(v, "mt", Self::get_f64(val));
            }, _ => { return Vec::new(); }
        }
        pos
    }

    pub fn mte<T:ConvertibleVal>(&self, val:T) -> Vec<usize> {
        if helpe::type_of(&val) == "&str" {
            println!("  Can't perform More Operation on String");
            return Vec::new();
        }
        let len = Self::get_len(&self);

        #[allow(unused_assignments)]
        let mut pos:Vec<usize> = Vec::with_capacity(len);
        match &self.data[0].values {
            ColumnType::StringVec(_) => { 
                println!("  Can't perform More Operation on String"); // red
                return Vec::new();
            },ColumnType::IntVec(v) => { 
                pos = Self::get_positions(v, "mte", Self::get_i32(val));
            },ColumnType::FloatVec(v) => { 
                pos = Self::get_positions(v, "mte", Self::get_f64(val));
            }, _ => { return Vec::new(); }
        }
        pos
    }

    pub fn lte<T:ConvertibleVal>(&self, val:T) -> Vec<usize> {
        if helpe::type_of(&val) == "&str" {
            println!("  Can't perform More Operation on String");
            return Vec::new();
        }
        let len = Self::get_len(&self);
        
        #[allow(unused_assignments)]
        let mut pos:Vec<usize> = Vec::with_capacity(len);
        match &self.data[0].values {
            ColumnType::StringVec(_) => { 
                println!("  Can't perform More Operation on String"); // red
                return Vec::new();
            },ColumnType::IntVec(v) => { 
                pos = Self::get_positions(v, "lte", Self::get_i32(val));
            },ColumnType::FloatVec(v) => { 
                pos = Self::get_positions(v, "lte", Self::get_f64(val));
            }, _ => { return Vec::new(); }
        }
        pos
    }

    pub fn replace<T:ConvertibleVal>(&self, val:T) {
        
    }

    fn get_positions<T>(v:&Vec<Option<T>>, cmp:&str, new:T) -> Vec<usize> 
        where T:PartialOrd
    {
        let mut pos:Vec<usize> = Vec::with_capacity(v.len());
        if cmp == "eq" {
            pos = v.iter().enumerate()
                .filter_map(|(i, option)| option.as_ref().filter(|f| f == &&new).map(|_| i))
            .collect()
        }else if cmp == "mt" {
            pos = v.iter().enumerate()
                .filter_map(|(i, option)| option.as_ref().filter(|x| x > &&new).map(|_| i))
            .collect()
        }else if cmp == "lt" {
            pos = v.iter().enumerate()
                .filter_map(|(i, option)| option.as_ref().filter(|x| x < &&new).map(|_| i))
            .collect()
        }else if cmp == "mte" {
            pos = v.iter().enumerate()
                .filter_map(|(i, option)| option.as_ref().filter(|x| x >= &&new).map(|_| i))
            .collect()
        }else if cmp == "lte" {
            pos = v.iter().enumerate()
                .filter_map(|(i, option)| option.as_ref().filter(|x| x <= &&new).map(|_| i))
            .collect()
        }
        pos
    }

    fn get_len(d:&Data) -> usize { 
        #[allow(unused_assignments)]
        let mut len:usize = 0;
        //col returns always just one column wrapped in the data
        match &d.data[0].values {
            ColumnType::StringVec(v) => len = v.len(),
            ColumnType::IntVec(v) => len = v.len(),
            ColumnType::FloatVec(v) => len = v.len(),
            ColumnType::BoolVec(v) => len = v.len(),
            ColumnType::Empty() => len = 0,
        }
        len
    }
    fn get_i32<T:ConvertibleVal>(val:T) -> i32{
        match val.cvrt_to_i32() {
            Some(x) => return x,
            None => return 0 }; 
    }
    fn get_f64<T:ConvertibleVal>(val:T) -> f64 {
        match val.cvrt_to_f64() {
            Some(x) => return x,
            None => return 0.0 }; 
    }

    pub fn print_frame(self) -> Frame {
        let fr = Frame::print_frame(
            FrameBuilder::new().size(self.data.len(), Self::get_len(&self)).data(self.data)
        .build());
        fr
    }

}

fn print_desc(cols:&Vec<Column>, same:bool) {
    let a:Vec<&str> = vec!["UNIQ", "TOP", "FREQ", "MEAN", "STD", "MIN", "25%", "50%", "75%", "MAX"];
    let names:Vec<String> = cols.iter().map(|n| n.name.clone()).collect();

    let mut s:usize = 0;
    let mut e:usize = a.len();
    if same {
        if cols[0].datatype == DataType::String { e = 3; }
        else { s = 3; e = a.len()}
    }

    printer::print_names(&names);
    for i in s..e {
        printer::print_cell(&a[i].to_string());
        for col in cols {
            match &col.values {
                ColumnType::StringVec(v) => {
                    if let Some(val) = &v[i] {
                        printer::print_cell(val);
                    }else { printer::print_none(); }
                },
                ColumnType::FloatVec(v) => {
                    if let Some(val) = &v[i] {
                        printer::print_cell(&val.to_string());
                    }else { printer::print_none(); }
                },
                _ => print!("[]")
            }
        }
        println!("\n");
    }
    printer::print_dash(&names);
}

impl DataBuilder {
    pub fn empty() -> Data { 
        Data{ positions:Vec::new(), data:Vec::new() } 
    }

    pub fn new(p:Vec<usize>, v:Vec<Column>) -> Data { 
        Data { positions: p, data: v }
    }
}