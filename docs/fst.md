### Finite state machine algorithm

* Using array

```rust
const MAX_STATES: usize = 1_000_000;
const MAX_ALPHABET_SIZE: usize = 256;

struct FSM {
    transitions: [[usize; MAX_ALPHABET_SIZE]; MAX_STATES],
    outputs: [u32; MAX_STATES],
    num_states: usize,
}

impl FSM {
    fn new() -> Self {
        Self {
            transitions: [[0; MAX_ALPHABET_SIZE]; MAX_STATES],
            outputs: [0; MAX_STATES],
            num_states: 1,
        }
    }

    fn add_key(&mut self, key: &[u8], output: u32) {
        let mut state = 0;
        for &byte in key {
            let next_state = self.transitions[state][byte as usize];
            if next_state == 0 {
                self.transitions[state][byte as usize] = self.num_states;
                state = self.num_states;
                self.num_states += 1;
            } else {
                state = next_state;
            }
        }
        self.outputs[state] = output;
    }

    fn get_output(&self, key: &[u8]) -> Option<u32> {
        let mut state = 0;
        for &byte in key {
            let next_state = self.transitions[state][byte as usize];
            if next_state == 0 {
                return None;
            } else {
                state = next_state;
            }
        }
        Some(self.outputs[state])
    }
}

```


* Using HashMap
```rust
use std::collections::HashMap;

type State = usize;
type Input = u8;
type Output = u32;

struct FSM {
    transitions: Vec<HashMap<Input, State>>,
    outputs: Vec<Output>,
}

impl FSM {
    fn new() -> Self {
        Self {
            transitions: vec![HashMap::new()],
            outputs: vec![0],
        }
    }

    fn add_key(&mut self, key: &[u8], output: Output) {
        let mut state = 0;
        for &byte in key {
            let next_state = *self.transitions[state].entry(byte).or_insert_with(|| {
                let next_state = self.transitions.len();
                self.transitions.push(HashMap::new());
                self.outputs.push(0);
                next_state
            });
            state = next_state;
        }
        self.outputs[state] = output;
    }

    fn get_output(&self, key: &[u8]) -> Option<Output> {
        let mut state = 0;
        for &byte in key {
            state = match self.transitions[state].get(&byte) {
                Some(&next_state) => next_state,
                None => return None,
            };
        }
        Some(self.outputs[state])
    }
}
```