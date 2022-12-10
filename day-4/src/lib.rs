use anyhow::Error;
use std::fs::read_to_string;

fn read_file(path: &str) -> Result<Vec<String>, Error> {
    return Ok(read_to_string(path)?
        .split("\n")
        .map(|x| x.to_string())
        .collect());
}

fn count_ranges(mut input: Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|x| {
            let y: Vec<&str> = x.split(",").collect();
            let v_1: Vec<String> = y[0].split("-").map(|c| c.to_string()).collect();
            let v_2: Vec<String> = y[1].split("-").map(|c| c.to_string()).collect();
            let num_1 = v_1[0].parse::<u32>().unwrap();
            let num_2 = v_1[1].parse::<u32>().unwrap();
            let num_3 = v_2[0].parse::<u32>().unwrap();
            let num_4 = v_2[1].parse::<u32>().unwrap();
            if num_1 <= num_3 && num_2 >= num_4 || num_3 <= num_1 && num_2 <= num_4 {
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
            vec!["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"],
            f.unwrap()
        );
    }

    #[test]
    fn test_count_range() {
        let r = count_ranges(read_file("./src/data/data.txt").unwrap());
        assert_eq!(511, r);
    }
}
