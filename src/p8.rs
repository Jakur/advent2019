use super::Answer; 

pub fn p8(input: &str) -> Answer {
    let width = 25;
    let height = 6;
    let layer_len = width * height;
    let chars: Vec<_> = input.lines().next().unwrap().chars().collect();
    let (min_index, _) = chars
        .chunks(layer_len)
        .map(|x| {
            x.iter().fold(0, |acc, c| match c {
                '0' => acc + 1,
                _ => acc,
            })
        })
        .enumerate()
        .min_by_key(|&(_, item)| item)
        .unwrap();
    let mut ones = 0;
    let mut twos = 0;
    for c in chars[min_index * layer_len..(min_index + 1) * layer_len].iter() {
        match c {
            '1' => ones += 1,
            '2' => twos += 1,
            _ => {}
        }
    }
    let chunks: Vec<_> = chars.chunks(layer_len).collect();
    let mut image = Vec::with_capacity(layer_len);
    for i in 0..chunks[0].len() {
        for chunk in chunks.iter() {
            let pixel = chunk[i];
            match pixel {
                '0' => {
                    image.push(' ');
                    break;
                }
                '1' => {
                    image.push('#');
                    break;
                }
                _ => {}
            }
        }
    }
    let mut part2 = "\n".to_string();
    for scan_line in image.chunks(width) {
        part2.extend(scan_line);
        part2.push_str("\n");
    }
    Answer::new(ones * twos, part2)
}