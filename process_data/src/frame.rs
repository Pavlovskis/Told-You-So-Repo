use std::{
    io::{self, BufReader, prelude::*}, 
    fs::File, collections::HashMap, mem,
};

mod column_wrp;
mod helpe;
mod column;

use column::{Column, ColumnType, ColumnBduilder, DataType};
use helpe::*;
use column_wrp::Data;

use crate::column_wrp::DataBuilder;

#[derive(Debug, Clone)]
pub struct Frame {
    pub size:(usize, usize),
    pub data:Vec<Column>
}

pub struct FrameBuilder {
    pub size: Option<(usize, usize)>,
    pub data: Option<Vec<Column>>,
}

pub fn open_file(path:impl Into<String>) -> BufReader<File>{
    let file = std::fs::File::open(path.into())
        .expect("File not found");

    BufReader::new(file)
}

//take any kind of vector for the column positions
    //range[..1], [1..], vec![1,3,5], vec!["col_name",(...)]
pub trait IntakeValues {
    fn get_positions(self, fr:&Frame) -> Vec<usize>;
}
impl IntakeValues for [std::ops::Range<usize>;1] {
    fn get_positions(self, _:&Frame) -> Vec<usize> {
        (self[0].start..=self[0].end).collect()    }
}
impl IntakeValues for [std::ops::RangeFrom<usize>;1] {
    fn get_positions(self, fr:&Frame) -> Vec<usize> {
        if self[0].start > fr.size.0 {
            (self[0].start..=fr.size.1-1).collect()
        }else { (self[0].start..=fr.size.0-1).collect() }
    }
}
impl IntakeValues for [std::ops::RangeTo<usize>;1] {
    fn get_positions(self, _:&Frame) -> Vec<usize> {
        (0..=self[0].end).collect()    }
}
impl IntakeValues for Vec<&str> {
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
impl IntakeValues for Vec<usize> {
    fn get_positions(self, _:&Frame) -> Vec<usize> {
        self.into_iter().collect()    }
}
impl IntakeValues for &str {
    fn get_positions(self, fr:&Frame) -> Vec<usize> {
        let mut c:usize = 0;
        for i in &fr.data {
            if i.name == self { return vec![c]; }
            c += 1;
        }
        Vec::new()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl Frame {
    const SPACE_LIM:i32 = 18;

    fn sub(&self, r_len:usize) -> Frame {

        let mut cols:Vec<Column> = Vec::with_capacity(self.size.0);
        for col in &self.data {
            let mut ct:ColumnType = ColumnType::Empty();
            match &col.datatype {
                DataType::Int => ct = ColumnType::IntVec(Vec::with_capacity(r_len)),
                DataType::Float => ct = ColumnType::FloatVec(Vec::with_capacity(r_len)),
                DataType::String => ct = ColumnType::StringVec(Vec::with_capacity(r_len)),
                DataType::Bool => ct = ColumnType::BoolVec(Vec::with_capacity(r_len)),
                DataType::None => ct = ColumnType::Empty(),
            }
            let new_col:Column = ColumnBduilder::new()
                .pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).values(ct)
            .build();
            cols.push(new_col);
        }
        FrameBuilder::new().size(0, 0).data(cols).build()
    }

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

    //for column span
    pub fn span<T:IntakeValues>(&self, p:T) -> Frame {
        let col_pos:Vec<usize> = p.get_positions(&self);

        Self::collect_values(&self, &(0..=self.size.1-1).collect(), &col_pos)
    }

    //for line range
    pub fn range<T:IntakeValues>(&self, p:T) -> Frame {
        let line_pos:Vec<usize> = p.get_positions(&self);

        Self::collect_values(&self, &line_pos, &(0..=self.size.0-1).collect())
    }

    //returns a Vec<Column>
    pub fn col<T:IntakeValues>(&self, p:T) -> Data {
        let mut cols:Vec<Column> = Vec::with_capacity(self.size.0);

        let mut ps:Vec<usize> = Vec::with_capacity(self.size.0);
        for pos in p.get_positions(&self) {
            ps.push(pos);
            cols.push(self.data[pos].clone());
        }

        if cols.is_empty() { 
            println!("Couldn't find column(s)"); // red
            return DataBuilder::empty();
        }
        DataBuilder::new(ps, cols)
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

        print!(" # \t ");
        Self::print_cell(&String::from("Column"));
        println!("\tNull-Count \t\t Dtype");

        print!("---\t ");
        Self::print_cell(&String::from("-----"));
        println!("\t---------- \t\t -----");

        let mut dtypes:HashMap<DataType, usize> = HashMap::with_capacity(self.size.0+1);
        let mut total:usize = 0;

        let mut i:usize = 0;
        for col in &self.data {
            print!("[{}] \t ", i);
            Self::print_cell(&col.name);
            print!("({}) null-vals \t {:?}\n", 0, &col.datatype);

            match &col.values {
                ColumnType::StringVec(v) => { total += get_mem(v) },
                ColumnType::IntVec(v) => { total += get_mem(v) },
                ColumnType::FloatVec(v) => { total += get_mem(v) },
                ColumnType::BoolVec(v) => { total += get_mem(v) },
                ColumnType::Empty() => { total += 0 },
            }

            if dtypes.contains_key(&col.datatype) {
                dtypes.entry(col.datatype.clone()).and_modify(|x| *x += 1);
            }else { dtypes.insert(col.datatype.clone(), 1); }

            i += 1;
        }
        println!("----------------------------------------------");
        print!("Dtypes:");
        for (k, v) in dtypes {
            print!("{:?}({})  ",k, v);
        }
        println!("\n Memory usage: {:.1} kB", total as f64 / 1000.0 );
        
    }

    pub fn describe(&self) {
        let mut cols:Vec<Column> = Vec::with_capacity(self.size.0);
        for col in &self.data {
            cols.push(col.clone());
        }
        Data::describe(&DataBuilder::new((0..=self.size.0).collect(), cols));
    }

    pub fn loc(&self, pos:Vec<usize>) -> Frame {
        fn collect_values<T>(v:&Vec<Option<T>>, pos:&Vec<usize>, mut raw_vals:Vec<Option<T>>) -> Vec<Option<T>> 
            where T: Clone 
        { 
            for p in pos {
                raw_vals.push(v[*p].clone());
            }
            raw_vals
        }

        let mut cols:Vec<Column> = Vec::with_capacity(self.size.0);
        for col in &self.data {
            match &col.values {
                ColumnType::StringVec(v) => {
                    let mut raw_vals:Vec<Option<String>> = Vec::with_capacity(pos.len()+ 1);
                    raw_vals = collect_values(v, &pos, raw_vals);
                    let col:Column = ColumnBduilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).values(ColumnType::StringVec(raw_vals)).build();
                    cols.push(col);
                },ColumnType::IntVec(v) => {
                    let mut raw_vals:Vec<Option<i32>> = Vec::with_capacity(pos.len() + 1);
                    raw_vals = collect_values(v, &pos, raw_vals);
                    let col:Column = ColumnBduilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).values(ColumnType::IntVec(raw_vals)).build();
                    cols.push(col);
                },ColumnType::FloatVec(v) => {
                    let mut raw_vals:Vec<Option<f64>> = Vec::with_capacity(pos.len() + 1);
                    raw_vals = collect_values(v, &pos, raw_vals);
                    let col:Column = ColumnBduilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).values(ColumnType::FloatVec(raw_vals)).build();
                    cols.push(col);
                },ColumnType::BoolVec(v) => {
                    let mut raw_vals:Vec<Option<bool>> = Vec::with_capacity(pos.len() + 1);
                    raw_vals = collect_values(v, &pos, raw_vals);
                    let col:Column = ColumnBduilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).values(ColumnType::BoolVec(raw_vals)).build();
                    cols.push(col);
                },
                ColumnType::Empty() => cols.push( ColumnBduilder::new().pos(col.pos.clone()).name(col.name.clone()).datatype(col.datatype.clone()).values(ColumnType::Empty()).build() ),
            }
        }
        FrameBuilder::new().size(cols.len(), pos.len()).data(cols).build()
    }

    pub fn and(self) -> Self {
        self
    }

    fn count_nulls(col:&Column) -> usize {
        fn count<T>(v:&Vec<Option<T>>) -> usize{
            let mut c:usize = 0;
            for val in v {
                if let None = val { c += 1; }
            }
            c
        }
        match &col.values {
            ColumnType::StringVec(v) => { count(v) },
            ColumnType::IntVec(v) => { count(v) },
            ColumnType::FloatVec(v) => { count(v) },
            ColumnType::BoolVec(v) => { count(v) },
            ColumnType::Empty() => 0,
        }

    }

    fn collect_values(&self, idx:&Vec<usize>, col_idx:&Vec<usize>) -> Frame {
        let mut cols:Vec<Column> = Vec::with_capacity(self.size.0);

        for i in col_idx.clone() {
            let new_col:Column = Self::collect_gen(&self.data[i], idx);
            cols.push(new_col);
        }

        FrameBuilder::new()
            .size(col_idx.len(), idx.len())
            .data(cols)
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
        
        let new_col = ColumnBduilder::new()
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
            }ColumnType::Empty() => {
                new_col.values(ColumnType::Empty()).build()
            },
        }
    }

    fn print_names(fr:&Frame) {
        for i in 0..fr.size.0 {
            let name:&String = &fr.data[i].name;
            Self::print_cell(name);
        }
        println!();
        Self::print_dash(fr);
    }

    fn print_dash(fr:&Frame) {
        print!("--- ");
        for i in 0..fr.size.0 {
            let len:usize = fr.data[i].name.len();
            let mut s:String = String::with_capacity(len);
            for _ in 0..len {
                s.push('-');
            }
            Self::print_cell(&s);
        }
        println!();
    }

    fn print_cell(s:&String) {
        let len:i32 = s.len() as i32;
        print!("{} ", s);
        if Self::SPACE_LIM - len <= 0 { return; }
        
        for _ in 0..(Self::SPACE_LIM-len-1) { print!(" "); }
    }

    fn print_none() {
        print!("None");
        for _ in 0..Self::SPACE_LIM-4 { print!(" "); }
    }

    pub fn print_frame(self) -> Self {
        if self.size.0 == 0 || self.size.1 == 0 { return self; }
        print!(" #  ");
        Self::print_names(&self);

        let mut c:usize = 1;
        for l in 0..self.size.1 {
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
                    },ColumnType::Empty() => {
                        for _ in 0..Self::SPACE_LIM { print!(" "); }
                    },
                }
            } 
            println!("");
            c += 1;
        }
        Self::print_dash(&self);
        self
    }

}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl FrameBuilder {
    pub fn empty() -> Frame {
        Frame { size: (0, 0), data: Vec::new() }
    } 

    pub fn new() -> Self{
        FrameBuilder { size: None, data: None }
    }

    pub fn size(mut self, c:usize, r:usize) -> Self {
        self.size = Some((c,r));
        self
    }

    pub fn data(mut self, data:Vec<Column>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> Frame {
        Frame { size: self.size.unwrap(), data: self.data.unwrap() }
    }

    //used exclusively to parse the csv into a Frame struct
    pub fn new_fileframe(reader: &mut BufReader<File>) -> Frame {
        let fin:Vec<ColumnType> = refine_data(get_raw_data(reader));

        let names:Vec<String> = get_col_names(reader);

        let mut cols:Vec<Column> = Vec::with_capacity(names.len());

        let mut i:usize = 0;
        for col in fin {
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
                },ColumnType::Empty() => continue,
            }

            let new_col = ColumnBduilder::new()
                .pos(pos.clone())
                .name(names[i].clone())
                .datatype(dt)
                .values(col)
            .build();

            cols.push(new_col);
            i += 1;
        }

        Frame { 
            size:(get_cols(reader), get_rows(reader)),
            data:cols
        }
    }

}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn get_rows(reader:&mut BufReader<File>) -> usize {
    let mut rows:usize = 0;

    for _ in reader.lines() {
        rows += 1;
    }

    //offset the pointer to the beggining of the buffer iterator
    let _ = reader.seek(io::SeekFrom::Start(0));
    
    rows - 1
}

