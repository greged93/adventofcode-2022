use anyhow::Error;
use std::fs::read_to_string;

fn read_file(path: &str) -> Result<Vec<String>, Error> {
    let s = read_to_string(path)?;
    Ok(s.split("\n\n").map(|x| x.to_string()).collect())
}

pub fn count_calories(calories: Vec<String>) -> u16 {
    let maxs = calories
        .into_iter()
        .map(|x| {
            x.split("\n")
                .map(|x| x.parse::<u16>().unwrap_or_default())
                .sum::<u16>()
        })
        .max();
    maxs.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        assert_eq!(
            vec![
                "1000\n2000\n3000",
                "4000",
                "5000\n6000",
                "7000\n8000\n9000",
                "10000"
            ],
            read_file("./data/calories.txt").unwrap()
        );
    }

    #[test]
    fn test_count_calories() {
        assert_eq!(
            24000,
            count_calories(read_file("./data/calories.txt").unwrap())
        );
    }
}
