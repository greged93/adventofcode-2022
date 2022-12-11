use anyhow::Error;
use std::fs::read_to_string;

fn read_file(path: &str) -> Result<Vec<String>, Error> {
    return Ok(read_to_string(path)?
        .split(|c| c == '\n' || c == '-' || c == ',')
        .map(|x| x.to_string())
        .collect());
}

fn count_ranges_1(mut input: Vec<String>) -> u32 {
    input
        .chunks(4)
        .map(|x| {
            let num_1 = x[0].parse::<u32>().unwrap();
            let num_2 = x[1].parse::<u32>().unwrap();
            let num_3 = x[2].parse::<u32>().unwrap();
            let num_4 = x[3].parse::<u32>().unwrap();
            if num_1 <= num_3 && num_2 >= num_4 || num_3 <= num_1 && num_2 <= num_4 {
                return 1;
            }
            return 0;
        })
        .sum()
}

fn count_ranges_2(input: Vec<String>) -> u32 {
    input
        .chunks(4)
        .map(|x| {
            let num_1 = x[0].parse::<u32>().unwrap();
            let num_2 = x[1].parse::<u32>().unwrap();
            let num_3 = x[2].parse::<u32>().unwrap();
            let num_4 = x[3].parse::<u32>().unwrap();
            if num_2 >= num_3 && num_1 <= num_4 {
                return 1;
            }
            return 0;
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let f = read_file("./src/data/test.txt");
        assert_eq!(
            vec![
                "2", "4", "6", "8", "2", "3", "4", "5", "5", "7", "7", "9", "2", "8", "3", "7",
                "6", "6", "4", "6", "2", "6", "4", "8"
            ],
            f.unwrap()
        );
    }

    #[test]
    fn test_count_range_1() {
        let r = count_ranges_1(read_file("./src/data/data.txt").unwrap());
        assert_eq!(511, r);
    }

    #[test]
    fn test_count_range_2() {
        let r = count_ranges_2(read_file("./src/data/data.txt").unwrap());
        assert_eq!(821, r);
    }
}
