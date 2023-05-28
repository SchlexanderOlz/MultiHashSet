mod multi_hash_set;


use multi_hash_set::MultiHashSet;

fn main() {
    let mut my_set: MultiHashSet<String> = MultiHashSet::new();

    
    for i in 0..1000 {
        my_set.put(i.to_string());
    }


    for val in my_set.iter() {
        println!("{}", val)
    }
}
