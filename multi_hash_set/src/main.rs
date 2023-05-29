mod multi_hash_set;

use multi_hash_set::MultiHashSet;
use std::{time::Instant, collections::HashSet};

fn main() {
    let mut my_set: MultiHashSet<String> = MultiHashSet::new();
    //let your_set: HashSet<String> = HashSet::new();

    let start_time = Instant::now();

    for i in 0..50000 {
        my_set.put(i.to_string());
    }

    for i in 0..50000 {
        my_set.count(&i.to_string());
    }
    
    let elapsed_time = start_time.elapsed();

    println!("Execution time: {:?}", elapsed_time);
}
