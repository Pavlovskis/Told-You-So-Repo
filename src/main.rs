use std::{
    time::Instant, fs::File, 
    io::{prelude::*,BufReader}
};

use chrono::NaiveDate;
use process_data2::*;
use rand::Rng;

pub fn compress(s:&str) -> String {
    let mut comp_s:String = String::with_capacity(25);

    let mut count:usize = 0;
    let mut first:char = ' ';

    for c in s.chars() {
            if c != first {
            first = c;

            if count > 1 {
                comp_s.push_str(&count.to_string());
            }
            
            comp_s.push(first);
            count = 1;
        }else { count += 1; }
    }
    comp_s.push_str(&count.to_string());

    comp_s
}

pub fn windit(v:&Vec<i32>) {
    let mut win_size:usize = v.len();

    let mut r:usize = win_size;
    let mut l:usize = 0;
    while win_size > 0 {
        for _ in 0..v.len() - win_size + 1 {
            for j in l..r {
                print!("{} ",v[j]);
            }
            println!();
            r += 1;
            l += 1;
        }
        win_size -= 1;
        r = win_size;
        l = 0;
    }
    
}

pub fn mean<T>(v:&Vec<T>) -> f64
    where T:Default + Copy + Into<f64> {

    let sum:f64 = v.iter().copied().map(T::into).sum();
    
    sum / v.len() as f64
}

pub fn mode<T>(v:&mut Vec<T>) -> (T, T)
    where T:Ord + From<i32> + Copy {
    
    if v.len() == 0 { return (0.into(), 0.into()) }
    if v.len() == 1 { return (v[1].clone(), 1.into()); }

    v.sort();

    let mut max:i32 = 0;
    let mut count:i32 = 1;
    let mut cur_mode = v[1].clone();
    for i in 0..(v.len() - 1) {
        if v[i] != v[i+1] {
            if count > max {
                max = count;
                cur_mode = v[i];
            };
            count = 1;
        }else {
            count += 1;
        }
    }

    (cur_mode, max.into())
}

// pub fn median<T>(v:&Vec<T>) -> T 
//     where T: Copy + std::ops::Div<T> + From<i32> + std::ops::Add<T> {
//     let len:usize = v.len();
//     if len % 2 == 1 { return v[len / 2] }
//     if len % 2 == 0 {
//         let mid1 = v[len - 1];
//         let mid2 = v[len];
//         (mid1 + mid2) / T::from(2)
//     }

//     v[vec_len+1]
// }

fn check_string(val:impl Into<String>) -> (bool, bool){
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

fn gen_date() -> String {
    let day = rand::thread_rng().gen_range(1..=30);
    let mon = rand::thread_rng().gen_range(1..=12);
    let year = rand::thread_rng().gen_range(2000..=2010);
    let date = format!(",{}/{}/{}\n", day.to_string(), mon.to_string(), year.to_string());

    date
}

fn add_date(path:&str) {
    let file:File = File::open(path)
        .expect("File not found");

    let reader:BufReader<File> = BufReader::new(file);

    let mut output_file:File = File::create("./data/peng_cp.csv")
        .expect("Couldn't create file");

    for line in reader.lines() {
        if let Ok(mut l) = line {
            l.push_str(&gen_date());

            output_file.write_all(l.as_bytes()).expect("Couldn't Write to file");
        }
    }
}

pub fn type_of<T>(_: &T) -> &str{
    std::any::type_name::<T>()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now: Instant = Instant::now();

    //////////////////////////////////////////////////////////////


    // let mut v:Vec<Option<usize>> = Vec::with_capacity(100);
    // for i in 0..100{
    //     v.push(Some(i));
    // }

    // let mut mem:usize = v.iter().map(|x| std::mem::size_of_val(x)).sum();
    // mem += std::mem::size_of_val(&v.len());
    // println!("{}", mem);

    let mut pf = FrameBuilder::from_csv(&mut process_data2::open_file("./data/peng_cp.csv"));
    // println!("{:?}", pf);

    // pf.head(Some(22)).print_frame(); // V

    // pf.tail(Some(4)).print_frame(); // V

    // let new = pf.range([2..9]).print_frame(); // V
    // pf.range([335..]).print_frame();
    // pf.range([..10]).print_frame();
    // pf.range(vec![1,3,5]).print_frame();


    // let small = new.span(vec![1,3,5]).print_frame(); // V
    // new.span(vec!["sex", "species"]).print_frame();
    // new.span([..3]).print_frame();
    // new.span([4..]).print_frame();
    // new.span([2..4]).print_frame();

    // pf.info(); // V

    // pf.data[6].describe(); // v
    // pf.data[3].describe();
    // pf.col(vec![0,1,6]).describe(); // v
    // pf.col(vec![2,3,4,5]).describe();
    // pf.describe();

    // let map = pf.data[0].count_values(); // v
    // let map = pf.col(vec!["species", "island"]).count_values();// 

    // let res = pf.loc( pf.col("date").eq("03/10/2010") ).print_frame();
    // let res2 = pf.loc( pf.col("body_mass_g").eq(3800) ).print_frame();
    // let res3 = pf.loc( pf.col("culmen_length_mm").eq(37.7) ).print_frame();
    // let res4 = pf.loc( pf.col("sex").eq("MALE")).head(Some(7)).print_frame();

    // let less_res = pf.loc( pf.col("body_mass_g").lt(3000) ).print_frame();
    // let empty = pf.loc( pf.col("species").lt(200) ).print_frame();
    // let less_res2 = pf.loc( pf.col("culmen_length_mm").lt(35.0) ).print_frame();
    // let less_res2 = pf.loc( pf.col("culmen_length_mm").lt(35) ).print_frame();

    // let more_res = pf.loc( pf.col("flipper_length_mm").mt(230) ).print_frame();
    // let more_res2 = pf.loc( pf.col("flipper_length_mm").mt(230.0) ).print_frame();
    // let more_res3 = pf.loc( pf.col("culmen_depth_mm").mt(21) ).print_frame();
    // let empty = pf.loc( pf.col("culmen_depth_mm").mt("MALE") ).print_frame();

    // let mte_res = pf.loc( pf.col("body_mass_g").lte(3000) ).print_frame();
    // let res = pf.loc( pf.col("body_mass_g").mte(5700) ).print_frame(); // and, or still to be done

    // pf.head(None).print_frame();
    // pf = pf.find( pf.col("culmen_depth_mm") ).add(1);
    // pf.head(None).print_frame();

    // pf.head(None).print_frame();
    // pf = pf.find( pf.col(vec!["culmen_depth_mm", "date"]) ).sub(5.4); // floats need rounding 
    // pf.head(None).print_frame();    

    // pf.head(None).print_frame();
    // pf = pf.find( pf.col(vec![4, 5]) ).div(4); // floats need rounding 
    // pf.head(None).print_frame();   

    
    // pf.head(None).print_frame();
    // let species = pf.pop("island");
    // pf.head(None).print_frame();   
    // println!("{:?}", species);

    pf.is_na();
    pf.col("sex").drop_na(None);

    // pf = pf.find( pf.col("body_mass_g").lte(2900) ).replace(6969);
    // pf.loc( pf.col("body_mass_g").eq(6969) ).print_frame(); 
    // pf.print_frame();

    //////////////////////////////////////////////////////////////    

    println!("  Time: {:#?}", now.elapsed());

    Ok(())
}
