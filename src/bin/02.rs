#![feature(iter_map_windows)]

advent_of_code::solution!(2);

fn validate_list(lvl: &[u16]) -> bool {
    let inc: bool = (lvl[1] as i16 - lvl[0] as i16) > 0;
    !lvl.iter()
        .map_windows(|[&x, &y]: &[&u16; 2]| (y as i16 - x as i16))
        .any(|diff| {
            diff == 0 || (inc && (diff < 1 || diff > 3)) || (!inc && (diff < -3 || diff > -1))
        })
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut safe: u16 = 0;
    for line in input.lines() {
        let lvl: Vec<u16> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<u16>().ok())
            .collect();
        safe += validate_list(&lvl) as u16;
    }
    Some(safe)
}

/*
pub fn part_two(input: &str) -> Option<u16> {
    let mut safe: u16 = 0;
    for line in input.lines() {
        let lvl: Vec<u16> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<u16>().ok())
            .collect();
        let inc: bool = (lvl[1] as i16 - lvl[0] as i16) > 0;
        let safent = lvl
            .iter()
            .map_windows(|[&x, &y]: &[&u16; 2]| (y as i16 - x as i16))
            .any(|diff| diff == 0 || (inc && (diff < 1 || diff > 3)) || (!inc && (diff < -3 || diff > -1)));
        if safent {
            let inc: bool = (lvl[1] as i16 - lvl[0] as i16) > 0;
            let mut i: usize = 0;
            if lvl
                .windows(3)
                .enumerate()
                .any(|(index, window)| {
                    let diff = window[2] as i16 - window[0] as i16;
                    if diff == 0 || (inc && (diff < 1 || diff > 3)) || (!inc && (diff < -3 || diff > -1)) {
                        i = index;
                        true
                    } else {
                        false
                    }
                }) {
                safe += validate_list(&line[i..]) as u16;
            }
        }
    }

    Some(safe)
}
*/

pub fn part_two(input: &str) -> Option<u16> {
    let mut safe: u16 = 0;
    for line in input.lines() {
        let lvl: Vec<u16> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<u16>().ok())
            .collect();
        if validate_list(&lvl) {
            safe += 1;
        } else {
            for i in 0..lvl.len() {
                if validate_list(
                    &lvl.iter()
                        .enumerate()
                        .filter_map(|(index, &val)| if index != i { Some(val) } else { None })
                        .collect::<Vec<_>>(),
                ) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
