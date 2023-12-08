use std::collections::HashMap;

pub fn run(part: u8, input: String) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<char>>();

    let mut paths: HashMap<&str, (String, String)> = HashMap::new();

    lines.enumerate().for_each(|(i, line)| {
        if i == 0 {
            return;
        }
        let mut parts = line.split(" = ");
        let path = parts.next().unwrap();
        let mut sides = parts.next().unwrap().split(", ");
        let left = sides.next().unwrap().replace("(", "");
        let right = sides.next().unwrap().replace(")", "");

        paths.insert(path, (left, right));
    });

    match part {
        1 => part1(&instructions, &paths),
        2 => part2(&instructions, &paths),
        _ => (),
    }
}

fn part1(instructions: &Vec<char>, paths: &HashMap<&str, (String, String)>) {
    let mut current = "AAA";

    let mut i = 1;

    while i < u128::MAX {
        let (left, right) = paths.get(current).unwrap();

        let instruction = instructions[(i as usize - 1) % instructions.len()];

        if instruction == 'L' {
            current = left;
        } else {
            current = right;
        }

        if current == "ZZZ" {
            break;
        }

        i += 1;
    }

    println!("{i}");
}

fn part2(instructions: &Vec<char>, paths: &HashMap<&str, (String, String)>) {
    let starting = paths
        .iter()
        .map(|(k, _)| *k)
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<&str>>();

    let at_z_in = starting
        .iter()
        .map(|start| {
            let mut current = *start;
            let mut i = 1;

            while i < u128::MAX {
                let (left, right) = paths.get(current).unwrap();

                let instruction = instructions[(i as usize - 1) % instructions.len()];

                if instruction == 'L' {
                    current = left;
                } else {
                    current = right;
                }

                if current.ends_with("Z") {
                    break;
                }

                i += 1;
            }

            i
        })
        .collect::<Vec<u128>>();

    fn greatest_common_denominator(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    fn lowest_common_multiple(vec: Vec<u128>) -> u128 {
        let mut result = vec[0];
        for i in 1..vec.len() {
            result = (result * vec[i]) / greatest_common_denominator(result, vec[i]);
        }
        result
    }

    let i = lowest_common_multiple(at_z_in);

    println!("{i}");
}
