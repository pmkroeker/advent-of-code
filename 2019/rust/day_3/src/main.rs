fn main() {
    let input_string = include_str!("input.txt");
    println!("Part 1: {}", part_1(input_string.to_string()));
    println!("Part 2: {}", part_2(input_string.to_string()));
}

type Coordinate = (i32, i32);
type LineSegment = (Coordinate, Coordinate);
type Distance = i32;

fn calc_manhattan_dist(start: Coordinate, end: Coordinate) -> Distance {
    let (x1 , y1) = start;
    let (x2 , y2) = end;
    ((x1 - x2).abs() + (y1 - y2).abs())
}

fn process_wires(input_str: String) -> Vec<Vec<LineSegment>> {
    let inputs: Vec<&str> = input_str.trim().split_whitespace().collect();

    let wires: Vec<Vec<LineSegment>> = inputs
        .into_iter()
        .map(|wire: &str| {
            let mut current_coord: Coordinate = (0, 0);

            let line_segments: Vec<LineSegment> = wire
                .trim()
                .split(',')
                .map(|instructions: &str| {
                    let instructions = instructions.trim();

                    let direction: char = instructions.chars().next().unwrap();
                    let steps: String = instructions.chars().skip(1).collect();
                    let steps: u32 = steps.parse().unwrap();

                    let previous_coord = current_coord;

                    match direction {
                        'U' => {
                            let (x, y) = current_coord;
                            current_coord = (x, y + (steps as i32));
                        }
                        'D' => {
                            let (x, y) = current_coord;
                            current_coord = (x, y - (steps as i32));
                        }
                        'L' => {
                            let (x, y) = current_coord;
                            current_coord = (x - (steps as i32), y);
                        }
                        'R' => {
                            let (x, y) = current_coord;
                            current_coord = (x + (steps as i32), y);
                        }
                        _ => {
                            panic!("Unknown direction: {}", direction);
                        }
                    }

                    assert_eq!(calc_manhattan_dist(previous_coord, current_coord), steps as i32);
                    (previous_coord, current_coord)
                })
                .collect();
            line_segments
        }).collect();
    wires
}

fn line_segments_intersection(first_segment: LineSegment, second_segment: LineSegment) -> Option<Coordinate> {
    let (point_1, point_2) = first_segment;
    let (point_3, point_4) = second_segment;

    let (x_1, y_1) = point_1;
    let (x_2, y_2) = point_2;
    let (x_3, y_3) = point_3;
    let (x_4, y_4) = point_4;

    let parameter_1_numerator = (y_3 - y_4) * (x_1 - x_3) + (x_4 - x_3) * (y_1 - y_3);
    let parameter_1_denominator = (x_4 - x_3) * (y_1 - y_2) - (x_1 - x_2) * (y_4 - y_3);

    let parameter_2_numerator = (y_1 - y_2) * (x_1 - x_3) + (x_2 - x_1) * (y_1 - y_3);
    let parameter_2_denominator = (x_4 - x_3) * (y_1 - y_2) - (x_1 - x_2) * (y_4 - y_3);

    if parameter_1_denominator == 0 || parameter_2_denominator == 0 {
        return None;
    }

    let parameter_1: f64 = parameter_1_numerator as f64 / parameter_1_denominator as f64;
    let parameter_2: f64 = parameter_2_numerator as f64 / parameter_2_denominator as f64;

    if (0.0 <= parameter_1 && parameter_1 <= 1.0) && (0.0 <= parameter_2 && parameter_2 <= 1.0) {
        let x = x_1 as f64 + parameter_1 * (x_2 as f64 - x_1 as f64);
        let y = y_1 as f64 + parameter_1 * (y_2 as f64 - y_1 as f64);

        return Some((x as i32, y as i32));
    }
    None
}

fn part_1(input_str: String) -> Distance {
    let wires: Vec<Vec<LineSegment>> = process_wires(input_str);
    assert!(wires.len() == 2);
    let wire_1: Vec<LineSegment> = wires[0].clone();
    let wire_2: Vec<LineSegment> = wires[1].clone();

    let mut intersections: Vec<Coordinate> = vec![];

    for segment_1 in wire_1 {
        for segment_2 in &wire_2 {
            match line_segments_intersection(segment_1, *segment_2) {
                None => {
                    continue;
                }
                Some(coord) => {
                    if coord == (0, 0) {
                        continue;
                    }
                    intersections.push(coord);
                }
            }
        }
    }

    let closest_intersection_to_port: Distance = intersections
        .into_iter()
        .map(|coord| {
            calc_manhattan_dist((0, 0), coord)
        })
        .min()
        .unwrap();

    closest_intersection_to_port
}

fn part_2(input_string: String) -> i32 {
    let wires: Vec<Vec<LineSegment>> = process_wires(input_string);
    assert!(wires.len() >= 2);
    let wire_1: Vec<LineSegment> = wires[0].clone();
    let wire_2: Vec<LineSegment> = wires[1].clone();

    let mut steps_to_reach_intersections: Vec<i32> = vec![];

    let mut steps_wire_1 = 0;

    for segment_1 in wire_1 {
        let (segment_1_start, segment_1_end) = segment_1;
        steps_wire_1 += calc_manhattan_dist(segment_1_start, segment_1_end);

        let mut steps_wire_2 = 0;

        for segment_2 in wire_2.iter() {
            let (segment_2_start, segment_2_end) = segment_2;
            steps_wire_2 += calc_manhattan_dist(*segment_2_start, *segment_2_end);

            match line_segments_intersection(segment_1.clone(), segment_2.clone()) {
                None => {
                    continue;
                }
                Some(intersection_coord) => {
                    if intersection_coord == (0, 0) {
                        continue;
                    }

                    // need to backtrack the amount of steps

                    let steps_wire_1_intersection =
                        steps_wire_1 - calc_manhattan_dist(intersection_coord, segment_1_end);
                    let steps_wire_2_intersection =
                        steps_wire_2 - calc_manhattan_dist(intersection_coord, *segment_2_end);

                    steps_to_reach_intersections
                        .push(steps_wire_1_intersection + steps_wire_2_intersection);
                }
            }
        }
    }

    let fewest_combined_steps: i32 = steps_to_reach_intersections.into_iter().min().unwrap();

    fewest_combined_steps
}
