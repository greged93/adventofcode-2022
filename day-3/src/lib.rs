use anyhow::Error;
use std::fs::read_to_string;

fn read_file(path: &str) -> Result<Vec<String>, Error> {
    Ok(read_to_string(path)?
        .split("\n")
        .map(|x| x.to_string())
        .collect())
}

fn count_duplicate<F>(input: Vec<String>, cs: usize, f: Box<F>) -> u32
where
    F: Fn(Vec<&[u8]>) -> u32 + ?Sized,
{
    input
        .chunks(cs)
        .into_iter()
        .map(|x| {
            let ch: Vec<&[u8]> = x.into_iter().map(|s| s.as_bytes()).collect();
            f(ch)
        })
        .map(|x| {
            if x > 90 {
                x - b'a' as u32 + 1
            } else {
                x - b'A' as u32 + 27
            }
        })
        .sum()
}

fn find_duplicate_first() -> Box<dyn Fn(Vec<&[u8]>) -> u32> {
    Box::new(|b| {
        let bytes = b[0];
        let t = bytes[0..bytes.len() / 2]
            .iter()
            .find(|&x| bytes[bytes.len() / 2..].contains(x))
            .unwrap_or(&0);
        *t as u32
    })
}

fn find_duplicate_second() -> Box<dyn Fn(Vec<&[u8]>) -> u32> {
    Box::new(|b| {
        let t = b[0]
            .iter()
            .find(|&x| b[1].contains(x) && b[2].contains(x))
            .unwrap_or(&0);
        *t as u32
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        assert_eq!(
            vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ],
            read_file("./data/test_1.txt").unwrap()
        );
    }

    #[test]
    fn test_find_duplicate_first() {
        let s = read_file("./data/test_1.txt").unwrap();
        let ch: &[u8] = s[0].as_bytes();
        let f = find_duplicate_first();
        assert_eq!(112, f(vec![ch]));
    }

    #[test]
    fn test_find_duplicate_second() {
        let s = read_file("./data/test_2.txt").unwrap();
        let ch1: &[u8] = s[0].as_bytes();
        let ch2: &[u8] = s[1].as_bytes();
        let ch3: &[u8] = s[2].as_bytes();
        let f = find_duplicate_second();
        assert_eq!(114, f(vec![ch1, ch2, ch3]));
    }

    #[test]
    fn test_count_duplicate_first() {
        let s = read_file("./data/test_1.txt").unwrap();
        assert_eq!(157, count_duplicate(s, 1, find_duplicate_first()));
    }

    #[test]
    fn test_count_duplicate_first_data() {
        let s = read_file("./data/data_1.txt").unwrap();
        assert_eq!(7766, count_duplicate(s, 1, find_duplicate_first()));
    }

    #[test]
    fn test_count_duplicate_second() {
        let s = read_file("./data/test_2.txt").unwrap();
        assert_eq!(70, count_duplicate(s, 3, find_duplicate_second()));
    }

    #[test]
    fn test_count_duplicate_second_data() {
        let s = read_file("./data/data_2.txt").unwrap();
        assert_eq!(2415, count_duplicate(s, 3, find_duplicate_second()));
    }
}
