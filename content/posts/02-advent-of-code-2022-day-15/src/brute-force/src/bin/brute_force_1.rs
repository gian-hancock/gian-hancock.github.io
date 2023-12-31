/*
 * It takes 1.9s to run 1e8 iterations. We need to run 4e6 * 4e6 = 1.6e13 iters.
 * 1.9s * (1.6e13/1e8) = 304000s = 3.5 days
 */

fn main() {
    let max_iters = std::env::args()
        .nth(1)
        .expect("Please provide max_iters as first argument")
        .parse::<usize>()
        .expect("Failed to parse max_iters as integer");
    dbg!(solve_by_brute_force(&mut SENSORS.clone(), 4_000_000, max_iters));
}

fn solve_by_brute_force(sensors: &mut [Sensor], dimension: i32, max_iters: usize) -> (Vec2, i64) {
    let mut count = 0;
    for x in 0..dimension {
        for y in 0..dimension {
            if count >= max_iters {
                panic!("Too many iterations: {count}");
            }
            count += 1;
            let mut is_solution = true;
            for sensor in sensors.iter() {
                if (sensor.pos.x - x).abs() + (sensor.pos.y - y).abs() <= sensor.range {
                    is_solution = false;
                    break;
                }
            }
            if is_solution {
                let answer = x as i64 * dimension as i64 + y as i64;
                return (Vec2 { x, y }, answer);
            }
        }
    }
    unreachable!("No solution found")
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
