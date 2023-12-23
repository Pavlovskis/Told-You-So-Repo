use std::{
    io::{self, BufReader, prelude::*, empty}, 
    fs::File, collections::HashMap, mem,
};
use chrono::NaiveDate;
use colored::Colorize;

mod column;
mod helpe;
mod conversion;

use column::{Column, ColumnType, DataType, ColumnBuilder, ConvertibleVal};
use helpe::*;
use conversion::FromCSV;

#[derive(Debug, Clone)]
pub struct Frame {
    pub size:(usize, usize),
    pub data:Vec<Column>, 
    pub col_indexed:Vec<usize>

}

pub struct FrameBuilder {
    pub size: Option<(usize, usize)>,
    pub data: Option<Vec<Column>>,    
    pub col_indexed:Option<Vec<usize>>

}

//take any kind of vector for the column positions
//range[..1], [1..], vec![1,3,5], vec!["col_name",(...)]
pub trait ColIndex {
    fn get_positions(self, fr:&Frame) -> Vec<usize>;
}
impl ColIndex for [std::ops::Range<usize>;1] {
    fn get_positions(self, _:&Frame) -> Vec<usize> {
        (self[0].start..=self[0].end).collect()    }
}
impl ColIndex for [std::ops::RangeFrom<usize>;1] {
    fn get_positions(self, fr:&Frame) -> Vec<usize> {
        if self[0].start > fr.size.0 {
            (self[0].start..=fr.size.1-1).collect()
        }else { (self[0].start..=fr.size.0-1).collect() }
    }
}
impl ColIndex for [std::ops::RangeTo<usize>;1] {
    fn get_positions(self, _:&Frame) -> Vec<usize> {
        (0..=self[0].end).collect()    }
}
impl ColIndex for Vec<&str> {
    fn get_positions(self, fr:&Frame) -> Vec<usize> {
        let names:Vec<&str> = self;
        let mut pos:Vec<usize> = Vec::with_capacity(names.len());
        for name in names {
            for i in 0..fr.size.0 {
                if name == fr.data[i].name { 
                    pos.push(i);
                }
            }
        }
        pos.sort();
        pos
    }
}
impl ColIndex for Vec<usize> {
    fn get_positions(self, _:&Frame) -> Vec<usize> {
        self.into_iter().collect()    }
}
impl ColIndex for &str {
    fn get_positions(self, fr:&Frame) -> Vec<usize> {
        let mut c:usize = 0;
        for i in &fr.data {
            if i.name == self { return vec![c]; }
            c += 1;
        }
        Vec::new()
    }
}

impl Frame {

    pub fn head(&self, lines:Option<usize>) -> Frame {
        if lines > Some(self.size.1) { 
            println!("Range too big for Frame");
            return FrameBuilder::empty(); }
        let mut n:usize = 10;
        if let Some(l) = lines { n = l }

        Self::collect_values(&self, &(0..=n-1).collect(), &mut(0..=self.size.0-1).collect())
    }

    pub fn tail(&self, lines:Option<usize>) -> Frame {
        if lines > Some(self.size.1) { 
            println!("Range too big for Frame");
            return FrameBuilder::empty(); }
        let mut n:usize = 10;
        if let Some(l) = lines { n = l }

        Self::collect_values(&self, &(self.size.1-n..=self.size.1-1).collect(), &mut(0..=self.size.0-1).collect())
    }

    //for line range
    pub fn range<T:ColIndex>(&self, p:T) -> Frame {
        let line_pos:Vec<usize> = p.get_positions(&self);

        Self::collect_values(&self, &line_pos, &(0..=self.size.0-1).collect())
    }   

    //for column span
    pub fn span<T:ColIndex>(&self, p:T) -> Frame {
        let col_pos:Vec<usize> = p.get_positions(&self);

        Self::collect_values(&self, &(0..=self.size.1-1).collect(), &col_pos)
    }

