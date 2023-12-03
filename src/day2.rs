pub fn run(part: u8, input: String) {
    let mut sum = 0;

    for (i, game) in input.lines().enumerate() {
        let cubes = game.split(": ").skip(1).next().unwrap();

        let turns = cubes.split("; ");

        let mut valid = true;
        let mut min_green = 0;
        let mut max_red = 0;
        let mut max_blue = -1;

        for turn in turns {
            let counts = turn.split(", ");

            for count in counts {
                let mut count = count.split(" ");
                let num = count.next().unwrap().parse::<i32>().unwrap();
                let colour = count.next().unwrap();

                match colour {
                    "green" => {
                        if num > 13 && part == 1 {
                            valid = false;
                            break;
                        } else if part == 2 {
                            if min_green == -1 {
                                min_green = num;
                            } else if num > min_green {
                                min_green = num;
                            }
                        }
                    }
                    "red" => {
                        if num > 12 && part == 1 {
                            valid = false;
                            break;
                        } else if part == 2 {
                            if max_red == -1 {
                                max_red = num;
                            } else if num > max_red {
                                max_red = num;
                            }
                        }
                    }
                    "blue" => {
                        if num > 14 && part == 1 {
                            valid = false;
                            break;
                        } else if part == 2 {
                            if max_blue == -1 {
                                max_blue = num;
                            } else if num > max_blue {
                                max_blue = num;
                            }
                        }
                    }
                    _ => {
                        println!("Invalid colour: {}", colour);
                        std::process::exit(1);
                    }
                }
            }

            if !valid {
                break;
            }
        }

        if valid && part == 1 {
            sum += i + 1;
        } else if part == 2 {
            sum += (min_green * max_red * max_blue) as usize;
        }
    }

    println!("{}", sum);
}
