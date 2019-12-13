use super::Answer;

fn digits(
    digit: i32,
    has_rep: bool,
    last_rep: bool,
    depth: u8,
    number: i32,
    start: i32,
    end: i32,
    blacklist: i32,
    part2: bool,
) -> i32 {
    if depth == 6 {
        if (has_rep || (last_rep && blacklist != digit)) && number >= start && number <= end {
            return 1;
        } else {
            return 0;
        }
    }
    let mut total = 0;
    // Only consider digits of equal of greater value
    for i in digit..10 {
        let mut update_rep = false;
        // Blacklist means we've seen 3 or more in a row
        let update_blacklist = {
            if last_rep && (digit == i) {
                digit
            } else {
                -1
            }
        };
        if part2 {
            if last_rep && (digit != i) && (digit != blacklist) {
                update_rep = true;
            }
        } else if digit == i {
            update_rep = true;
        }
        total += digits(
            i,
            has_rep || update_rep,
            digit == i,
            depth + 1,
            10 * number + i,
            start,
            end,
            update_blacklist,
            part2,
        );
    }
    total
}

pub fn p4(input: &str) -> Answer {
    let mut split = input.split("-");
    let start: i32 = split.next().unwrap().parse().unwrap();
    let end: i32 = split.next().unwrap().parse().unwrap();
    let mut sums = Vec::new();
    for part2 in [false, true].into_iter() {
        let mut sum = 0;
        for initial in 1..10 {
            if initial >= (start / 100000) && initial <= (end / 100000) {
                sum += digits(initial, false, false, 1, initial, start, end, -1, *part2);
            }
        }
        sums.push(sum);
    }

    Answer::new(sums[0], sums[1])
}