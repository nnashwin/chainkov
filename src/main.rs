use rand::prelude::*;
use std::collections::HashMap;

struct MarkovChain {
    transition_prob: HashMap<String, HashMap<String, u8>>,
    states: Vec<String>,
}

fn main() {
    println!("Hello, world!");
}
