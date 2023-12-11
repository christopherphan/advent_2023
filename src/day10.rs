/* Solution to 2023 Advent of Code, Day 10
 *
 * Christopher Phan
 */

use std::collections::HashMap;
use std::error::Error;

use crate::common;
use crate::common::AdventError;

const DAY: usize = 10;
const PART_1_IMPL: bool = true;
const PART_2_IMPL: bool = true;

pub fn run() {
    let input = common::get_day(DAY).unwrap();
    if PART_1_IMPL {
        println!(
            "{}",
            common::soln_output(DAY, 1, part_1(input.clone()).unwrap())
        );
    }
    if PART_2_IMPL {
        println!("{}", common::soln_output(DAY, 2, part_2(input).unwrap()));
    }
    if !(PART_1_IMPL || PART_2_IMPL) {
        println!("Not implemented yet");
    }
}

pub fn part_1(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    let board = Board::try_from(input)?;
    let loop_len = board.find_main_loop()?.len() - 1;
    Ok(loop_len as u64 / 2)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    // loc given in (col, row)
    fn from_coords(&self, loc: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::North => {
                if loc.1 > 0 {
                    Some((loc.0, loc.1 - 1))
                } else {
                    None
                }
            }
            Self::South => Some((loc.0, loc.1 + 1)),
            Self::East => Some((loc.0 + 1, loc.1)),
            Self::West => {
                if loc.0 > 0 {
                    Some((loc.0 - 1, loc.1))
                } else {
                    None
                }
            }
        }
    }

    // gives the direction between two coordinates in the same row or column
    fn between_coords(start: (usize, usize), end: (usize, usize)) -> Option<Self> {
        if start == end {
            None
        } else if start.0 == end.0 {
            // same column
            if end.1 < start.1 {
                Some(Self::North)
            } else {
                Some(Self::South)
            }
        } else if start.1 == end.1 {
            // same row
            if end.0 < start.0 {
                Some(Self::West)
            } else {
                Some(Self::East)
            }
        } else {
            // not on same row or column
            None
        }
    }

    fn adjacent_coords(loc: (usize, usize)) -> Vec<(usize, usize)> {
        Self::iter_all()
            .map(|k| k.from_coords(loc))
            .filter_map(|k| k)
            .collect()
    }

    fn iter_all() -> Box<dyn Iterator<Item = Direction>> {
        Box::new(
            [Self::North, Self::West, Self::South, Self::East]
                .iter()
                .cloned(),
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PipeTile {
    NorthWest,
    NorthSouth,
    NorthEast,
    SouthWest,
    SouthEast,
    WestEast,
}

impl From<PipeTile> for [Direction; 2] {
    fn from(tile: PipeTile) -> Self {
        match tile {
            PipeTile::NorthWest => [Direction::North, Direction::West],
            PipeTile::NorthSouth => [Direction::North, Direction::South],
            PipeTile::NorthEast => [Direction::North, Direction::East],
            PipeTile::SouthWest => [Direction::South, Direction::West],
            PipeTile::SouthEast => [Direction::South, Direction::East],
            PipeTile::WestEast => [Direction::West, Direction::East],
        }
    }
}

impl TryFrom<[Direction; 2]> for PipeTile {
    type Error = AdventError;

    fn try_from(arr: [Direction; 2]) -> Result<Self, Self::Error> {
        let mut vec: Vec<Direction> = arr.into();
        vec.dedup();
        if vec.len() != 2 {
            Err(AdventError::new(
                "input must be an array of two distinct Directions",
            ))
        } else {
            if vec.contains(&Direction::North) {
                if vec.contains(&Direction::West) {
                    Ok(Self::NorthWest)
                } else if vec.contains(&Direction::South) {
                    Ok(Self::NorthSouth)
                } else {
                    Ok(Self::NorthEast)
                }
            } else if vec.contains(&Direction::South) {
                if vec.contains(&Direction::West) {
                    Ok(Self::SouthWest)
                } else {
                    Ok(Self::SouthEast)
                }
            } else {
                Ok(Self::WestEast)
            }
        }
    }
}

impl PipeTile {
    fn possible_directions(&self) -> Vec<Direction> {
        let dirs_arr: [Direction; 2] = (*self).into();
        dirs_arr.into()
    }

    fn other_direction(&self, dir: Direction) -> Option<Direction> {
        let poss_dir = self.possible_directions();
        if poss_dir.contains(&dir) {
            poss_dir.iter().filter(|k| **k != dir).copied().last()
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Start,
    Pipe(PipeTile),
}

impl TryFrom<char> for Tile {
    type Error = AdventError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Self::Pipe(PipeTile::NorthSouth)),
            '-' => Ok(Self::Pipe(PipeTile::WestEast)),
            'L' => Ok(Self::Pipe(PipeTile::NorthEast)),
            'J' => Ok(Self::Pipe(PipeTile::NorthWest)),
            '7' => Ok(Self::Pipe(PipeTile::SouthWest)),
            'F' => Ok(Self::Pipe(PipeTile::SouthEast)),
            '.' => Ok(Self::Empty),
            'S' => Ok(Self::Start),
            _ => Err(AdventError(format!("unknown character: {}", c))),
        }
    }
}

impl Tile {
    fn possible_directions(&self) -> Vec<Direction> {
        match self {
            Self::Pipe(pipe_type) => pipe_type.possible_directions(),
            Self::Start => Direction::iter_all().collect(),
            Self::Empty => vec![],
        }
    }
}

struct Board(HashMap<(usize, usize), Tile>);

impl TryFrom<Vec<String>> for Board {
    type Error = AdventError;

    fn try_from(input: Vec<String>) -> Result<Self, Self::Error> {
        let mut hmap: HashMap<(usize, usize), Tile> = HashMap::new();
        for (row, line) in input.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                let tile: Tile = Tile::try_from(c)?;
                hmap.insert((col, row), tile);
            }
        }
        Ok(Self(hmap))
    }
}

