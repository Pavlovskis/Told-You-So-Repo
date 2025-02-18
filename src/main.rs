use std::time::Instant;

use structs::list::list::FindSmallest as ListFindSmallest;
use structs::heap::FindSmallest as HeapFindSmallest; 

use num_gen::Generator;

mod num_gen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let now:Instant = Instant::now();

    //////////////////////////////////////////////////////////////

    let percents:Vec<f32> = vec![0.1, 0.5, 1.0, 2.0, 5.0, 8.0, 10.0, 15.0, 25.0, 50.0];

    let quantities:Vec<usize> = vec![1_000, 10_000, 100_000, 500_000, 1_000_000, 2_500_000, 5_000_000, 10_000_000];

    let mut num_gen: Generator = Generator::new(Some(42));

    println!("Generating Numbers ...\n");
    let mut batches:Vec<Vec<usize>> = Vec::with_capacity(quantities.len());
    for quantity in quantities {
        batches.push(num_gen.generate_batch(quantity, (0, 50_000)));
    }

    println!("### Heap Benchmarks ###");

    for batch in batches.iter() {
        for percent in percents.iter() {
            let t_now:Instant = Instant::now();
    
            let total:usize = ((percent / 100.0) * batch.len() as f32) as usize;
            let _ = HeapFindSmallest::find_smallest(batch, total);
            
            println!("[Q {} | P {}] => {:?}", batch.len(), percent, t_now.elapsed());
        }
        println!();
    }

    println!("### List Benchmarks ###");

    for batch in batches.iter() {
        for percent in percents.iter() {
            let t_now:Instant = Instant::now();
    
            let total:usize = ((percent / 100.0) * batch.len() as f32) as usize;
            let _ = ListFindSmallest::find_smallest(batch, total);
            
            println!("[Q {} | P {}] => {:?}", batch.len(), percent, t_now.elapsed());
        }
        println!();
    }

    //////////////////////////////////////////////////////////////

    println!("  Time: {:?}", now.elapsed());

    Ok(())
}
