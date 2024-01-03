// include str input.txt
const INPUT: &str = include_str!("../input.txt");

fn main() {
    let sensors = parse(INPUT);
    dbg!(sensors);
}

fn parse(input: &str) -> Vec<Sensor> {
    fn parse_int(s: &str, trailing_chars: usize) -> i32 {
        (s[2..s.len() - trailing_chars]).parse().unwrap()
    }

    let mut sensors = Vec::new();
    for (_line_idx, line) in input.lines().enumerate() {
        let mut words = line.split_ascii_whitespace().skip(2);
        let sensor_pos = Vec2 {
            x: parse_int(words.next().unwrap(), 1),
            y: parse_int(words.next().unwrap(), 1),
        };
        let mut words = words.skip(4);
        let beacon = Vec2 {
            x: parse_int(words.next().unwrap(), 1),
            y: parse_int(words.next().unwrap(), 0),
        };

        let range = sensor_pos.manhattan_distance(&beacon);
        sensors.push(Sensor {
            pos: sensor_pos,
            range,
        });
    }
    sensors
}

#[derive(Debug, Clone)]
struct Sensor {
    pos: Vec2,
    range: i32,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    const fn manhattan_distance(&self, other: &Vec2) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}