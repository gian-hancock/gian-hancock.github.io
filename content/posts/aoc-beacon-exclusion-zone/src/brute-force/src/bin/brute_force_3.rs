fn main() {
    dbg!(solve_by_brute_force(&mut SENSORS.clone(), DIMENSION));
}

fn solve_by_brute_force(sensors: &mut [Sensor], dimension: i32) -> Vec2 {
    // Sort sensors by ascending upper y bound.
    sensors.sort_by(|a, b| (a.pos.y + a.range).cmp(&(b.pos.y + b.range)));
    let mut candidate_sensors = Vec::new();

    let mut y = 0;
    let mut x = 0;
    let mut coords_checked = 0;
    let mut ranges_checked = 0;
    init_candidate_sensors(y, sensors, &mut candidate_sensors);
    while y < dimension {
        while x < dimension {
            coords_checked += 1;
            let mut is_solution = true;
            for sensor in candidate_sensors.iter() {
                ranges_checked += 1;
                let distance = (sensor.pos.x - x).abs() + (sensor.pos.y - y).abs();
                if distance <= sensor.range {
                    // Skip along all x values covered by this sensor on the current row
                    x = sensor.pos.x + sensor.range - (sensor.pos.y - y).abs();
                    is_solution = false;
                    break;
                }
            }
            if is_solution {
                println!("Coords checked: {}", coords_checked);
                println!("Ranges checked: {}", ranges_checked);
                return Vec2 { x, y };
            }
            x += 1;
        }
        y += 1;
        init_candidate_sensors(y, sensors, &mut candidate_sensors);
        x = 0;
    }
    unreachable!("No solution found");

    fn init_candidate_sensors<'a>(y: i32, sensors: &'a [Sensor], candidate_sensors: &mut Vec<&'a Sensor>) {
        candidate_sensors.clear();
        for sensor in sensors {
            let lower_bound = sensor.pos.y - sensor.range;
            let upper_bound = sensor.pos.y + sensor.range;
            if lower_bound <= y && upper_bound >= y {
                candidate_sensors.push(&sensor);
            }
        }
    }
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

/*
const DIMENSION: i32 = 4;
const SENSORS: [Sensor; 1] = [
    Sensor {
        pos: Vec2 {
            x: 1,
            y: 1,
        },
        range: 3,
    },
];
*/
const DIMENSION: i32 = 4_000_000;
const SENSORS: [Sensor; 40] = [
    Sensor {
        pos: Vec2 {
            x: 1112863,
            y: 496787,
        },
        range: 1595476,
    },
    Sensor {
        pos: Vec2 {
            x: 2980210,
            y: 1712427,
        },
        range: 33563,
    },
    Sensor {
        pos: Vec2 {
            x: 2799204,
            y: 1425283,
        },
        range: 434943,
    },
    Sensor {
        pos: Vec2 {
            x: 3999908,
            y: 2754283,
        },
        range: 166993,
    },
    Sensor {
        pos: Vec2 {
            x: 760990,
            y: 1455625,
        },
        range: 803985,
    },
    Sensor {
        pos: Vec2 {
            x: 3996490,
            y: 3239979,
        },
        range: 656107,
    },
    Sensor {
        pos: Vec2 {
            x: 3347352,
            y: 3603589,
        },
        range: 285495,
    },
    Sensor {
        pos: Vec2 {
            x: 2888433,
            y: 2337157,
        },
        range: 682944,
    },
    Sensor {
        pos: Vec2 {
            x: 3423261,
            y: 2191958,
        },
        range: 599241,
    },
    Sensor {
        pos: Vec2 {
            x: 1160237,
            y: 3999960,
        },
        range: 1465582,
    },
    Sensor {
        pos: Vec2 {
            x: 693519,
            y: 3701289,
        },
        range: 700193,
    },
    Sensor {
        pos: Vec2 {
            x: 2615270,
            y: 2824808,
        },
        range: 171414,
    },
    Sensor {
        pos: Vec2 {
            x: 3046971,
            y: 1755494,
        },
        range: 143035,
    },
    Sensor {
        pos: Vec2 {
            x: 139591,
            y: 1186912,
        },
        range: 1694097,
    },
    Sensor {
        pos: Vec2 {
            x: 2309134,
            y: 47090,
        },
        range: 1742448,
    },
    Sensor {
        pos: Vec2 {
            x: 1849154,
            y: 1377259,
        },
        range: 1433017,
    },
    Sensor {
        pos: Vec2 {
            x: 2515971,
            y: 2851853,
        },
        range: 121372,
    },
    Sensor {
        pos: Vec2 {
            x: 2524614,
            y: 2738138,
        },
        range: 226444,
    },
    Sensor {
        pos: Vec2 {
            x: 3811778,
            y: 1370280,
        },
        range: 1150020,
    },
    Sensor {
        pos: Vec2 {
            x: 2615590,
            y: 3819371,
        },
        range: 945765,
    },
    Sensor {
        pos: Vec2 {
            x: 3996286,
            y: 3719213,
        },
        range: 479063,
    },
    Sensor {
        pos: Vec2 {
            x: 3963152,
            y: 2368927,
        },
        range: 383561,
    },
    Sensor {
        pos: Vec2 {
            x: 3495504,
            y: 3076982,
        },
        range: 663950,
    },
    Sensor {
        pos: Vec2 {
            x: 3725521,
            y: 2560764,
        },
        range: 429355,
    },
    Sensor {
        pos: Vec2 {
            x: 952643,
            y: 2385401,
        },
        range: 453358,
    },
    Sensor {
        pos: Vec2 {
            x: 3934384,
            y: 2596106,
        },
        range: 185150,
    },
    Sensor {
        pos: Vec2 {
            x: 3060628,
            y: 3082730,
        },
        range: 654162,
    },
    Sensor {
        pos: Vec2 {
            x: 3468382,
            y: 3916817,
        },
        range: 455679,
    },
    Sensor {
        pos: Vec2 {
            x: 3300107,
            y: 469364,
        },
        range: 1350301,
    },
    Sensor {
        pos: Vec2 {
            x: 2306388,
            y: 1932261,
        },
        range: 860093,
    },
    Sensor {
        pos: Vec2 {
            x: 1965,
            y: 3514070,
        },
        range: 178580,
    },
    Sensor {
        pos: Vec2 {
            x: 3081537,
            y: 1841861,
        },
        range: 92580,
    },
    Sensor {
        pos: Vec2 {
            x: 2997643,
            y: 1729779,
        },
        range: 67992,
    },
    Sensor {
        pos: Vec2 {
            x: 21714,
            y: 3624181,
        },
        range: 126158,
    },
    Sensor {
        pos: Vec2 {
            x: 1549467,
            y: 3109269,
        },
        range: 1178850,
    },
    Sensor {
        pos: Vec2 {
            x: 3722307,
            y: 3839410,
        },
        range: 325281,
    },
    Sensor {
        pos: Vec2 {
            x: 3848580,
            y: 3544878,
        },
        range: 296458,
    },
    Sensor {
        pos: Vec2 {
            x: 1189516,
            y: 2153239,
        },
        range: 322155,
    },
    Sensor {
        pos: Vec2 {
            x: 468190,
            y: 1889204,
        },
        range: 663206,
    },
    Sensor {
        pos: Vec2 {
            x: 270403,
            y: 2762568,
        },
        range: 984144,
    },
];