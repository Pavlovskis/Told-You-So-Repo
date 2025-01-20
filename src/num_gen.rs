#![allow(dead_code)]

use std::{fs::{File, OpenOptions}, io::Write};

use rand::{distributions::uniform::SampleUniform, rngs::{StdRng, ThreadRng}, Rng, SeedableRng};

pub enum RngType {
    Rng(ThreadRng),
    SRng(StdRng)
}

pub struct Generator {
    pub seed:Option<u64>,
    rng:RngType,
}

impl Generator {
    pub fn new(seed:Option<u64>) -> Self {
        if let Some(s) = seed {
            let srng = StdRng::seed_from_u64(s);
            return Self { 
                seed,
                rng: RngType::SRng(srng)
            };
        }

        Self { seed, rng: RngType::Rng( rand::thread_rng() ) }
    }

    /// Generate an amount of numbers of your choosing
    /// 
    /// #### quantity: Amount of numbers you want to generate
    /// #### range: Range of numbers (from, to)
    pub fn generate_batch<T>(&mut self, quantity:usize, range:(T, T)) -> Vec<T> 
        where T: SampleUniform + PartialOrd + Copy
    {
        let mut res:Vec<T> = Vec::with_capacity(quantity);

        match &mut self.rng {
            RngType::Rng( rng ) => {
                for _ in 0..quantity {
                    res.push(rng.gen_range(range.0..=range.1));
                }
            },
            RngType::SRng( srng ) => {
                for _ in 0..quantity {
                    res.push(srng.gen_range(range.0..=range.1));
                }
            }
        }

        res
    }

    /// Same as generate_batch() but put it into a file
    /// unfinished
    pub fn generate_batch_to_file<T>(&mut self, quantity:usize, range:(T, T), path:&str) 
        where T: SampleUniform + PartialOrd + Copy + ToString + std::fmt::Debug
    {
        let _tfile:File = File::create(path).expect("File could not be truncated");

        let mut file:File = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
        .expect("Error Open in write mode");

        match &mut self.rng {
            RngType::Rng( rng ) => {
                for i in 0..quantity {
                    if i % 100 == 0 {
                        let _ = file.write_all( format!("\n").as_bytes() );   
                    } 
                    let _ = file.write_all( format!("{:?} ", rng.gen_range(range.0..=range.1) ).as_bytes() );
                }
            },
            RngType::SRng( srng ) => {
                for i in 0..quantity {
                    if i % 100 == 0 {
                        let _ = file.write_all( format!("\n").as_bytes() );   
                    }
                    let _ = file.write_all( format!("{:?} ", srng.gen_range(range.0..=range.1) ).as_bytes() );
                }
            }
        }
    }

}