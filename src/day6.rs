#[derive(Debug)]
struct Race {
    time_length: u64,
    record: u64,
}

pub fn run(part: u8, input: String) {
    let mut races: Vec<Race> = Vec::new();

    let mut lines = input.lines();

    let time_lengths_line = lines.next().unwrap();

    let time_lengths = match part {
        1 => time_lengths_line
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>(),
        2 => vec![time_lengths_line
            .split_whitespace()
            .skip(1)
            .fold(String::new(), |mut acc, x| {
                acc.push_str(x);
                acc
            })
            .parse::<u64>()
            .unwrap()],
        _ => panic!("Unknown part"),
    };

    let record_line = lines.next().unwrap();

    let records = match part {
        1 => record_line
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>(),
        2 => vec![record_line
            .split_whitespace()
            .skip(1)
            .fold(String::new(), |mut acc, x| {
                acc.push_str(x);
                acc
            })
            .parse::<u64>()
            .unwrap()],
        _ => panic!("Unknown part"),
    };

    time_lengths.iter().enumerate().for_each(|(i, &time)| {
        let record = records[i];

        races.push(Race {
            time_length: time,
            record,
        });
    });

    let optimal_races = races
        .iter()
        .map(|race| {
            let mut ways_to_beat: u64 = 0;
            for speed in 0..=race.time_length {
                let traveled = (race.time_length - speed) * speed;

                if traveled > race.record {
                    ways_to_beat += 1;
                }
            }

            ways_to_beat
        })
        .fold(1, |acc, x| acc * x);

    println!("{optimal_races}");
}
