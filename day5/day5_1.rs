// Answer is 8622
use std::fs;
use std::cmp;
use std::collections::HashMap;

const FILE_NAME: &str = "day5.txt";
const DANGEROUS_VENT_LIMIT: &u64 = &2;

fn main() {
    let file_contents = fs::read_to_string(FILE_NAME)
        .expect("Something went wrong reading the file");

    let mut parsed_lines: Vec<Line> = Vec::new();

    for line in file_contents.lines() {
        parsed_lines.push(parse_line(line));
    }

    let mut point_vents: HashMap<Point, u64> = HashMap::new();

    for line in parsed_lines {
        for point in line.get_all_points() {
            if point_vents.contains_key(&point) {
                point_vents.insert(point.clone(), point_vents.get(&point).unwrap() + 1 as u64);
            } else {
                point_vents.insert(point.clone(), 1);
            }
        }
    }

    let mut dangerous_vent_count = 0;
    for val in point_vents.values() {
        if val >= DANGEROUS_VENT_LIMIT {
            dangerous_vent_count += 1;
        }
    }

    println!("{}", dangerous_vent_count);

}

fn parse_line(unparsed_line: &str) -> Line {
    let parsed_line: Vec<Vec<&str>> = unparsed_line
        .split(" -> ")
        .map(|coordinate: &str| coordinate
             .split(",")
             .collect())
        .collect();

    let start_point = Point { 
        x: String::from(parsed_line[0][0])
            .parse::<u64>()
            .unwrap(),
        y: String::from(parsed_line[0][1])
            .parse::<u64>()
            .unwrap(),
    };
    let end_point = Point { 
        x: String::from(parsed_line[1][0])
            .parse::<u64>()
            .unwrap(),
        y: String::from(parsed_line[1][1])
            .parse::<u64>()
            .unwrap(),
    };

    return Line { start: start_point, end: end_point };
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Point {
    x: u64,
    y: u64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn max_x(&self) -> u64 {
        return cmp::max(self.start.x, self.end.x);
    }

    fn max_y(&self) -> u64 {
        return cmp::max(self.start.y, self.end.y);
    }


    fn min_x(&self) -> u64 {
        return cmp::min(self.start.x, self.end.x);
    }

    fn min_y(&self) -> u64 {
        return cmp::min(self.start.y, self.end.y);
    }

    fn get_all_points(&self) -> Vec<Point> {
        let mut all_points: Vec<Point> = Vec::new();

        if self.start == self.end {
            return all_points;
        } else if self.start.x == self.end.x {
            for y in self.min_y()..=self.max_y() {
                all_points.push(Point { x: self.start.x, y: y } );
            }
        } else if self.start.y == self.end.y {
            for x in self.min_x()..=self.max_x() {
                all_points.push(Point { x: x, y: self.start.y } );
            }
        }

        return all_points;
    }
}
