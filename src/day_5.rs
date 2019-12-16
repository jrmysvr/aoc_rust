use std::fs;
use std::io::{stdin,stdout, Write};

fn initial_state(input_text: &str) -> Vec<i32> {
    input_text.trim()
              .split(",")
              .map(|s| s.parse().unwrap())
              .collect()
}

fn get_input_value() -> i32 {
    print!("Input a Value (integer): ");
    let _ = stdout().flush();
    let mut s = String::new();
    stdin().read_line(&mut s).expect("Whoops");

    s.trim().parse::<i32>().unwrap()
}

fn parse_code_param_modes(value: i32) -> (u32, Vec<i32>) {
    let code = (value % 100) as u32;
    let mut param_value = value / 100;
    let mut params = Vec::new();
    while param_value >= 1 {
        params.push(param_value % 10);
        param_value /= 10;
    }
    while params.len() < 3 {
        params.push(0);
    }

    (code, params)
}

fn run(state: &mut Vec<i32>) {

    let mut i = 0;

    while i < state.len() {
        let (code, modes) = parse_code_param_modes(state[i]);
        if code == 99 { break };

        let ix1 = state[i+1] as usize;
        let param1 = if modes[0] == 0 { state[ix1] } else { ix1 as i32};

        //// STDIN/STDOUT Codes
        // Code take user input
        if code == 3 {
            state[ix1] = get_input_value();
            i += 2;
            continue;
        // Code to output state value
        }else if code == 4 {
            println!("Code 4 output: {}", param1);
            i += 2;
            continue;
        }

        let ix2 = state[i+2] as usize;
        let param2 = if modes[1] == 0 { state[ix2] } else { ix2 as i32};

        //// Jump Codes
        // Jump if non-zero
        if code == 5 {
            i = if param1 != 0 { param2 as usize } else { i + 3 };
            continue;
        // Jump if zero
        }else if code == 6 {
            i = if param1 == 0 { param2 as usize } else { i + 3 };
            continue;
        }

        let wix = state[i+3] as usize;

        //// Read/Write Codes
        // Add instruction
        if code == 1 {
            let value = param1 + param2;
            state[wix] = value;
            i += 4;
        // Multiply Instruction
        }else if code == 2 {
            let value = param1 * param2;
            state[wix] = value;
            i += 4;
        // Check if param1 < param2
        }else if code == 7 {
            state[wix] = if param1 < param2 { 1 } else { 0 };
            i += 4
        // Check if param1 == param2
        }else if code == 8 {
            state[wix] = if param1 == param2 { 1 } else { 0 };
            i += 4
        }else {
            panic!("Wrong code: {}", code);
        }
    }
}

pub fn solve() {

    const FILENAME: &str = "../challenges/2019/input_2019_5.txt";

    let input_text: std::string::String = fs::read_to_string(FILENAME)
                                             .expect("Something went wrong");

    println!("\n#########################################");
    println!("Advent of Code 2019 - Day 5");
    println!("=========================================");

    let mut state = initial_state(&input_text);
    run(&mut state);
    println!("Value of End State, Position 0: {}", state[0]);

    println!("#########################################");
}

#[warn(dead_code)]
fn main() {
    //// Test if input to program is equal to 8 (output 1 if true, 0 if false)
    // let mut test: Vec<i32> = vec![3,9,8,9,10,9,4,9,99,-1,8];

    //// Test if input to program is less than 8 (output 1 if true, 0 if false)
    // let mut test: Vec<i32> = vec![3,9,7,9,10,9,4,9,99,-1,8];

    //// Test if input to program is equal to 8 (output 1 if true, 0 if false)
    // let mut test: Vec<i32> = vec![3,3,1108,-1,8,3,4,3,99];

    //// Test if input to program is less than 8 (output 1 if true, 0 if false)
    // let mut test: Vec<i32> = vec![3,3,1107,-1,8,3,4,3,99];

    //// Test if the input to the program was non-zero (output 1 if non-zero, 0 if zero)
    // let mut test: Vec<i32> = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    // let mut test: Vec<i32> = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];

    ////The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8.
    let mut test: Vec<i32> = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                                  1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                                  999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    run(&mut test);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_values() {
        let (_, modes) = parse_code_param_modes(1101);
        assert_eq!(modes[0], 1);
        assert_eq!(modes[1], 1);

        let (_, modes) = parse_code_param_modes(101);
        assert_eq!(modes[0], 1);
        assert_eq!(modes.len(), 1);
    }
}
