pub fn run(part: u8, input: String) {
    let histories = input
        .lines()
        .map(|line| line.split(" ").map(|n| n.parse::<i64>().unwrap()).collect())
        .collect::<Vec<Vec<i64>>>();

    let mut sum = 0;

    for history in histories {
        let lines = expand(history);

        let mut next_val = 0;

        for (i, difference_line) in lines.iter().rev().enumerate() {
            if i == 0 {
                next_val = difference_line[0];
            } else {
                if part == 1 {
                    let value = difference_line[difference_line.len() - 1];
                    next_val = value + next_val;
                } else {
                    let value = difference_line[0];
                    next_val = value - next_val;
                }
            }
        }

        sum += next_val;
    }

    println!("{sum}");
}

fn find_difference(line: Vec<i64>) -> Vec<i64> {
    let mut result = Vec::new();

    for i in 1..line.len() {
        result.push(line[i] - line[i - 1]);
    }

    result
}

fn expand(line: Vec<i64>) -> Vec<Vec<i64>> {
    let mut result = vec![line.clone()];
    let mut line = line;

    loop {
        let diff = find_difference(line);

        if diff.iter().all(|&n| n == 0) {
            break;
        }

        result.push(diff.clone());
        line = diff;
    }

    result
}
