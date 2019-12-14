use std::fs;
use std::cmp;
use std::string;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Move {
    row: i32,
    col: i32
}

fn manhattan_distance(point_1: &Point, point_2: &Point) -> u32 {
     let x = point_2.x - point_1.x;
     let y = point_2.y - point_2.y;
     (x + y).abs() as u32
}

fn _split_input_text(input_text: &str) -> (&str, &str) {
    let split: Vec<&str> = input_text.trim().split("\n").collect();
    (split[0], split[1])
}

fn parse_wire_routes(input_text: &str) -> (Vec<&str>, Vec<&str>) {
    let (wire_1, wire_2) = _split_input_text(input_text);

    let route_1 = wire_1.split(",")
                        .collect();

    let route_2 = wire_2.split(",")
                        .collect();

    (route_1, route_2)
}

fn get_move(direction: &str) -> &Move {
    return match direction {
        "U" => &Move { row:  0,  col: -1 },
        "R" => &Move { row:  1,  col:  0 },
        "D" => &Move { row:  0,  col:  1 },
        "L" => &Move { row: -1,  col:  0 },
        _ => panic!("Unknown Direction")
    }
}

fn route_to_move_vector(route: &str) -> Vec<&Move> {
    let direction = &route[0..1];
    let length: u32 = route[1..].parse().unwrap();
    let mut output = Vec::new();

    let mv = get_move(direction);
    for _ in 0..length {
        output.push(mv);
    }

    output
}

fn trace_wire_route(wire_route: &Vec<&str>) -> Point {
    let mut point = Point { x: 0, y: 0 };
    for movements in wire_route.iter().map(|r| route_to_move_vector(r)) {
        for mv in movements {
            point.x += mv.row;
            point.y += mv.col;
        }
    }

    point
}

pub fn solve() {
        const FILENAME: &str = "../challenges/2019/input_2019_3.txt";
        let input_text: string::String = fs::read_to_string(FILENAME)
                                            .expect("Something went wrong");
        let (route_1, route_2) = parse_wire_routes(&input_text);
        let end_1 = trace_wire_route(&route_1);
        let end_2 = trace_wire_route(&route_2);

        let start = Point { x: 0, y: 0 };

        let min_distance = cmp::min(manhattan_distance(&start, &end_1),
                                    manhattan_distance(&start, &end_2));

        println!("\n#########################################");
        println!("Advent of Code 2019 - Day 3");
        println!("=========================================");
        println!("The manhattan distance to the closest endpoint is {}", min_distance);
        println!("#########################################");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance() {

        const POINT_1: Point = Point {x: 0, y: 0};
        const POINT_2: Point = Point {x: 1, y: 1};
        const POINT_3: Point = Point {x: 1, y: 0};
        const POINT_4: Point = Point {x: 1, y: 0};

        let distance = manhattan_distance(&POINT_1, &POINT_2);
        assert_eq!(distance, 2);

        let distance = manhattan_distance(&POINT_1, &POINT_3);
        assert_eq!(distance, 1);

        let distance = manhattan_distance(&POINT_1, &POINT_4);
        assert_eq!(distance, 1);
    }

    #[test]
    fn test_manhattan_distance_reverse() {
        const POINT_1: Point = Point {x: 0, y: 0};
        const POINT_2: Point = Point {x: 1, y: 1};
        const POINT_3: Point = Point {x: 1, y: 0};
        const POINT_4: Point = Point {x: 1, y: 0};

        let distance = manhattan_distance(&POINT_2, &POINT_1);
        assert_eq!(distance, 2);

        let distance = manhattan_distance(&POINT_3, &POINT_1);
        assert_eq!(distance, 1);

        let distance = manhattan_distance(&POINT_4, &POINT_1);
        assert_eq!(distance, 1);
    }
}
