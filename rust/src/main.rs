mod merge_sorted_array;
use std::vec;

use merge_sorted_array::merge;

fn main() {
    // Test 1
    let mut vec_a = vec![1, 2, 3, 0, 0, 0];
    let mut vec_b = vec![2, 5, 6];
    let mut size_a = vec_a.len() as i32;
    let mut size_b = vec_b.len() as i32;
    merge(&mut vec_a, size_a, &mut vec_b, size_b);

    // Test 2
    vec_a = vec![1];
    vec_b = vec![];
    size_a = vec_a.len() as i32;
    size_b = vec_b.len() as i32;
    merge(&mut vec_a, size_a, &mut vec_b, size_b);

    // Test 3
    vec_a = vec![0];
    vec_b = vec![1];
    size_a = 0;
    size_b = 1;
    merge(&mut vec_a, size_a, &mut vec_b, size_b);
}