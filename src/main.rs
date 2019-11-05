use rand::prelude::*;
use std::collections::HashMap;

struct MarkovChain {
    pub transition_prob: HashMap<String, Vec<(String, f32)>>,
    states: Vec<String>,
}

impl MarkovChain {
    fn next_state(&mut self, current_state: String) -> String {
        let probabilities = &self.transition_prob.get(&current_state).unwrap();

        return probabilities
            .choose_weighted(&mut thread_rng(), |state_prob| state_prob.1)
            .unwrap()
            .0
            .clone();
    }

    fn new() -> MarkovChain {
        MarkovChain {
            transition_prob: HashMap::new(),
            states: Vec::new(),
        }
    }

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
}

fn main() {
    let mut m_chain = MarkovChain::new();
    m_chain.add_state_choice("a", ("c".to_string(), 0.8));
    println!("{:?}", m_chain.transition_prob);
    m_chain.add_state_choice("a", ("b".to_string(), 0.8));
    println!("{:?}", m_chain.transition_prob);
    m_chain.add_state_choice("a", ("b".to_string(), 0.6));
    println!("{:?}", m_chain.transition_prob);
    m_chain.add_state_choice("a", ("a".to_string(), 0.9));
    println!("{:?}", m_chain.transition_prob);
    m_chain.add_state_choice("a", ("d".to_string(), 0.6));
    println!("{:?}", m_chain.transition_prob);
    m_chain.add_state_choice("a", ("f".to_string(), 0.9));
    println!("{:?}", m_chain.transition_prob);
    m_chain.add_state_choice("a", ("c".to_string(), 0.4));
    println!("{:?}", m_chain.transition_prob);
}
