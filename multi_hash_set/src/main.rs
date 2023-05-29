mod multi_hash_set;


use multi_hash_set::MultiHashSet;
use std::{time::Instant, collections::HashSet};

use crate::multi_hash_set::next_prime;

fn main() {
    let mut my_set: MultiHashSet<String> = MultiHashSet::new();
    //let your_set: HashSet<String> = HashSet::new();

    let start_time = Instant::now();

    for i in 0..1000 {
        my_set.put(i.to_string());
    }

    my_set.remove(&123.to_string());
    my_set.remove(&838.to_string());

    for i in 0..10000 {
        println!("{}", my_set.count(&i.to_string()));
    }

    println!("-----------------------------------------");


    for element in my_set.iter() {
        println!("{}", element);
    }


    
    let elapsed_time = start_time.elapsed();

    println!("Execution time: {:?}", elapsed_time);
}