impl Board {
    fn find_start(&self) -> Option<(usize, usize)> {
        self.0
            .iter()
            .filter(|(_, k)| **k == Tile::Start)
            .map(|k| k.0)
            .copied()
            .last()
    }

    fn find_path(&self, dir: Direction) -> Vec<(usize, usize)> {
        let mut out_vec = vec![];
        if let Some(starting_pos) = self.find_start() {
            let mut cur_pos = starting_pos;
            let mut next_dir = dir;
            let mut done = false;
            while !done {
                if let Some(tile) = self.0.get(&cur_pos) {
                    match tile {
                        Tile::Start | Tile::Pipe(_) => {
                            out_vec.push(cur_pos);
                            if let Some(new_pos) = next_dir.from_coords(cur_pos) {
                                if let Some(next_tile) = self.0.get(&new_pos) {
                                    if next_tile
                                        .possible_directions()
                                        .iter()
                                        .map(|k| k.opposite())
                                        .filter(|k| *k == next_dir)
                                        .count()
                                        == 1
                                    // This will be false if at a dead-end
                                    {
                                        cur_pos = new_pos;
                                        if let Tile::Pipe(next_type) = next_tile {
                                            next_dir = next_type
                                                .other_direction(next_dir.opposite())
                                                .unwrap();
                                        } else {
                                            out_vec.push(new_pos);
                                            done = true; // We are back at start
                                        }
                                    } else {
                                        done = true;
                                    }
                                } else {
                                    done = true;
                                }
                            } else {
                                done = true;
                            }
                        }
                        _ => {
                            done = true;
                        }
                    }
                } else {
                    done = true;
                }
            }
        }
        out_vec
    }

    fn find_paths(&self) -> HashMap<Direction, Vec<(usize, usize)>> {
        HashMap::from_iter(Direction::iter_all().map(|dir| (dir, self.find_path(dir))))
    }

