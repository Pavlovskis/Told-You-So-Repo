use std::vec;

use tokio::time::error::Elapsed;


pub fn solve_nto1(n:u32) -> () {
    if n <= 0 { return; }

    println!("{n}");

    solve_nto1(n -1);    
}

pub fn solve_1ton(n:u32) -> () {
    if n <= 0 { return; }

    solve_1ton(n-1);

    print!("{n} ");
}

pub fn fact(n:u32) -> u32 {
    if n <= 0 { return 1; }

    let res = n * fact( n-1 );

    println!("{res}");

    res
}

//?????
pub fn fib(n:u32) -> u32{
    if n <= 1 { return 1; }

    let res = fib(n-1) + fib(n-1);

    res
}

pub fn filter(v:&mut Vec<i32>, n:i32) -> Vec<i32>{
    let len:usize = v.len();

    fn iter(v:&mut Vec<i32>, i:usize, n: i32) -> &mut Vec<i32>{
        if i <= 0 { return v; }
        if v[i] == n {
            v[i] = 0;
        }
        iter(v, i-1, n)
    }
    iter(v, len-1, n);

    v.to_vec()
}



pub fn create_list(n:u32) -> Vec<u32> {
    let mut v:Vec<u32> = Vec::with_capacity(n as usize);

    fn increase(n:u32, c:u32, v:&mut Vec<u32>) -> Vec<u32> {
        if c >= n { return v.to_vec(); }
        else {
            v.push(c);
            increase(n, c+1, v)
        }
    }
    increase(n, 1, &mut v);

    return v;
}

