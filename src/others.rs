#![allow(dead_code)]
//conversion
pub mod purple{
    use std::any::type_name;

    pub fn str_to_i(s:&str) -> i32{
        return s.parse::<i32>().unwrap();
    }

    pub fn str_to_f(s:&str) -> f32{
        return s.parse::<f32>().unwrap();
    }

    //unsigned int
    pub fn bin_to_u(b:&str) -> u8{ 
        return u8::from_str_radix(b, 2).expect("Not binary number");
    }

    pub fn chars_to_str(v:Vec<char>) -> String{
        let mut s:String = String::new();
        for c in v{
            s.push(c);
        }
        return s;
    }

    pub fn is_numeric(val:&str) -> bool{
        let chars:Vec<char> = val.chars().collect();

        for c in chars{
            if (c as u8) < 48 || (c as u8) > 57{
                return false;
            }
        }
        return true;

    }

    pub fn type_of<T>(_: T) -> &'static str {
        return type_name::<T>();
    }

}

pub mod blue{

    pub fn train_test_split(v:&mut Vec<usize>, test_size:usize) -> (&Vec<usize>, Vec<usize>){
        let mut test_split:Vec<usize> = vec![];

        let split:usize = (v.len() * test_size) / 100;

        while split <= v.len() - 1{
            if split == v.len(){
                v.pop();
                break;
            }
            test_split.push(v[split]);
            v.remove(v[split]);
        }

        return (v, test_split);
    }

    pub mod scallers{
        use crate::{others::black::{get_min, get_max, get_log}, stats::base::{z_score, mean, std_deviation}};

        pub fn min_max_scaler(mut v:Vec<f32>) -> Vec<f32>{
            let min:f32 = get_min(&v) as f32;
            let max:f32 = get_max(&v) as f32;
    
            // println!("min:{} max:{}", min, max);
        
            for i in 0..v.len(){
                v[i] = (v[i] as f32 - min) / (max - min);
            }
            return v;
        }

        pub fn log_scaller(mut v:Vec<f32>) -> Vec<f32>{
            for i in 0..v.len(){
                v[i] = get_log(v[i]);
            }
            return v;
        }

        pub fn z_scaler(mut v:Vec<f32>) -> Vec<f32>{
            let mean:f32 = mean(&v);
            let std_dev:f32 = std_deviation(&v);

            for i in 0..v.len(){
                v[i] = z_score(v[i], mean, std_dev);
            }
            return v;
        }

        pub fn clip_scaler(mut v:Vec<f32>, min:Option<f32>, max:Option<f32>) -> Vec<f32>{

            let mut mi:f32 = 0.0;
            if let Some(min) = min{mi = min;}
            println!("{}", mi);

            let mut ma:f32 = 0.0;
            if let Some(max) = max{ma = max;}
            println!("{}", ma);

            for i in 0..v.len(){
                if v[i] < mi {
                    v[i] = mi;
                }else if v[i] > ma {
                    v[i] = ma;
                }else{ continue; }
            }
            return v;
        }
    }

}

pub mod black{

    pub fn ppi_matrix(v:&Vec<i32>) {
        for i in 0..v.len(){
            if i % 10 == 0 && i != 0{
                println!(" {} ", v[i]);
            }else {
                print!(" {} ", v[i]);
            }
        }
    }
    pub fn ppf_matrix(v:&Vec<f32>) {
        for i in 0..v.len(){
            if i % 10 == 0 && i != 0{
                println!(" {} ", v[i]);
            }else {
                print!(" {} ", v[i]);
            }
        }
    }

    pub fn get_min(v:&Vec<f32>) -> f32{
        let mut min:f32 = v[0];
        for i in 0..v.len(){
            if v[i] < min{ min = v[i]; }
        }
        return min;
    }

    pub fn get_max(v:&Vec<f32>) -> f32{
        let mut max:f32 = v[0];
        for i in 0..v.len(){
            if v[i] > max{ max = v[i]; }
        }
        return max;
    }

    pub fn get_log(n:f32) -> f32{
        return f32::log10(n as f32);
    }
}
