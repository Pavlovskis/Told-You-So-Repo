use std::{collections::HashMap, ops};

use chrono::NaiveDate;

use crate::{helpe::{self, printer}, Frame};

#[derive(Debug, Clone)]
pub struct Column {
    pub pos:usize,
    pub name:String,
    pub datatype:DataType,
    pub values:ColumnType
}

pub struct ColumnBduilder {
    pub pos:Option<usize>,
    pub name:Option<String>,
    pub datatype:DataType,
    pub values:Option<ColumnType>
}

pub struct Positions( Vec<usize> );

impl ops::Deref for Positions {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl ops::DerefMut for Positions {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    StringVec(Vec<Option<String>>),
    IntVec(Vec<Option<i32>>),
    FloatVec(Vec<Option<f64>>),
    BoolVec(Vec<Option<bool>>),
    // DateVec(Vec<Option<NaiveDate>>),
    // ...
    // Index(Vec<usize>),
    Empty()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    // NaiveDate,
    // ...
    None,
}

impl Column {

    pub fn describe(&self) -> Column {
        let new_col = ColumnBuilder
        match &self.values {
            ColumnType::IntVec(v) => { 
                let new_col = ColumnBduilder::new().pos(self.pos.clone()).name(&self.name.clone()).datatype(DataType::Float)
                    .values(ColumnType::FloatVec(Self::desc_num(v)))
                .build();
                new_col
            },
            ColumnType::FloatVec(v) => { 
                let new_col = ColumnBduilder::new().pos(self.pos.clone()).name(&self.name.clone()).datatype(DataType::Float)
                    .values(ColumnType::FloatVec(Self::desc_num(v)))
                .build();
                new_col
            },
            ColumnType::StringVec(v) => { 
                let new_col = ColumnBduilder::new().pos(self.pos.clone()).name(&self.name.clone()).datatype(DataType::String)
                    .values(ColumnType::StringVec(Self::desc_str(v)))
                .build();
                new_col
            },
            _ => ColumnBduilder::empty(),
        }

    }

    fn desc_num<T> (v:&Vec<Option<T>>) -> Vec<Option<f64>>
        where T:Into<f64> + Copy
    {
        fn round_dec(value: f64) -> f64 {
            (value * 1000.0).round() / 1000.0
        }

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
    
}

impl Positions {
    pub fn new(pos:Vec<usize>) -> Self { Positions(pos) }

    pub fn replace(&self) -> Frame {
        todo!()
    }
}

impl ColumnBduilder {

    pub fn empty() -> Column {
        Column {
            pos: 0,
            name: String::from(""),
            datatype:DataType::None,
            values: ColumnType::Empty()
        }
    }
    pub fn new() -> Self {
        ColumnBduilder { 
            pos: None,
            name: None, 
            datatype: DataType::None, 
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
    pub fn values(mut self, values:ColumnType) -> Self {
        self.values = Some(values);
        self
    }
    pub fn build(self) -> Column {
        Column {
            pos: self.pos.unwrap(),
            name: self.name.unwrap(),
            datatype: self.datatype, 
            values: self.values.unwrap()
        }
    }

}

fn print_desc(c:&Column) {
    match &c.values {
        ColumnType::StringVec(v) => {
            printer::print_names(&vec![c.name.clone()]);
            println!(" UNIQUE:  {}\n TOP:  {}\n FREQ:  {}", v[0].as_ref().unwrap(), v[1].as_ref().unwrap(), v[2].as_ref().unwrap());
            println!("Dtype:  {:?}\n", c.datatype);
        },ColumnType::FloatVec(v) => {
            printer::print_names(&vec![c.name.clone()]);
            let t:Vec<&str> = vec!["MEAN", "STD", "MIN", "25%", "50%", "75%", "MAX"];
            for i in 0..v.len() {
                println!(" {}: {:.3}", t[i], v[i].as_ref().unwrap());
            }
            println!("Dtype:  {:?}\n", c.datatype);
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