pub fn get_cols(reader:&mut BufReader<File>) -> usize {
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

//puts each col from csv into Vec<String>
pub fn get_raw_data(reader:&mut BufReader<File>) -> Vec<Vec<String>> {
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
                        if let Ok(n) = int_val {
                            f_vec.push(Some(n));
                        }else {
                            f_vec.push(None);
                        }
                    }
                }
                data_cols.push(ColumnType::FloatVec(f_vec));
            },
            (true, false) => {
                let mut int_vec:Vec<Option<i32>> = Vec::with_capacity(c.len());
                for val in c {
                    if is_na(&val) {
                        int_vec.push(None);
                    }else {
                        let int_val:Result<i32, _> = val.parse();
                        if let Ok(n) = int_val {
                            int_vec.push(Some(n));
                        }else {
                            int_vec.push(None);
                        }
                    }
                }
                data_cols.push(ColumnType::IntVec(int_vec));
            },
            _ => {
                let mut new_vec:Vec<Option<String>> = Vec::with_capacity(c.len());
                for val in c {
                    if is_na(&val) {
                        new_vec.push(None);
                    }else {
                        new_vec.push(Some(val));
                    }
                }
                data_cols.push(ColumnType::StringVec(new_vec));
            },
        }
    }
    data_cols
}
