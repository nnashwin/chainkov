# Chainkov [![Build Status](https://travis-ci.org/ru-lai/chainkov.svg?branch=master)](https://travis-ci.org/ru-lai/chainkov)
> HashMap / tuple-backed Markov Chains 

## Install

- Add the dependency to the Cargo.toml of your project

```
// Cargo.toml
[dependencies]
chainkov = "0.1.0"
```

- Run cargo build
```
cargo build
```

## Usage
```rust
extern crate chainkov;

use chainkov::*;

let mut m = MarkovChain::new();
// MarkovChain{ transition_prob: {} }

m.add_state_choice("a", ("b".to_string(), 1.0));
// MarkovChain { transition_prob: {"a": [("b", 0.4)]} }

m.add_state_choice("b", ("c".to_string(), 1.0));
// MarkovChain { transition_prob: {"a": [("b", 1.0)], "b": [("c", 1.0)]} }

m.add_state_choice("c", ("d".to_string(), 1.0));
// MarkovChain { transition_prob: {"c": [("a", 1.0)], "a": [("b", 1.0)], "b": [("c", 1.0)]} }

m.generate_states("a".to_string(), 4);
// ["b", "c", "a", "b"]

m.next_state("a".to_string());
// "b"
```
