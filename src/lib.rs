use rand::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub struct MarkovChain {
    pub transition_prob: HashMap<String, Vec<(String, f32)>>,
}

impl MarkovChain {
    fn add_state_choice(&mut self, key: &str, probability: (String, f32)) {
        if self.transition_prob.contains_key(key) {
            // write logic to replace probability if it already exists in the Vector
            match self
                .transition_prob
                .get_mut(key)
                .unwrap()
                .iter()
                .position(|x| x.0 == probability.0)
            {
                Some(x) => {
                    let vec_to_swap = self.transition_prob.get_mut(key).unwrap();
                    vec_to_swap.push(probability);
                    vec_to_swap.swap_remove(x);
                }
                None => self.transition_prob.get_mut(key).unwrap().push(probability),
            }
        } else {
            let mut prob_vec = vec![];
            prob_vec.push(probability);
            self.transition_prob.insert(key.to_string(), prob_vec);
        }
    }

    fn generate_states(&self, mut current_state: String, num_of_states: u16) -> Vec<String> {
        let mut future_states: Vec<String> = vec![];
        
        for _ in 0..num_of_states {
            let next_state = self.next_state(current_state.to_string());
            future_states.push(next_state.clone());
            current_state = next_state;
        }
        return future_states;
    }

    fn new() -> MarkovChain {
        MarkovChain {
            transition_prob: HashMap::new(),
        }
    }

    fn next_state(&self, current_state: String) -> String {
        let probabilities = if self.transition_prob.contains_key(&current_state) {
            self.transition_prob.get(&current_state)
        } else {
            None
        };

        match probabilities {
            Some(x) => { 
                return x
                    .choose_weighted(&mut thread_rng(), |state_prob| state_prob.1)
                    .unwrap()
                    .0
                    .clone();

            },
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = MarkovChain::new();
        let expected = MarkovChain {
            transition_prob: HashMap::new(),
        };

        assert_eq!(m.transition_prob, expected.transition_prob);
    }

    #[test]
    fn test_add_state_choice() {
        let mut m = MarkovChain::new();
        m.add_state_choice("a", ("c".to_string(), 0.8));
        m.add_state_choice("a", ("b".to_string(), 0.19));
        m.add_state_choice("a", ("a".to_string(), 0.01));

        let mut expected_prob_vec = vec![];
        expected_prob_vec.push(("c".to_string(), 0.8));
        expected_prob_vec.push(("b".to_string(), 0.19));
        expected_prob_vec.push(("a".to_string(), 0.01));

        let mut expected_hash_map = HashMap::new();
        expected_hash_map.insert("a", expected_prob_vec);

        assert_eq!(m.transition_prob.get("a"), expected_hash_map.get("a"));
    }

    #[test]
    fn test_next_state() {
        let mut m = MarkovChain::new(); 
        m.add_state_choice("a", ("b".to_string(), 1.0));
        m.add_state_choice("a", ("c".to_string(), 0.0));

        assert_eq!(m.next_state("a".to_string()), "b");
        assert!(m.next_state("a".to_string()) != "c");
        // test that a state that doesn't exist in the chain returns an empty string
        assert_eq!(m.next_state("b".to_string()), "");
    }

    #[test]
    fn test_generate_states() {
        let mut m = MarkovChain::new(); 
        m.add_state_choice("a", ("b".to_string(), 1.0));
        m.add_state_choice("b", ("c".to_string(), 1.0));
        m.add_state_choice("c", ("a".to_string(), 1.0));

        assert_eq!(m.generate_states("a".to_string(), 6), vec!["b", "c", "a", "b", "c", "a"]);
        assert!(m.next_state("a".to_string()) != "c");
    }
}