    pub fn col<T:ColIndex>(&self, p:T) -> Frame {
        let mut cols:Vec<Column> = Vec::with_capacity(self.size.0);

        let mut ps:Vec<usize> = Vec::with_capacity(self.size.0);
        for pos in p.get_positions(&self) {
            ps.push(pos);
            cols.push(self.data[pos].clone());
        }

        if cols.is_empty() { 
            println!("Couldn't find column(s)"); // red
            return FrameBuilder::empty();
        }
        if cols.len() == 1 {
            return Frame { size:(1, self.size.1),
                data: vec![ ColumnBuilder::new()
                    .pos(ps[0])
                    .name(cols[0].name.clone())
                    .datatype(cols[0].datatype.clone())
                    .op_pos(Some(vec![ps[0]]))
                    .values(cols[0].values.clone())
                .build() ],
                col_indexed: vec![ps[0]]
            };
        }
        FrameBuilder::new().size(cols.len(), self.size.1).data(cols).col_indexed(ps).build()
    }

    pub fn describe(&self) {
        let mut cols:Vec<Column> = Vec::with_capacity(self.size.0);
        let mut same:bool = true;
        let dt:&DataType = &self.data[0].datatype;
        for col in &self.data {
            if &col.datatype != dt { same = false; }
            let col_desc = Column::describe(&col);
            cols.push(col_desc);
        }
        print_desc(&Frame { size: (cols.len(), 10), data: cols, col_indexed:Vec::new() }, same);
    }

    pub fn count_values(&self) -> Vec<HashMap<String, usize>> {
        let mut maps:Vec<HashMap<String, usize>> = Vec::with_capacity(self.data.len());
        for col in &self.data {
            let map = Column::count_values(col);
            maps.push(map);
        }
        maps
    }

