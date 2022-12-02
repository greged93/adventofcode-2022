use anyhow::Error;
use std::fs;

const SCORES_SIGN: [u32; 3] = [1, 2, 3];
const A: u8 = b'A' - 1;
const X: u8 = b'X' - 1;

fn read_file(path: &str) -> Result<String, Error> {
    let input = fs::read_to_string(path)?;
    Ok(input.replace(" ", "").replace("\n", ""))
}

fn calculate_score<F>(input: String, f: Box<F>) -> u32
where
    F: Fn(u8, u8) -> u32 + ?Sized,
{
    input
        .as_bytes()
        .chunks(2)
        .map(|x| f(x[0] - A, x[1] - X))
        .sum()
}

fn score_first() -> Box<dyn Fn(u8, u8) -> u32> {
    return Box::new(|i, j| match (i as i8 - j as i8 + 3) % 3 {
        0 => 3u32 + j as u32,
        1 => 0u32 + j as u32,
        2 => 6u32 + j as u32,
        _ => panic!("incorrect value"),
    });
}

fn score_second() -> Box<dyn Fn(u8, u8) -> u32> {
    return Box::new(|i, j| match j {
        1 => SCORES_SIGN[((i + 1) % 3) as usize],
        2 => 3u32 + i as u32,
        3 => 6u32 + SCORES_SIGN[(i % 3) as usize],
        _ => panic!("incorrect value"),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        assert_eq!("AXAYAZBXBYBZCXCYCZ", read_file("./data/test.txt").unwrap());
    }

    #[test]
    fn test_score_first() {
        let s = read_file("./data/data.txt").unwrap();
        let output = calculate_score(s, score_first());
        dbg!(output);
    }

    #[test]
    fn test_score_second() {
        let s = read_file("./data/data.txt").unwrap();
        let output = calculate_score(s, score_second());
        dbg!(output);
    }
}
