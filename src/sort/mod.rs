use rand::prelude::SliceRandom;

pub mod bubble;
pub mod bucket;
pub mod cocktail;
pub mod counting;
pub mod insertion;
pub mod radix;

pub fn gen_vec(len: usize) -> Vec<usize> {
    use rand::thread_rng;
    let l = len;
    let mut vec: Vec<usize> = (0..l).to_owned().collect();
    vec.shuffle(&mut thread_rng());
    vec
}
