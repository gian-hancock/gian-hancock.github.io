/* Visualise an AoC 2022 day 15 input as an SVG. This is pretty hacky, it doesn't actually output an 
 * SVG file, but instead prints SVG elements for the sensor polygons to the console. I then copy 
 * paste this output into a handwritten SVG file... it gets the job done. 
 */
fn main() {
    let colours = vec![
        "#ff0080",
        "#ff8000",
        "#ff0000",
        "#00ffaa",
        "#0040ff",
        "#006aff",
        "#d4ff00",
        "#ffbf00",
        "#ff00ff",
        "#ff002b",
        "#40ff00",
        "#aa00ff",
        "#95ff00",
        "#0095ff",
        "#00eaff",
        "#ffea00",
        "#0000ff",
        "#00ffea",
        "#ff00bf",
        "#00bfff",
    ];
    let input = include_str!("aoc-input.txt");

    for (line, color) in input.lines().zip(colours.iter().cycle()) {
        // Ugly parsing
        let mut words = line.split_ascii_whitespace().skip(2);
        let sensor = Vec2 {
            x: parse_int(words.next().unwrap(), 1),
            y: parse_int(words.next().unwrap(), 1),
        };
        let mut words = words.skip(4);
        let beacon = Vec2 {
            x: parse_int(words.next().unwrap(), 1),
            y: parse_int(words.next().unwrap(), 0),
        };
        let range = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

        let top = Vec2 {
            x: sensor.x,
            y: sensor.y + range,
        };
        let bottom = Vec2 {
            x: sensor.x,
            y: sensor.y - range,
        };
        let left = Vec2 {
            x: sensor.x - range,
            y: sensor.y,
        };
        let right = Vec2 {
            x: sensor.x + range,
            y: sensor.y,
        };

        let solution = Vec2 { x: 3270298, y: 2638237 };
        let distance = solution.manhattan_distance(&sensor);
        let print_all = false;
        if (distance == range + 1 || print_all) {
            println!("<polygon points=\"{},{},{},{},{},{},{},{}\" fill=\"{}\" fill-opacity=\"0.25\" stroke=\"black\" stroke-width=\"0.1%\" />", top.x, top.y, right.x, right.y, bottom.x, bottom.y, left.x, left.y, color);
        }
    }
}


fn parse_int(s: &str, trailing_chars: usize) -> i32 {
    s[2..s.len() - trailing_chars].parse().unwrap()
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const fn manhattan_distance(&self, other: &Vec2) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub const fn sub(&self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}