    //return a suframe;
    //col:Frame is suppose to be just one column
    pub fn loc(&self, fr:Frame) -> Frame {
        fn collect_values<T>(v:&Vec<Option<T>>, pos:&Vec<usize>, mut raw_vals:Vec<Option<T>>) -> Vec<Option<T>> 
            where T: Clone 
        { 
            for p in pos {
                raw_vals.push(v[*p].clone());
            }
            raw_vals
        }
        //returns empty frame
        if fr.col_indexed.is_empty() { return FrameBuilder::empty(); }
        let positions = match fr.data[0].op_pos.clone() {
            Some(x) => x,
            None => Vec::new()
        };

        let mut cols:Vec<Column> = Vec::with_capacity(self.size.0 + 1);
        for col in &self.data {
            match &col.values {
                ColumnType::StringVec(v) => {
                    let mut raw_vals:Vec<Option<String>> = Vec::with_capacity(positions.len() + 1);
                    raw_vals = collect_values(v, &positions, raw_vals);
                    let col:Column = ColumnBuilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).op_pos(None).values(ColumnType::StringVec(raw_vals)).build();
                    cols.push(col);
                },ColumnType::IntVec(v) => {
                    let mut raw_vals:Vec<Option<i32>> = Vec::with_capacity(positions.len() + 1);
                    raw_vals = collect_values(v, &positions, raw_vals);
                    let col:Column = ColumnBuilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).op_pos(None).values(ColumnType::IntVec(raw_vals)).build();
                    cols.push(col);
                },ColumnType::FloatVec(v) => {
                    let mut raw_vals:Vec<Option<f64>> = Vec::with_capacity(positions.len() + 1);
                    raw_vals = collect_values(v, &positions, raw_vals);
                    let col:Column = ColumnBuilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).op_pos(None).values(ColumnType::FloatVec(raw_vals)).build();
                    cols.push(col);
                },ColumnType::BoolVec(v) => {
                    let mut raw_vals:Vec<Option<bool>> = Vec::with_capacity(positions.len() + 1);
                    raw_vals = collect_values(v, &positions, raw_vals);
                    let col:Column = ColumnBuilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).op_pos(None).values(ColumnType::BoolVec(raw_vals)).build();
                    cols.push(col);
                },ColumnType::DateVec(v) => {
                    let mut raw_vals:Vec<Option<NaiveDate>> = Vec::with_capacity(positions.len() + 1);
                    raw_vals = collect_values(v, &positions, raw_vals);
                    let col:Column = ColumnBuilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).op_pos(None).values(ColumnType::DateVec(raw_vals)).build();
                    cols.push(col);
                },ColumnType::Empty() => cols.push( ColumnBuilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).values(ColumnType::Empty()).build() ),
            }
        }
        FrameBuilder::new().size(cols.len(), positions.len()).data(cols).col_indexed(vec![self.data[0].pos]).build()
    }

    //locates only the column
    pub fn find(&mut self, df:Frame) -> Self {
        if !self.col_indexed.is_empty() { self.col_indexed.clear() }

        for i in 0..df.data.len() {
            self.col_indexed.push(df.data[i].pos);
        }
        
        self.clone()
    }

    //pops the column out of dataframe and returns it
    pub fn pop<T:ColIndex>(&mut self, col:T) -> Column {
        let pos:Vec<usize> = col.get_positions(&self);
        
        self.size.0 -= 1;
        self.data.remove(pos[0])
    }

    pub fn is_na(&self) -> HashMap<String, usize> {
        fn count_na<T>(v:&Vec<Option<T>>) -> usize {
            let mut n:usize = 0;
            for opt in v {
                if let Some(_) = opt {
                    continue;
                }else { n += 1; }
            }
            n
        }

        let mut map:HashMap<String, usize> = HashMap::with_capacity(self.size.0);
        for col in &self.data {
            #[allow(unused_assignments)]
            let mut na:usize = 0;
            match &col.values {
                ColumnType::StringVec(v) => {
                    na = count_na(v);
                },ColumnType::IntVec(v) => {
                    na = count_na(v);
                },ColumnType::FloatVec(v) => {
                    na = count_na(v);
                },ColumnType::BoolVec(v) => {
                    na = count_na(v);
                },ColumnType::DateVec(v) => {
                    na = count_na(v);
                },ColumnType::Empty() => na = 0,
            }
            map.insert(col.name.clone(), na);
        }
        print_isna(&self, &map);
        map
    }

    pub fn drop_na(&self, val:Option<&str>) {
        todo!()
    }

    pub fn replace<T:ConvertibleVal>(mut self, val:T) -> Self
        where T:Copy 
    {
        fn replace_values<T>(v:&mut Vec<Option<T>>, op_pos:Option<Vec<usize>>, val:T) -> &mut Vec<Option<T>>
            where T: Copy{
            if let Some(pos) = &op_pos {
                for i in pos {
                    v[*i] = Some(val);
                }
            }
            v
        }

        for p in self.col_indexed.clone() {
            match self.data[p].clone() { mut c => {
                match c.values {
                    ColumnType::StringVec(mut v) => {
                        if let Some(pos) = &c.op_pos {
                            for i in pos {
                                v[*i] = Some(val.cvrt_to_str().unwrap().to_string());
                            }
                        }
                        let new = v;
                        c.values = ColumnType::StringVec(new);
                    },
                    ColumnType::IntVec(mut v) => {
                        let new = replace_values(&mut v, c.op_pos, val.cvrt_to_i32().expect("Funky Input"));
                        c.values = ColumnType::IntVec(new.to_vec());
                    },
                    ColumnType::FloatVec(mut v) => {
                        let new = replace_values(&mut v, c.op_pos, val.cvrt_to_f64().expect("Funky Input"));
                        c.values = ColumnType::FloatVec(new.to_vec());
                    },
                    ColumnType::BoolVec(mut v) => {
                        let new = replace_values(&mut v, c.op_pos, val.cvrt_to_bool().expect("Funky Input"));
                        c.values = ColumnType::BoolVec(new.to_vec());
                    },
                    ColumnType::DateVec(mut v) => {
                        let new = replace_values(&mut v, c.op_pos, val.cvrt_to_date().expect("Funky Input"));
                        c.values = ColumnType::DateVec(new.to_vec());
                    },
                    ColumnType::Empty() => todo!(),
                }
                c.op_pos = None;
                self.data[p] = c;
            } }
        }

        self
    }

    pub fn add<T:ConvertibleVal>(mut self, val:T) -> Self {
        for i in self.col_indexed.clone() {
            match &self.data[i].values {
                ColumnType::IntVec(v) => {
                    let new = v.iter().map(|opt| 
                        opt.map(|x| x + val.cvrt_to_i32().unwrap())
                    ).collect::<Vec<Option<i32>>>();
                    self.data[i].values = ColumnType::IntVec(new);
                },ColumnType::FloatVec(v) => {
                    let new = v.iter().map(|opt| 
                        opt.map(|x| x + val.cvrt_to_f64().unwrap())
                    ).collect::<Vec<Option<f64>>>();
                    self.data[i].values = ColumnType::FloatVec(new);
                },ColumnType::DateVec(v) => {
                    for mut opt in v {
                        if let Some(x) = opt {
                            opt = &Date::add(Date(*x), val.cvrt_to_i32().unwrap());
                        }
                    }
                    self.data[i].values = ColumnType::DateVec(v.to_vec());
                },
                _ => println!("{}","Can't perform Add() Operation on this column".red().bold()),
            }
        }
        self
    }
    pub fn sub<T:ConvertibleVal>(mut self, val:T) -> Self {
        for i in self.col_indexed.clone() {
            match &self.data[i].values {
                ColumnType::IntVec(v) => {
                    let new = v.iter().map(|opt| 
                        opt.map(|x| x - val.cvrt_to_i32().unwrap())
                    ).collect::<Vec<Option<i32>>>();
                    self.data[i].values = ColumnType::IntVec(new);
                },ColumnType::FloatVec(v) => {
                    let new = v.iter().map(|opt| 
                        opt.map(|x| x - val.cvrt_to_f64().unwrap() )
                    ).collect::<Vec<Option<f64>>>();
                    self.data[i].values = ColumnType::FloatVec(new);
                },ColumnType::DateVec(v) => {
                    let mut new_date:Vec<Option<NaiveDate>> = Vec::with_capacity(v.len());
                    for opt in v {
                        if let Some(date) = opt {
                            new_date.push(Date::sub(Date(*date), val.cvrt_to_i32().unwrap()));
                        }
                    }
                    self.data[i].values = ColumnType::DateVec(new_date);
                },
                _ => println!("{}","Can't perform Subtract() Operation on this column".red().bold()),
            }
        }
        self
    }
    pub fn div<T:ConvertibleVal>(mut self, val:T) -> Self {
        for i in self.col_indexed.clone() {
            match &self.data[i].values {
                ColumnType::IntVec(v) => {
                    let new = v.iter().map(|opt| 
                        opt.map(|x| x as f64 / val.cvrt_to_i32().unwrap() as f64)
                    ).collect::<Vec<Option<f64>>>();
                    self.data[i].values = ColumnType::FloatVec(new);
                },ColumnType::FloatVec(v) => {
                    let new = v.iter().map(|opt| 
                        opt.map(|x| x - val.cvrt_to_f64().unwrap() )
                    ).collect::<Vec<Option<f64>>>();
                    self.data[i].values = ColumnType::FloatVec(new);
                },ColumnType::DateVec(v) => {
                    let mut new_date:Vec<Option<NaiveDate>> = Vec::with_capacity(v.len());
                    for opt in v {
                        if let Some(date) = opt {
                            new_date.push(Date::div(Date(*date), val.cvrt_to_i32().unwrap()));
                        }
                    }
                    self.data[i].values = ColumnType::DateVec(new_date);
                },
                _ => println!("{}","Can't perform Subtract() Operation on this column".red().bold()),
            }
        }
        self
    }
    pub fn mul<T:ConvertibleVal>(mut self, val:T) -> Self {
        for i in self.col_indexed.clone() {
            match &self.data[i].values {
                ColumnType::IntVec(v) => {
                    let new = v.iter().map(|opt| 
                        opt.map(|x| x * val.cvrt_to_i32().unwrap())
                    ).collect::<Vec<Option<i32>>>();
                    self.data[i].values = ColumnType::IntVec(new);
                },ColumnType::FloatVec(v) => {
                    let new = v.iter().map(|opt| 
                        opt.map(|x| x * val.cvrt_to_f64().unwrap() )
                    ).collect::<Vec<Option<f64>>>();
                    self.data[i].values = ColumnType::FloatVec(new);
                },ColumnType::DateVec(v) => {
                    let mut new_date:Vec<Option<NaiveDate>> = Vec::with_capacity(v.len());
                    for opt in v {
                        if let Some(date) = opt {
                            new_date.push(Date::mul(Date(*date), val.cvrt_to_i32().unwrap()));
                        }
                    }
                    self.data[i].values = ColumnType::DateVec(new_date);
                },
                _ => println!("{}","Can't perform Subtract() Operation on this column".red().bold()),
            }
        }
        self
    }
   
    //check if size is correct
    pub fn info(&self) {
        pub fn get_mem<T>(v:&Vec<Option<T>>) -> usize{
            let vec_size:usize = mem::size_of_val(v);
            let str_size:usize = v.iter().map(|x| 
                mem::size_of_val(x) 
            ).sum();
            vec_size + str_size
        }
        let col_names:Vec<String> = vec![String::from("Column"), String::from("Null-Count"), String::from("Dtype")];

        print!(" #  ");
        for name in &col_names {
            Self::print_clr_cell(name);
        }
        println!();
        print!("{}", "--- ".blue().bold());
        for name in &col_names {
            let len:usize = name.len();
            let mut s:String = String::with_capacity(len);
            for _ in 0..len {
                s.push('-');
            }
            Self::print_clr_cell(&s);
        }println!();

        let mut dtypes:HashMap<DataType, usize> = HashMap::with_capacity(self.size.0+1);
        let mut total:usize = 0;

        let mut i:usize = 0;
        for col in &self.data {
            print!("[{}]  ", i);
            Self::print_cell(&col.name);
            print!("({}) null-vals \t {:?}\n", 0, &col.datatype);

            match &col.values {
                ColumnType::StringVec(v) => { total += get_mem(v) },
                ColumnType::IntVec(v) => { total += get_mem(v) },
                ColumnType::FloatVec(v) => { total += get_mem(v) },
                ColumnType::BoolVec(v) => { total += get_mem(v) },
                ColumnType::DateVec(v) => { total += get_mem(v) },
                ColumnType::Empty() => { total += 0 },
            }

            if dtypes.contains_key(&col.datatype) {
                dtypes.entry(col.datatype.clone()).and_modify(|x| *x += 1);
            }else { dtypes.insert(col.datatype.clone(), 1); }

            i += 1;
        }
        print!("{}", "--- ".blue().bold());
        for name in col_names {
            let len:usize = name.len();
            let mut s:String = String::with_capacity(len);
            for _ in 0..len {
                s.push('-');
            }
            Self::print_clr_cell(&s);
        }
        println!();

        print!(" {}:","Dtypes".red().bold());
        for (k, v) in dtypes {
            print!("{:?}({})  ",k, v);
        }
        println!("\n {}: {:.1} kB","DataUsage".red().bold(), total as f64 / 1000.0 );
        
    }
    
    //comparison operators only for columns//////////////////////////////////////
    pub fn eq<T:ConvertibleVal>(self, val:T) -> Frame {
        let col = self.data[0].clone();
        let len:usize = Self::get_len(&col);

        #[allow(unused_assignments)]
        let mut pos:Vec<usize> = Vec::with_capacity(len);
        match &col.values {
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
            },ColumnType::DateVec(v) => {
                pos = Self::get_positions(v, "eq", Self::get_date(val));
            }, _ => { return FrameBuilder::empty(); }
        }
        Frame { size: (self.size.0, self.size.1),
            data: vec![ ColumnBuilder::new().
                pos(col.pos).name(col.name).datatype(col.datatype).op_pos(Some(pos)).values(col.values)
            .build() ],
            col_indexed: vec![col.pos]
        }
    }   
    pub fn lt<T:ConvertibleVal>(self, val:T) -> Frame {
        let col:Column = self.data[0].clone();
        if helpe::type_of(&val) == "&str" {
            println!("  Can't perform Less Operation on String");
            return FrameBuilder::empty();
        }
        let len = Self::get_len(&col);

        #[allow(unused_assignments)]
        let mut pos:Vec<usize> = Vec::with_capacity(len);
        match &col.values {
            ColumnType::StringVec(_) => { 
                println!("  Can't perform Less Operation on String"); // red
                return FrameBuilder::empty();
            },ColumnType::IntVec(v) => { 
                pos = Self::get_positions(v, "lt", Self::get_i32(val));
            },ColumnType::FloatVec(v) => { 
                pos = Self::get_positions(v, "lt", Self::get_f64(val));
            },ColumnType::DateVec(v) => {
                pos = Self::get_positions(v, "lt", Self::get_date(val));
            },_ => { return FrameBuilder::empty(); }
        }
        Frame { size: (self.size.0, self.size.1),
            data: vec![ ColumnBuilder::new()
                .pos(col.pos).name(col.name).datatype(col.datatype).op_pos(Some(pos)).values(col.values)
            .build()],
            col_indexed: vec![col.pos]
        }
    } 
    pub fn mt<T:ConvertibleVal>(self, val:T) -> Frame {
        let col:Column = self.data[0].clone();
        if helpe::type_of(&val) == "&str" {
            println!("  Can't perform More Operation on String");
            return FrameBuilder::empty();
        }
        let len = Self::get_len(&col);

        #[allow(unused_assignments)]
        let mut pos:Vec<usize> = Vec::with_capacity(len);
        match &col.values {
            ColumnType::StringVec(_) => { 
                println!("  Can't perform More Operation on String"); // red
                return FrameBuilder::empty();
            },ColumnType::IntVec(v) => { 
                pos = Self::get_positions(v, "mt", Self::get_i32(val));
            },ColumnType::FloatVec(v) => { 
                pos = Self::get_positions(v, "mt", Self::get_f64(val));
            },ColumnType::DateVec(v) => {
                pos = Self::get_positions(v, "mt", Self::get_date(val));
            }, _ => { return FrameBuilder::empty(); }
        }
        Frame { size: (self.size.0, self.size.1),
            data: vec![ ColumnBuilder::new()
                .pos(col.pos).name(col.name).datatype(col.datatype).op_pos(Some(pos)).values(col.values)
            .build()],
            col_indexed: vec![col.pos]
        }    
    }
    pub fn lte<T:ConvertibleVal>(self, val:T) -> Frame {
        let col:Column = self.data[0].clone();
        if helpe::type_of(&val) == "&str" {
            println!("  Can't perform More Operation on String");
            return FrameBuilder::empty();
        }
        let len = Self::get_len(&col);
        
        #[allow(unused_assignments)]
        let mut pos:Vec<usize> = Vec::with_capacity(len);
        match &col.values {
            ColumnType::StringVec(_) => { 
                println!("  Can't perform More Operation on String"); // red
                return FrameBuilder::empty();
            },ColumnType::IntVec(v) => { 
                pos = Self::get_positions(v, "lte", Self::get_i32(val));
            },ColumnType::FloatVec(v) => { 
                pos = Self::get_positions(v, "lte", Self::get_f64(val));
            },ColumnType::DateVec(v) => {
                pos = Self::get_positions(v, "lte", Self::get_date(val));
            }, _ => { FrameBuilder::empty(); }
        }
        Frame { size: (self.size.0, self.size.1),
            data: vec![ ColumnBuilder::new()
                .pos(col.pos).name(col.name).datatype(col.datatype).op_pos(Some(pos)).values(col.values)
            .build()],
            col_indexed: vec![col.pos]
        }
    }
    pub fn mte<T:ConvertibleVal>(self, val:T) -> Frame {
        let col:Column = self.data[0].clone();
        if helpe::type_of(&val) == "&str" {
            println!("  Can't perform More Operation on String");
            return FrameBuilder::empty();
        }
        let len = Self::get_len(&col);

        #[allow(unused_assignments)]
        let mut pos:Vec<usize> = Vec::with_capacity(len);
        match &col.values {
            ColumnType::StringVec(_) => { 
                println!("  Can't perform More Operation on String"); // red
                return FrameBuilder::empty();
            },ColumnType::IntVec(v) => { 
                pos = Self::get_positions(v, "mte", Self::get_i32(val));
            },ColumnType::FloatVec(v) => { 
                pos = Self::get_positions(v, "mte", Self::get_f64(val));
            },ColumnType::DateVec(v) => {
                pos = Self::get_positions(v, "mte", Self::get_date(val));
            }, _ => { return FrameBuilder::empty(); }
        }
        Frame { size: (self.size.0, self.size.1),
            data: vec![ ColumnBuilder::new()
                .pos(col.pos).name(col.name).datatype(col.datatype).op_pos(Some(pos)).values(col.values)
            .build()],
            col_indexed: vec![col.pos]
        }
    }

    //gets the position that need to be altered
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
    fn get_len(c:&Column) -> usize { 
        #[allow(unused_assignments)]
        let mut len:usize = 0;
        //col returns always just one column wrapped in the data
        match &c.values {
            ColumnType::StringVec(v) => len = v.len(),
            ColumnType::IntVec(v) => len = v.len(),
            ColumnType::FloatVec(v) => len = v.len(),
            ColumnType::BoolVec(v) => len = v.len(),            
            ColumnType::DateVec(v) => len = v.len(),
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
    fn get_date<T:ConvertibleVal>(val:T) -> NaiveDate {
            match val.cvrt_to_date() {
                Some(x) => return x,
                None => return  NaiveDate::from_num_days_from_ce_opt(1).unwrap() }; 
        }
    //////////////////////////////////////////////////////////////////////////////

    fn collect_values(&self, idx:&Vec<usize>, col_idx:&Vec<usize>) -> Frame {
        let mut cols:Vec<Column> = Vec::with_capacity(self.size.0);

        for i in col_idx.clone() {
            let new_col:Column = Self::collect_gen(&self.data[i], idx);
            cols.push(new_col);
        }

        FrameBuilder::new()
            .size(col_idx.len(), idx.len())
            .data(cols)
            .col_indexed(Vec::new())
        .build()
    }
    fn collect_gen(col:&Column, idx:&Vec<usize>) -> Column {
        fn collect<T>(v:&Vec<Option<T>>, idx:&Vec<usize>) -> Vec<Option<T>> 
            where T: Clone 
        {
            let mut new_vals:Vec<Option<T>> = Vec::with_capacity(idx.len());
            for i in idx {
                if let Some(val) = &v[*i] {
                    new_vals.push(Some(val.clone()));
                }else {
                    new_vals.push(None);
                }
            }
            new_vals
        }
        
        let new_col = ColumnBuilder::new()
            .pos(col.pos.clone())
            .name(col.name.clone())
            .datatype(col.datatype.clone());

        match &col.values {
            ColumnType::StringVec(v) => {
                let new_vals:Vec<Option<String>> = collect(&v, idx);
                new_col.values(ColumnType::StringVec(new_vals)).build()
            },ColumnType::IntVec(v) => {
                let new_vals:Vec<Option<i32>> = collect(&v, idx);
                new_col.values(ColumnType::IntVec(new_vals)).build()
            },ColumnType::FloatVec(v) => {
                let new_vals:Vec<Option<f64>> = collect(&v, idx);
                new_col.values(ColumnType::FloatVec(new_vals)).build()
            },ColumnType::BoolVec(v) => {
                let new_vals:Vec<Option<bool>> = collect(&v, idx);
                new_col.values(ColumnType::BoolVec(new_vals)).build()
            },ColumnType::DateVec(v) => {
                let new_vals:Vec<Option<NaiveDate>> = collect(&v, idx);
                new_col.values(ColumnType::DateVec(new_vals)).build()
            },ColumnType::Empty() => {
                new_col.values(ColumnType::Empty()).build()
            }
        }
    }
    const SPACE_LIM:i32 = 18;
    pub fn print_frame(self) -> Self {
        if self.size.0 == 0 || self.size.1 == 0 { return self; }
        print!("{}", " #  ".blue().bold());
        Self::print_names(&self);

        let mut c:usize = 1;
        for l in 0..self.size.1 {
            if l % 5 == 0 { 
                if l == 0 { print!(""); }
                else { println!();}
            }

            print!("[{}] ", c);
            for col in &self.data {
                match &col.values {
                    ColumnType::StringVec(v) => {
                        if let Some(val) = &v[l] {
                            Self::print_cell(val);
                        }else { Self::print_none(); }
                    },ColumnType::IntVec(v) => { 
                        if let Some(val) = &v[l] {
                            let val:String = val.to_string();
                            Self::print_cell(&val);
                        }else { Self::print_none(); }
                    },ColumnType::FloatVec(v) => {
                        if let Some(val) = &v[l] {
                            let val:String = val.to_string();
                            Self::print_cell(&val);
                        }else { Self::print_none(); }
                    },ColumnType::BoolVec(v) => {
                        if let Some(val) = &v[l] {
                            if *val { 
                                print!("TRUE");
                                for _ in 0..Self::SPACE_LIM-4 { print!(" "); }
                            }
                            else { 
                                print!("FALSE");
                                for _ in 0..Self::SPACE_LIM-5 { print!(" ")}
                            }
                        }else { Self::print_none(); }
                    },ColumnType::DateVec(v) => {
                        if let Some(val) = &v[l] { 
                            let date:String = val.format("%d/%m/%Y").to_string();
                            Self::print_cell(&date);
                        }else { Self::print_none(); }
                    },ColumnType::Empty() => {
                        for _ in 0..Self::SPACE_LIM { print!(" "); }
                    },
                }
            }
            println!("");
            c += 1;
        }
        Self::print_dash(&self);
        println!("({}, {})", self.size.0, self.size.1);
        self
    }
    fn print_names(fr:&Frame) {
        for i in 0..fr.size.0 {
            let name:&String = &fr.data[i].name;
            Self::print_clr_cell(name);
        }
        println!();
        Self::print_dash(fr);
    }
    fn print_dash(fr:&Frame) {
        print!("{}", "--- ".blue().bold());
        for i in 0..fr.size.0 {
            let len:usize = fr.data[i].name.len();
            let mut s:String = String::with_capacity(len);
            for _ in 0..len {
                s.push('-');
            }
            Self::print_clr_cell(&s);
        }
        println!();
    }
    fn print_cell(s:&String) {
        let len:i32 = s.len() as i32;
        print!("{} ", s);
        if Self::SPACE_LIM - len <= 0 { return; }
        
        for _ in 0..(Self::SPACE_LIM-len-1) { print!(" "); }
    }
    fn print_clr_cell(s:&String) {
        let len:i32 = s.len() as i32;
        print!("{} ", s.blue().bold());
        if Self::SPACE_LIM - len <= 0 { return; }
        
        for _ in 0..(Self::SPACE_LIM-len-1) { print!(" "); }
    }
    fn print_none() {
        print!("{}","None".red().bold());
        for _ in 0..Self::SPACE_LIM-4 { print!(" "); }
    }

}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl FrameBuilder {

    pub fn empty() -> Frame {
        Frame { size: (0, 0), data: Vec::new(), col_indexed: Vec::new() }
    } 

    pub fn new() -> Self{
        FrameBuilder { size: None, data: None, col_indexed:None }
    }
    pub fn size(mut self, c:usize, r:usize) -> Self {
        self.size = Some((c,r));
        self
    }
    pub fn data(mut self, data:Vec<Column>) -> Self {
        self.data = Some(data);
        self
    }
    pub fn col_indexed(mut self, cols:Vec<usize>) -> Self {
        self.col_indexed = Some(cols);
        self
    }
    pub fn build(self) -> Frame {
        Frame { size: self.size.unwrap(), data: self.data.unwrap(), col_indexed: self.col_indexed.unwrap() }
    }

    //used exclusively to parse the csv into a Frame struct
    pub fn from_csv(reader: &mut BufReader<File>) -> Frame {
        FromCSV::csv(reader)
    }

}

fn print_desc(fr:&Frame, same:bool) {
    let a:Vec<&str> = vec!["UNIQ", "TOP", "FREQ", "MEAN", "STD", "MIN", "25%", "50%", "75%", "MAX"];
    let names:Vec<String> = fr.data.iter().map(|n| n.name.clone()).collect();

    let mut s:usize = 0;
    let mut e:usize = a.len();
    if same {
        if fr.data[0].datatype == DataType::String { e = 3; }
        else { s = 3; e = a.len()}
    }

    printer::print_names(&names);
    for i in s..e {
        printer::print_cell(&a[i].to_string());
        for col in &fr.data {
            match &col.values {
                ColumnType::StringVec(v) => {
                    if let Some(val) = &v[i] {
                        printer::print_cell(val);
                    }else { printer::print_none(); }
                },ColumnType::FloatVec(v) => {
                    if let Some(val) = &v[i] {
                        printer::print_cell(&val.to_string());
                    }else { printer::print_none(); }
                },ColumnType::DateVec(v) => {
                    if let Some(val) = &v[i] {
                        printer::print_cell(&val.to_string());
                    }else { printer::print_none(); }
                },
                _ => print!("[]")
            }
        }
        println!();
    }
    printer::print_dash(&names);
}
fn print_isna(fr:&Frame, nas:&HashMap<String,usize>) {
    let names:Vec<String> = vec![String::from("Columns"), String::from("NaN Count")];
    printer::print_names_without(&names);
    printer::print_dash_without(&names);    
    let mut i:usize = 0;
    for (k, v) in nas {
        if i % 5 == 0 { 
            if i == 0 { print!(""); }
            else { println!();}
        }
        printer::print_clr_cell(k);
        printer::print_cell(&v.to_string());
        println!();
        i += 1;
    }
    printer::print_dash_without(&names);    

}
pub fn open_file(path:impl Into<String>) -> BufReader<File>{
    let file = std::fs::File::open(path.into())
        .expect("File not found");

    BufReader::new(file)
}