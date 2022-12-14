// use std::collections::HashMap;

use std::collections::HashMap;

fn main() {
    // Given a list of integers, use a vector and return the median

    // Sample vectors
    // let mut vec = vec![1, 4, 5, 2, 3];
    let int_vector = vec![1, 4, 3, 2];

    // print median
    median(int_vector);

    let int_vector = vec![1, 4, 3, 2, 2, 2, 1, 3, 4, 1, 1, 1, 2];
    // print mode
    mode(int_vector);
}

fn median(vect: Vec<i32>) {
    let mut vec = Vec::from(vect);

    vec.sort();
    println!("{:?}", vec);
    // println!("length = {:?}", vec.len());

    let len = vec.len();

    if len % 2 == 0 {
        // println!("even number of elements");
        let start_index = len / 2 - 1;
        let end_index = len / 2;

        let median: f32 = (vec[start_index] + vec[end_index]) as f32 / 2.0;
        println!("median: {}", median);
    } else {
        // println!("odd number of elements");
        let mid_number = vec.len() / 2;
        println!("median: {}", vec[mid_number]);
    }
}

fn mode(vect: Vec<i32>) {
    println!("{:?}", vect);
    let mut map = HashMap::new();

    for integer in vect {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
    }

    let mut maxKey = 0;
    let mut maxValue = 0;
    for (key, value) in &map {
        if value > &maxValue {
            maxValue = *value;
            maxKey = *key;
        }
        // println!("{}: {}", key, value);
    }
    println!("mode - {}: occurs {} times", maxKey, maxValue);
}
