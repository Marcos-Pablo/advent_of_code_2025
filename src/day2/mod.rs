pub fn solve() {
    let ranges = parse_ranges();
    let mut part_1 = 0;
    let mut part_2 = 0;

    for range in ranges {
        for id in range.start..=range.end {
            let s = id.to_string();

            let mid = s.len() / 2;
            let (left, right) = s.split_at(mid);

            if left == right {
                part_1 += id;
            }

            for size in 1..=mid {
                if s.len() % size != 0 {
                    continue;
                }

                let num_windows = s.len() / size;
                let mut invalid_id = true;
                let mut w_start = 0;
                let mut w_end = w_start + size;
                let target_window = &s[w_start..w_end];

                for _ in 0..num_windows {
                    let window = &s[w_start..w_end];

                    if window != target_window {
                        invalid_id = false;
                        break;
                    }

                    w_start += size;
                    w_end += size;
                }

                if invalid_id {
                    part_2 += id;
                    break;
                }
            }
        }
    }

    println!("Adding up all the invalid IDs in part 1 produces {part_1}.");
    println!("Adding up all the invalid IDs in part 2 produces {part_2}.");
}

struct Range {
    start: u64,
    end: u64,
}

fn parse_ranges() -> Vec<Range> {
    let s = std::fs::read_to_string("./src/day2/input.txt").expect("Error reading file");

    s.trim()
        .split(',')
        .map(|chunk| {
            let (start, end) = chunk.split_once('-').expect("expected start-end");

            Range {
                start: start
                    .parse()
                    .unwrap_or_else(|_| panic!("invalid start {start:?}")),
                end: end
                    .parse()
                    .unwrap_or_else(|_| panic!("invalid end {end:?}")),
            }
        })
        .collect()
}