    fn find_loops(&self) -> Vec<Vec<(usize, usize)>> {
        self.find_paths()
            .iter()
            .filter(|(_, pth)| Self::path_is_loop(pth.to_vec()))
            .map(|(_, pth)| pth.to_vec())
            .collect()
    }

    fn vec_steps(vec: Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize))> {
        if vec.len() < 2 {
            vec![]
        } else {
            let mut copy1: Vec<(usize, usize)> = vec.clone();
            copy1.pop();
            let copy2 = vec.iter().skip(1);
            copy1.iter().zip(copy2).map(|(a, b)| (*a, *b)).collect()
        }
    }

    fn vec_is_path(vec: Vec<(usize, usize)>) -> bool {
        if vec.len() < 2 {
            true // vacuously
        } else {
            Self::vec_steps(vec)
                .iter()
                .all(|(a, b)| Direction::adjacent_coords(*a).contains(b))
        }
    }

    fn path_is_loop(pth: Vec<(usize, usize)>) -> bool {
        pth.len() > 1 && Self::vec_is_path(pth.clone()) && pth[0] == *pth.last().unwrap()
    }

    fn find_main_loop(&self) -> Result<Vec<(usize, usize)>, AdventError> {
        let mut loops = self.find_loops();
        if loops.is_empty() {
            Err(AdventError::new("no loops on map"))
        } else {
            Ok(loops.pop().unwrap())
        }
    }
}

pub fn part_2(input: Vec<String>) -> Result<u64, Box<dyn Error>> {
    let board = Board::try_from(input)?;
    let main_loop = board.find_main_loop()?;
    Ok(count_inside(main_loop)? as u64)
}

/* Path doubling:
 *
 * As stated in the problem, some of the tiles completely surrounded by pipes in the loop are still
 * outside the main loop in this example:
 *
 *  ..........
 *  .S------7.
 *  .|F----7|.
 *  .||....||.
 *  .||....||.
 *  .|L-7F-J|.
 *  .|..||..|.
 *  .L--JL--J.
 *  ..........
 *
 *  The tiles marked I are inside while the tiles marked O are outside.
 *  ..........
 *  .S------7.
 *  .|F----7|.
 *  .||OOOO||.
 *  .||OOOO||.
 *  .|L-7F-J|.
 *  .|II||II|.
 *  .L--JL--J.
 *  ..........
 *
 * My solution is to make a new loop that is twice as big, mapping (x, y) |-> (2x + 1, 2y + 1):
 *
 *  ...................
 *  ...................
 *  ...................
 *  ...###############.
 *  ...#.............#.
 *  ...#.###########.#.
 *  ...#.#.........#.#.
 *  ...#.#.........#.#.
 *  ...#.#.........#.#.
 *  ...#.#.........#.#.
 *  ...#.#.........#.#.
 *  ...#.#####.#####.#.
 *  ...#.....#.#.....#.
 *  ...#.....#.#.....#.
 *  ...#.....#.#.....#.
 *  ...#######.#######.
 *  ...................
 *
 *  Now we only have to consider whether the loop contains a point or not, not the direction of the
 *  pipes there. Find the component of the complement of the loop that contains (0, 0), and then
 *  count all coordinates of the form (2x + 1, 2y + 1) not contained in either the loop or the
 *  complement.
*/

fn double_coord(loc: (usize, usize)) -> (usize, usize) {
    (loc.0 * 2 + 1, loc.1 * 2 + 1)
}

fn double_path(pth: Vec<(usize, usize)>) -> Result<Vec<(usize, usize)>, AdventError> {
    let mut out_vec: Vec<(usize, usize)> = vec![];
    for (start, end) in Board::vec_steps(pth.clone()) {
        let dir: Direction = Direction::between_coords(start, end)
            .ok_or(AdventError::new("not in same row or column"))?;
        let new_start = double_coord(start);
        let middle = dir.from_coords(new_start).unwrap();
        out_vec.push(new_start);
        out_vec.push(middle);
    }
    out_vec.push(double_coord(*(pth.iter().last().unwrap())));
    Ok(out_vec)
}

