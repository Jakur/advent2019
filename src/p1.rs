use super::Answer;

pub fn p1(input: &str) -> Answer {
    let vec: Vec<_> = input
        .lines()
        .map(|s| (s.parse::<i32>().unwrap() / 3) - 2)
        .collect();
    let simple_sum: i32 = vec.iter().sum();
    let total: i32 = vec
        .iter()
        .map(|x| {
            let mut partial = *x;
            let mut next = (x / 3) - 2;
            while next > 0 {
                partial += next;
                next = (next / 3) - 2;
            }
            partial
        })
        .sum();
    Answer::new(simple_sum, total)
}