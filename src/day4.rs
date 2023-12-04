pub fn run(part: u8, input: String) {
    let mut sum: u32 = 0;

    let mut on_hand: Vec<u32> = vec![];

    for (i, line) in input.lines().enumerate() {
        if on_hand.len() <= i {
            on_hand.push(1);
        } else {
            on_hand[i] += 1;
        }

        let mut count: u32 = 0;

        let data = line
            .split(":")
            .nth(1)
            .unwrap()
            .split(" | ")
            .collect::<Vec<&str>>();

        let winning = data[0]
            .split_whitespace()
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let own = data[1].split_whitespace().map(|x| x.parse::<u8>().unwrap());

        for num in own {
            if winning.contains(&num) {
                count += 1;
            }
        }

        if part == 1 {
            let mut score = 0;

            for j in 0..count {
                if j == 0 {
                    score += 1;
                } else {
                    score *= 2;
                }
            }

            sum += score;
        } else if part == 2 {
            for j in 1..=count {
                let next = i + j as usize;
                if on_hand.len() <= next {
                    on_hand.push(on_hand[i]);
                } else {
                    on_hand[next] += on_hand[i];
                }
            }
        }
    }

    if part == 1 {
        println!("{}", sum);
    } else if part == 2 {
        let mut total: u32 = 0;

        for i in on_hand.iter() {
            total += *i as u32;
        }

        println!("{}", total);
    }
}