fn southeast_corner(pth: Vec<(usize, usize)>) -> Option<(usize, usize)> {
    if pth.is_empty() {
        None
    } else {
        let max_x = pth.iter().map(|(x, _)| x).max().unwrap();
        let max_y = pth.iter().map(|(_, y)| y).max().unwrap();
        Some((*max_x, *max_y))
    }
}

fn complement_component(pth: Vec<(usize, usize)>, loc: (usize, usize)) -> Vec<(usize, usize)> {
    let mut out_vec: Vec<(usize, usize)> = vec![];
    if !pth.is_empty() && !pth.contains(&loc) {
        let se_corner = southeast_corner(pth.clone()).unwrap();
        let se_corner = (se_corner.0 + 1, se_corner.1 + 1);
        let mut to_process: Vec<(usize, usize)> = vec![loc];
        while !to_process.is_empty() {
            let cur_loc = to_process.pop().unwrap();
            let mut adjacent: Vec<(usize, usize)> = Direction::adjacent_coords(cur_loc)
                .iter()
                .filter(|k| !pth.clone().contains(&k))
                .filter(|k| !out_vec.clone().contains(&k))
                .filter(|k| !to_process.clone().contains(&k))
                .copied()
                .filter(|k| k.0 <= se_corner.0 && k.1 <= se_corner.1)
                .collect();
            to_process.append(&mut adjacent);
            out_vec.push(cur_loc);
        }
    }
    out_vec
}

