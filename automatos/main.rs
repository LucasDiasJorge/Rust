// Define the states of the DFA
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Start,
    EvenOnes,
    OddOnes,
}

// Define the alphabet
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Symbol {
    Zero,
    One,
}

// Define the DFA transition function
fn transition(current_state: State, symbol: Symbol) -> State {
    match (current_state, symbol) {
        (State::Start, Symbol::Zero) => State::Start,
        (State::Start, Symbol::One) => State::OddOnes,
        (State::EvenOnes, Symbol::Zero) => State::EvenOnes,
        (State::EvenOnes, Symbol::One) => State::OddOnes,
        (State::OddOnes, Symbol::Zero) => State::OddOnes,
        (State::OddOnes, Symbol::One) => State::EvenOnes,
    }
}

// Define the DFA simulation function
fn simulate_dfa(input: &[Symbol]) -> bool {
    let mut current_state = State::Start;

    for symbol in input {
        current_state = transition(current_state, *symbol);
    }

    current_state == State::EvenOnes
}

fn main() {
    // Test the DFA simulation with some examples
    let test_cases = [
        (vec![Symbol::Zero, Symbol::One, Symbol::Zero], false),
        (vec![Symbol::One, Symbol::One, Symbol::Zero, Symbol::One], true),
        (vec![Symbol::Zero, Symbol::Zero, Symbol::One], true),
        (vec![Symbol::One, Symbol::One, Symbol::One], false),
    ];

    for (input, expected_result) in test_cases.iter() {
        let result = simulate_dfa(&input);
        println!(
            "Input: {:?}, Expected: {}, Result: {}",
            input,
            expected_result,
            result
        );
    }
}
