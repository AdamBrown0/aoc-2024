use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;

advent_of_code::solution!(6);

type Parsed = (Vec<Vec<char>>, Guard);

pub fn parse(input: &str) -> Parsed {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut guard = Guard::new(0, 0, Direction::Up);
    let mut x = 0;
    let mut y = 0;
    let mut found = false;
    for line in input.lines() {
        let mut inner: Vec<char> = vec![];
        for c in line.chars() {
            inner.push(c);
            if !found && "^><v".contains(c) {
                guard.x = x;
                guard.y = y;
                match c {
                    '^' => guard.direction = Direction::Up,
                    '>' => guard.direction = Direction::Right,
                    '<' => guard.direction = Direction::Left,
                    'v' => guard.direction = Direction::Down,
                    _ => eprintln!("well that's not good"),
                }
                found = true;
            }
            x += 1;
        }
        x = 0;
        y += 1;
        grid.push(inner);
    }
    (grid, guard)
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone)]
pub struct Guard {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Guard {
    fn new(x: isize, y: isize, direction: Direction) -> Self {
        Guard { x, y, direction }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn is_obstacle(&self, grid: &Vec<Vec<char>>) -> bool {
        match self.direction {
            Direction::Up => self.y > 0 && grid[(self.y - 1) as usize][self.x as usize] == '#',
            Direction::Right => {
                self.x < (grid[0].len() as isize - 1)
                    && grid[self.y as usize][(self.x + 1) as usize] == '#'
            }
            Direction::Down => {
                self.y < (grid.len() as isize - 1)
                    && grid[(self.y + 1) as usize][self.x as usize] == '#'
            }
            Direction::Left => self.x > 0 && grid[self.y as usize][(self.x - 1) as usize] == '#',
        }
    }

    fn patrol(&mut self, grid: &Vec<Vec<char>>, visited: &mut HashSet<(isize, isize)>) -> bool {
        let mut seen_positions = BTreeSet::new();
        visited.insert((self.x, self.y));
        while self.x < grid[0].len() as isize && self.y < grid.len() as isize {
            if self.is_obstacle(grid) {
                self.direction = self.direction.turn_right();
            } else {
                self.move_forward();
                if self.x >= grid[0].len() as isize
                    || self.x < 0
                    || self.y >= grid.len() as isize
                    || self.y < 0
                {
                    break;
                }
                if !seen_positions.insert((self.x, self.y, self.direction)) {
                    return true;
                }
                visited.insert((self.x, self.y));
            }
        }
        false
    }
}

pub fn part_one(mut input: Parsed) -> Option<u32> {
    let mut visited = HashSet::new();
    input.1.patrol(&input.0, &mut visited);
    Some(visited.len() as u32)
}

pub fn part_two(input: Parsed) -> Option<u32> {
    let mut successful: u32 = 0;
    let grid = input.0;
    let guard = input.1;
    let mut temp_guard = Guard::new(guard.x, guard.y, guard.direction);
    let mut temp_visited = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            temp_guard.x = guard.x;
            temp_guard.y = guard.y;
            temp_guard.direction = guard.direction;
            temp_visited.clear();
            if grid[y][x] != '#' {
                let mut temp_grid = grid.clone();
                temp_grid[y][x] = '#';
                if temp_guard.patrol(&temp_grid, &mut temp_visited) {
                    successful += 1;
                }
            } else {
                if temp_guard.patrol(&grid, &mut temp_visited) {
                    successful += 1;
                }
            }
        }
    }

    Some(successful)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(parse(&advent_of_code::template::read_file("examples", DAY)));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(parse(&advent_of_code::template::read_file("examples", DAY)));
        assert_eq!(result, Some(6));
    }
}
