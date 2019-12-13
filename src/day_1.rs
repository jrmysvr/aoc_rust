use std::fs;

#[warn(dead_code)]
fn parse_masses(input_text: &str) -> Vec<u32> {
    input_text.trim()
              .split("\n")
              .map(|s| s.parse().unwrap())
              .collect()
}

#[warn(dead_code)]
fn calculate_fuel(mass: u32) -> u32 {
    let fuel = if mass >= 6 { (mass / 3) - 2 } else { 0 };

    fuel
}

#[warn(dead_code)]
fn calculate_fuel_recur(mass: u32) -> u32 {
    let fuel = calculate_fuel(mass);
    if fuel > 0 {
        return fuel + calculate_fuel_recur(fuel)
    } else {
        return 0
    }
}


#[warn(dead_code)]
pub fn solve() {
    const FILENAME: &str = "../challenges/2019/input_2019_1.txt";
    let input_text: std::string::String = fs::read_to_string(FILENAME)
                                               .expect("Something went wrong");
    let masses: Vec<u32> = parse_masses(&input_text);

    let part_1_result: u32 = masses.iter()
                                   .map(|m| calculate_fuel(*m))
                                   .sum();

    let part_2_result: u32 = masses.iter()
                                   .map(|m| calculate_fuel_recur(*m))
                                   .sum();

    println!("\n#########################################");
    println!("Advent of Code 2019 - Day 1");
    println!("=========================================");
    println!("Fuel Requirements (Part 1): {}", part_1_result);

    println!("Fuel Requirements (Part 2): {}", part_2_result);
    println!("#########################################");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_masses_splits_string() {
        let input_text = "101\n100";
        let output = parse_masses(input_text);
        assert_eq!(output.len(), 2);
    }

    #[test]
    fn test_parse_masses_trims_string() {
        let input_text = "101\n100\n ";
        let output = parse_masses(input_text);
        assert_eq!(output.len(), 2);
    }


    #[test]
    fn test_parse_masses_returns_vector_of_integers() {
        let input_text = "100\n100\n100";
        let output = parse_masses(input_text);
        for mass in output.iter() {
            assert_eq!(*mass, 100);
        }
    }

    #[test]
    fn test_calculate_fuel_given_30_returns_8() {
        let mass = 30;
        let fuel = calculate_fuel(mass);
        assert_eq!(fuel, 8);
    }

    #[test]
    fn test_calculate_fuel_given_31_returns_8() {
        let mass = 31;
        let fuel = calculate_fuel(mass);
        assert_eq!(fuel, 8);
    }

    #[test]
    fn test_calculate_fuel_given_29_returns_7() {
        let mass = 29;
        let fuel = calculate_fuel(mass);
        assert_eq!(fuel, 7);
    }

    #[test]
    fn test_calculate_fuel_given_0_returns_0() {
        let mass = 0;
        let fuel = calculate_fuel(mass);
        assert_eq!(fuel, 0);
    }

    #[test]
    fn test_calculate_fuel_recur_given_30_returns_8() {
        let mass = 30;
        let fuel = calculate_fuel_recur(mass);
        assert_eq!(fuel, 8);
    }


    #[test]
    fn test_calculate_fuel_recur_given_31_returns_8() {
        let mass = 31;
        let fuel = calculate_fuel_recur(mass);
        assert_eq!(fuel, 8);
    }

    #[test]
    fn test_calculate_fuel_recur_given_29_returns_7() {
        let mass = 29;
        let fuel = calculate_fuel_recur(mass);
        assert_eq!(fuel, 7);

    }

    #[test]
    fn test_calculate_fuel_recur_given_0_returns_0() {
        let mass = 0;
        let fuel = calculate_fuel_recur(mass);
        assert_eq!(fuel, 0);
    }


    #[test]
    fn test_calculate_fuel_recur_given_60_returns_22() {
        let mass = 60;
        let fuel = calculate_fuel_recur(mass);
        assert_eq!(fuel, 22);
    }


}
