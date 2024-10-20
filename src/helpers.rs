#![allow(dead_code)]
use num::Num;
use rand::Rng;
use std::vec::Vec;

pub fn shuffle<T>(array: &mut [T]) {
    let mut rng = rand::thread_rng();
    for i in 0..array.len() {
        array.swap(i, rng.gen_range(0..array.len()));
    }
}

pub fn gen_vec<T>(start: T, end: T) -> Vec<T>
where
    T: Num + PartialOrd + Copy,
{
    let mut vec = Vec::new();
    let mut current = start;
    while current <= end {
        vec.push(current);
        current = current + T::one();
    }
    vec
}
