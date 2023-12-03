pub fn run(part: u8, input: String) {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut nums: Vec<u32> = vec![];
        let mut buff: String = String::new();
        let mut i = 0;

        while i < line.len() {
            let ch = line.chars().nth(i).unwrap();

            if let Some(num) = ch.to_digit(10) {
                nums.push(num);
                i += 1;
                continue;
            }

            if part == 2 {
                buff.push(ch);

                if buff.contains("one") {
                    nums.push(1);
                    buff.clear();
                    i -= 1;
                } else if buff.contains("two") {
                    nums.push(2);
                    buff.clear();
                    i -= 1;
                } else if buff.contains("three") {
                    nums.push(3);
                    buff.clear();
                    i -= 1;
                } else if buff.contains("four") {
                    nums.push(4);
                    buff.clear();
                    i -= 1;
                } else if buff.contains("five") {
                    nums.push(5);
                    buff.clear();
                    i -= 1;
                } else if buff.contains("six") {
                    nums.push(6);
                    buff.clear();
                    i -= 1;
                } else if buff.contains("seven") {
                    nums.push(7);
                    buff.clear();
                    i -= 1;
                } else if buff.contains("eight") {
                    nums.push(8);
                    buff.clear();
                    i -= 1;
                } else if buff.contains("nine") {
                    nums.push(9);
                    buff.clear();
                    i -= 1;
                }
            }

            i += 1;
        }

        let first = nums[0];
        let last = nums[nums.len() - 1];

        sum += first * 10 + last;
    }

    println!("{}", sum);
}
