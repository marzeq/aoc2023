pub fn run(part: u8, input: String) {
    match part {
        1 => part_1(&input),
        2 => part_2(&input),
        _ => println!("Invalid part number"),
    }
}

fn part_1(input: &str) {
    fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<(u64, u64, u64)>>) {
        let mut lines = input.lines();
        let seeds: Vec<u64> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();

        let mut maps = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }

            if line.contains("map") {
                maps.push(Vec::new());
                continue;
            }

            let map = maps.last_mut().unwrap();

            let mut iter = line.split_whitespace();

            let x: u64 = iter.next().unwrap().parse().unwrap();
            let y: u64 = iter.next().unwrap().parse().unwrap();
            let z: u64 = iter.next().unwrap().parse().unwrap();

            map.push((x, y, z));
        }

        return (seeds, maps);
    }

    fn map_number(num: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
        for &(destination, source, len) in map {
            if num >= source && num < source + len {
                let offset = num - source;

                return destination + offset;
            }
        }

        return num;
    }

    let (mut seeds, maps) = parse_input(&input);

    for map in &maps {
        for i in 0..seeds.len() {
            seeds[i] = map_number(seeds[i], map);
        }
    }

    let min = seeds.iter().min().unwrap();

    println!("{min}");
}

// for some reason this is not working
// whoever came up with this is a bloody bastard
// i cannot be bothered to fix this, it works on the sample input but not on the real input
// piece of s***

fn part_2(input: &str) {
    #[derive(Debug, Clone, Copy)]
    struct Range {
        start: u64,
        end: u64,
        len: u64,
    }

    impl Range {
        fn overlaps(&self, other: &Range) -> bool {
            self.start < other.end && self.end > other.start
        }

        fn intersection(&self, other: &Range) -> Option<Range> {
            if !self.overlaps(other) {
                return None;
            }

            let start = if self.start > other.start {
                self.start
            } else {
                other.start
            };

            let end = if self.end < other.end {
                self.end
            } else {
                other.end
            };

            let len = end - start;

            return Some(Range { start, end, len });
        }

        fn subtract(&self, other: &Range) -> (Option<Range>, Option<Range>) {
            // case 1: a and x don't overlap
            if !self.overlaps(other) {
                return (Some(self.clone()), None);
            }
            // case 2: a and x overlap on the left side
            else if self.start < other.start && self.end <= other.end {
                let left = Range {
                    start: self.start,
                    end: other.start - 1,
                    len: other.start - self.start - 1,
                };

                return (Some(left), None);
            }
            // case 3: a and x overlap on the right side
            else if self.start >= other.start && self.end > other.end {
                let right = Range {
                    start: other.end + 1,
                    end: self.end,
                    len: self.end - other.end - 1,
                };

                return (None, Some(right));
            }
            // case 4: a completely overlaps x
            else if self.start >= other.start && self.end <= other.end {
                return (None, None);
            }
            // case 5: a is completely inside of x
            else if self.start < other.start && self.end > other.end {
                let left = Range {
                    start: self.start,
                    end: other.start - 1,
                    len: other.start - self.start - 1,
                };

                let right = Range {
                    start: other.end + 1,
                    end: self.end,
                    len: self.end - other.end - 1,
                };

                return (Some(left), Some(right));
            }

            return (None, None);
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct MapRange {
        source: Range,
        add: i64,
    }

    fn parse_input(input: &str) -> (Vec<Range>, Vec<Vec<MapRange>>) {
        let mut lines = input.lines();
        let ranges = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .chunks(2)
            .map(|pair| {
                let mut iter = pair.iter();
                let start: u64 = iter.next().unwrap().parse().unwrap();
                let len: u64 = iter.next().unwrap().parse().unwrap();

                let end = start + len;

                Range { start, end, len }
            })
            .collect::<Vec<Range>>();

        let mut maps = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }

            if line.contains("map") {
                maps.push(Vec::new());
                continue;
            }

            let map = maps.last_mut().unwrap();

            let mut iter = line.split_whitespace();

            let destination: u64 = iter.next().unwrap().parse().unwrap();
            let source: u64 = iter.next().unwrap().parse().unwrap();
            let len: u64 = iter.next().unwrap().parse().unwrap();

            let source_end = source + len - 1;

            let add = destination as i64 - source as i64;

            map.push(MapRange {
                source: Range {
                    start: source,
                    end: source_end,
                    len,
                },
                add,
            });
        }

        return (ranges, maps);
    }

    fn map_range(range: Range, map: &Vec<MapRange>) -> Vec<Range> {
        let mut result = vec![];

        for MapRange {
            source: source_range,
            add,
        } in map
        {
            if !range.overlaps(source_range) {
                continue;
            }

            let intersection = range.intersection(source_range).unwrap();

            let (left, right) = range.subtract(source_range);

            if let Some(right) = right {
                result.push(right);
            }

            if let Some(left) = left {
                result.push(left);
            }

            let start = intersection.start as i64 + *add;
            let end = intersection.end as i64 + *add;

            result.push(Range {
                start: start as u64,
                end: end as u64,
                len: intersection.len,
            });
        }

        if result.is_empty() {
            result.push(range);
        }

        return result;
    }

    let (ranges, maps) = parse_input(&input);

    let mut vecranges = ranges
        .clone()
        .iter()
        .map(|r| vec![r.clone()])
        .collect::<Vec<_>>();

    for map in &maps {
        for i in 0..vecranges.len() {
            let mut mapped: Vec<Range> = Vec::new();

            for range in &vecranges[i] {
                mapped.append(&mut map_range(range.clone(), map));
            }

            vecranges[i] = mapped;
        }
    }

    let min = vecranges
        .iter()
        .map(|ranges| ranges.iter().map(|range| range.start).min().unwrap())
        .min()
        .unwrap();

    println!("{min}");
}
