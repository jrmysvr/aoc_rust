use std::fs;

#[warn(dead_code)]
fn initial_state(input_text: &str) -> Vec<u32> {
    input_text.trim()
              .split(",")
              .map(|s| s.parse().unwrap())
              .collect()
}

#[warn(dead_code)]
fn run(state: &mut Vec<u32>) {

    for i in (0..state.len()).step_by(4) {
        let ix1 = state[i+1] as usize;
        let ix2 = state[i+2] as usize;
        let wix = state[i+3] as usize;
        if state[i] == 99 {
            break;
        }else if state[i] == 1 {
            let value = state[ix1] + state[ix2];
            state[wix] = value;
        }else if state[i] == 2 {
            let value = state[ix1] * state[ix2];
            state[wix] = value;
        }else {
            panic!("Wrong code");
        }
    }
}

struct NV {
    noun: u32,
    verb: u32
}

#[warn(dead_code)]
fn brute_force(state: &Vec<u32>, expected: u32) -> NV {
    for n in 0..100 {
        for v in 0..100 {
            let mut cloned = state.clone();
            cloned[1] = n;
            cloned[2] = v;
            run(&mut cloned);
            if cloned[0] == expected {
                return NV {noun : n, verb : v};
            }
        }
    }
    NV {noun : 100, verb : 100}
}

pub fn solve() {

    const FILENAME: &str = "../challenges/2019/input_2019_2.txt";

    let input_text: std::string::String = fs::read_to_string(FILENAME)
                                             .expect("Something went wrong");

    println!("\n#########################################");
    println!("Advent of Code 2019 - Day 2");
    println!("=========================================");

    let mut state = initial_state(&input_text);
    state[1] = 12;
    state[2] = 2;
    run(&mut state);
    println!("Value of End State, Position 0: {}", state[0]);

    let state = initial_state(&input_text);
    let expected = 19690720;
    let NV {noun, verb} = brute_force(&state, expected);
    println!("Value of Noun/Verbs for expected output of {}: {}, {}", expected, noun, verb);
    println!("#########################################");
}

#[cfg(test)]
mod tests {

}
