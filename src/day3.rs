use std::collections::HashMap;
use std::vec;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Location {
    line: usize,
    column: usize,
}

struct Number {
    start: Location,
    length: usize,
    value: u32,
}

struct NeighbouringChar {
    location: Location,
    character: char,
}

impl Number {
    fn get_neighbouring_chars(&self, input: String) -> Vec<NeighbouringChar> {
        let mut chars: Vec<NeighbouringChar> = vec![];

        let line_above = if self.start.line > 0 {
            Some(input.lines().nth(self.start.line - 1).unwrap())
        } else {
            None
        };

        let line_middle = input.lines().nth(self.start.line).unwrap();

        let line_below = if self.start.line < input.lines().count() - 1 {
            Some(input.lines().nth(self.start.line + 1).unwrap())
        } else {
            None
        };

        let start_column = if self.start.column == 0 {
            0
        } else {
            self.start.column - 1
        };

        let end_column = if self.start.column + self.length
            == input.lines().nth(self.start.line).unwrap().len()
        {
            input.lines().nth(self.start.line).unwrap().len()
        } else {
            self.start.column + self.length + 1
        };

        if let Some(line) = line_above {
            for (column_num, ch) in line.chars().enumerate() {
                if column_num >= start_column && column_num < end_column {
                    chars.push(NeighbouringChar {
                        location: Location {
                            line: self.start.line - 1,
                            column: column_num,
                        },
                        character: ch,
                    });
                }
            }
        }

        for (column_num, ch) in line_middle.chars().enumerate() {
            if column_num >= start_column && column_num < end_column {
                if column_num < self.start.column || column_num >= self.start.column + self.length {
                    chars.push(NeighbouringChar {
                        location: Location {
                            line: self.start.line,
                            column: column_num,
                        },
                        character: ch,
                    });
                }
            }
        }

        if let Some(line) = line_below {
            for (column_num, ch) in line.chars().enumerate() {
                if column_num >= start_column && column_num < end_column {
                    chars.push(NeighbouringChar {
                        location: Location {
                            line: self.start.line + 1,
                            column: column_num,
                        },
                        character: ch,
                    });
                }
            }
        }

        return chars;
    }
}

pub fn run(part: u8, input: String) {
    let mut numbers: Vec<Number> = vec![];
    let mut skip_until: usize = 0;

    for (line_num, line) in input.lines().enumerate() {
        for (column_num, ch) in line.chars().enumerate() {
            if column_num < skip_until {
                continue;
            }
            if ch.is_digit(10) {
                let mut length = 1;
                let mut value = ch.to_digit(10).unwrap();

                while column_num + length < line.len() {
                    let next = line.chars().nth(column_num + length).unwrap();
                    if next.is_digit(10) {
                        value = value * 10 + next.to_digit(10).unwrap();
                        length += 1;
                    } else {
                        break;
                    }
                }

                numbers.push(Number {
                    start: Location {
                        line: line_num,
                        column: column_num,
                    },
                    length: length,
                    value: value,
                });
                skip_until = column_num + length;
            }
        }
        skip_until = 0;
    }

    if part == 1 {
        let mut sum = 0;

        for number in numbers {
            if number
                .get_neighbouring_chars(input.clone())
                .iter()
                .any(|x| x.character != '.')
            {
                sum += number.value;
            }
        }

        println!("{}", sum);
    } else if part == 2 {
        struct Star {
            neighbour_count: u32,
            value: u32,
        }

        let mut stars: HashMap<Location, Star> = HashMap::new();

        for number in numbers {
            for neighbour in number.get_neighbouring_chars(input.clone()) {
                if neighbour.character == '*' {
                    if stars.contains_key(&neighbour.location) {
                        let star = stars.get_mut(&neighbour.location).unwrap();
                        star.neighbour_count += 1;
                        star.value *= number.value;
                    } else {
                        stars.insert(
                            neighbour.location,
                            Star {
                                neighbour_count: 1,
                                value: number.value,
                            },
                        );
                    }
                }
            }
        }

        let stars: u32 = stars
            .values()
            .filter(|x| x.neighbour_count > 1)
            .map(|x| x.value)
            .sum();

        println!("{}", stars);
    }
}
