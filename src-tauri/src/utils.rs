use rand::distributions::{Alphanumeric,Standard,Bernoulli};
use rand::Rng;

pub fn random_alphanumeric_string(length:usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}