fn count_inside(pth: Vec<(usize, usize)>) -> Result<usize, AdventError> {
    let mut out_val: usize = 0;
    let se_corner = southeast_corner(pth.clone()).ok_or(AdventError::new("empty path"))?;
    if !pth.is_empty() {
        let doubled_path: Vec<(usize, usize)> = double_path(pth.clone())?;
        let outside = complement_component(doubled_path.clone(), (0, 0));
        for col in 0..(se_corner.0) {
            for row in 0..(se_corner.1) {
                if !outside.contains(&(col * 2 + 1, row * 2 + 1)) && !pth.contains(&(col, row)) {
                    out_val += 1;
                }
            }
        }
    }
    Ok(out_val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common;

    const EXAMPLE_INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const EXAMPLE_INPUT_2: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const EXAMPLE_INPUT_3: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const EXAMPLE_INPUT_4: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    const EXAMPLE_INPUT_5: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const EXAMPLE_INPUT_6: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const EXAMPLE_INPUT_7: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    const ANOTHER_EXAMPLE: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    fn get_example_inputs() -> Vec<Vec<String>> {
        [
            EXAMPLE_INPUT_1,
            EXAMPLE_INPUT_2,
            EXAMPLE_INPUT_3,
            EXAMPLE_INPUT_4,
        ]
        .iter()
        .map(|k| common::split_string(k.to_string()))
        .collect()
    }

    fn get_example_inputs_part_2() -> Vec<Vec<String>> {
        let mut out_vec = get_example_inputs();
        let mut new_stuff: Vec<Vec<String>> = [EXAMPLE_INPUT_5, EXAMPLE_INPUT_6, EXAMPLE_INPUT_7]
            .iter()
            .map(|k| common::split_string(k.to_string()))
            .collect();
        out_vec.append(&mut new_stuff);
        out_vec
    }

    #[test]
    fn find_start_test() {
        let boards: Vec<Board> = get_example_inputs()
            .iter()
            .map(|k| Board::try_from(k.to_vec()).unwrap())
            .collect();
        let starts: Vec<(usize, usize)> = boards.iter().map(|k| k.find_start().unwrap()).collect();
        assert_eq!(starts, vec![(1, 1), (1, 1), (0, 2), (0, 2)]);
    }

    #[test]
    fn find_paths_test_1() {
        let example_input: Vec<String> = get_example_inputs().get(0).unwrap().clone();
        let board = Board::try_from(example_input).unwrap();
        let paths = board.find_paths();
        println!("{:?}", paths);
        for k in paths.values() {
            assert!(k.len() == 9 || k.len() == 1);
        }
        assert_eq!(board.find_paths().len(), 4);
    }
    #[test]
    fn find_loops_test_1() {
        let example_input: Vec<String> = get_example_inputs().get(0).unwrap().clone();
        let board = Board::try_from(example_input).unwrap();
        let loops = board.find_loops();
        assert_eq!(loops.len(), 2);
    }

    #[test]
    fn vec_steps_test_1() {
        let pth: Vec<(usize, usize)> = vec![(0, 0), (0, 1), (1, 1), (1, 0), (0, 0)];
        let vec_steps = Board::vec_steps(pth);
        assert_eq!(
            vec_steps,
            vec![
                ((0, 0), (0, 1)),
                ((0, 1), (1, 1)),
                ((1, 1), (1, 0)),
                ((1, 0), (0, 0))
            ]
        );
    }

    #[test]
    fn path_doubling_test_0a() {
        let pth: Vec<(usize, usize)> = vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)];
        let doubled_pth = double_path(pth).unwrap();
        assert_eq!(
            doubled_pth,
            vec![
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (2, 5),
                (3, 5),
                (4, 5),
                (5, 5)
            ]
        );
    }

    #[test]
    fn path_doubling_test_0b() {
        let pth: Vec<(usize, usize)> = vec![(2, 2), (1, 2), (0, 2), (0, 1), (0, 0)];
        let doubled_pth = double_path(pth).unwrap();
        assert_eq!(
            doubled_pth,
            vec![
                (5, 5),
                (4, 5),
                (3, 5),
                (2, 5),
                (1, 5),
                (1, 4),
                (1, 3),
                (1, 2),
                (1, 1)
            ]
        );
    }

    #[test]
    fn path_doubling_test_1() {
        let example_input: Vec<String> = get_example_inputs().get(0).unwrap().clone();
        let board = Board::try_from(example_input).unwrap();
        let pth = board.find_path(Direction::South);
        println!("original path: {:?}", pth);
        let doubled_pth = double_path(pth).unwrap();
        assert_eq!(
            doubled_pth,
            vec![
                (3, 3),
                (3, 4),
                (3, 5),
                (3, 6),
                (3, 7),
                (4, 7),
                (5, 7),
                (6, 7),
                (7, 7),
                (7, 6),
                (7, 5),
                (7, 4),
                (7, 3),
                (6, 3),
                (5, 3),
                (4, 3),
                (3, 3)
            ]
        );
    }

    #[test]
    fn complement_test_0() {
        let example_input = get_example_inputs().get(0).unwrap().clone();
        let board: Board = Board::try_from(example_input).unwrap();
        let pth = board.find_main_loop().unwrap();
        let outside = complement_component(pth.clone(), (0, 0));
        println!("path: {:?}\noutside: {:?}", pth, outside);
        assert_eq!(outside.len(), 16);
    }

    #[test]
    fn complement_test_1() {
        let example_input = common::split_string(ANOTHER_EXAMPLE.to_string());
        assert_eq!(part_2(example_input).unwrap(), 4);
    }

    #[test]
    fn part1_test() {
        let example_inputs = get_example_inputs();
        let outputs: Vec<u64> = example_inputs
            .iter()
            .map(|k| part_1(k.to_vec()).unwrap())
            .collect();
        assert_eq!(outputs, vec![4, 4, 8, 8]);
    }

    #[test]
    fn part2_test() {
        if PART_2_IMPL {
            let example_inputs = get_example_inputs_part_2();
            let outputs: Vec<u64> = example_inputs
                .iter()
                .map(|k| part_2(k.to_vec()).unwrap())
                .collect();
            assert_eq!(outputs, vec![1, 1, 1, 1, 4, 8, 10]);
        }
    }
}
