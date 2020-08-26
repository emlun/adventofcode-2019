use crate::common::Solution;
use crate::intcode::IntcodeComputer;
use std::collections::HashMap;
use std::collections::VecDeque;

type Point = (i64, i64);

const ENABLE_OUTPUT: bool = false;

fn add(p1: &Point, p2: &Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

#[derive(Eq, PartialEq)]
enum Tile {
    Floor,
    Wall,
}

fn rotate_cw(dir: &Point) -> Point {
    (dir.1, -dir.0)
}

fn rotate_ccw(dir: &Point) -> Point {
    (-dir.1, dir.0)
}

fn dir_to_cmd(dir: Point) -> i64 {
    match dir {
        (1, 0) => 4,
        (0, 1) => 1,
        (-1, 0) => 3,
        (0, -1) => 2,
        _ => unreachable!(),
    }
}

struct State {
    pos: Point,
    dir: Point,
    dist: u32,
    computer: IntcodeComputer,
}

struct World {
    tiles: HashMap<Point, Tile>,
    goal: Option<(Point, u32)>,
}
impl World {
    fn new() -> World {
        let mut tiles = HashMap::new();
        tiles.insert((0, 0), Tile::Floor);
        World { tiles, goal: None }
    }
}

fn print_state(state: &State, queue: &VecDeque<State>, world: &World) {
    let minx = *world.tiles.keys().map(|(x, _)| x).min().unwrap_or(&0);
    let maxx = *world.tiles.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let miny = *world.tiles.keys().map(|(_, y)| y).min().unwrap_or(&0);
    let maxy = *world.tiles.keys().map(|(_, y)| y).max().unwrap_or(&0);

    println!(
        "{}",
        (miny..=maxy)
            .rev()
            .rev()
            .map(|y| {
                (minx..=maxx)
                    .map(|x| {
                        if (x, y) == state.pos {
                            match state.dir {
                                (1, 0) => ">",
                                (-1, 0) => "<",
                                (0, 1) => "v",
                                (0, -1) => "^",
                                _ => unreachable!(),
                            }
                        } else if queue
                            .iter()
                            .any(|state| add(&state.pos, &state.dir) == (x, y))
                            || add(&state.pos, &state.dir) == (x, y)
                        {
                            "?"
                        } else if world.goal.map(|(p, _)| p == (x, y)).unwrap_or(false) {
                            "X"
                        } else {
                            match world.tiles.get(&(x, y)) {
                                None => " ",
                                Some(Tile::Floor) => ".",
                                Some(Tile::Wall) => "#",
                            }
                        }
                    })
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn build_map(computer: IntcodeComputer) -> World {
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut world = World::new();

    for dir in &[(0, 1), (1, 0), (0, -1)] {
        queue.push_back(State {
            pos: (0, 0),
            dir: *dir,
            dist: 0,
            computer: computer.clone(),
        });
    }
    queue.push_back(State {
        pos: (0, 0),
        dir: (-1, 0),
        dist: 0,
        computer,
    });

    while let Some(mut state) = queue.pop_front() {
        let new_pos = add(&state.pos, &state.dir);

        #[allow(clippy::map_entry)]
        if !world.tiles.contains_key(&new_pos) {
            if ENABLE_OUTPUT {
                println!("{}", state.dist);
                print_state(&state, &queue, &world);
                println!();
            }

            state.computer.run_mut(Some(dir_to_cmd(state.dir)));

            let output = state.computer.output.pop_front().unwrap();
            if output == 0 {
                world.tiles.insert(new_pos, Tile::Wall);
            } else {
                let dist = state.dist + 1;
                world.tiles.insert(new_pos, Tile::Floor);

                if output == 2 && world.goal.is_none() {
                    world.goal = Some((new_pos, dist));
                }

                let next_candidates = [rotate_ccw(&state.dir), state.dir, rotate_cw(&state.dir)];
                let mut nexts: Vec<&Point> = next_candidates
                    .iter()
                    .filter(|dir| !world.tiles.contains_key(&add(&new_pos, dir)))
                    .collect();

                while nexts.len() > 1 {
                    queue.push_back(State {
                        pos: new_pos,
                        dir: *nexts.pop().unwrap(),
                        dist,
                        computer: state.computer.clone(),
                    });
                }

                if !nexts.is_empty() {
                    queue.push_back(State {
                        pos: new_pos,
                        dir: *nexts.pop().unwrap(),
                        dist,
                        computer: state.computer,
                    });
                }
            }
        }
    }

    world
}

fn solve_a(computer: IntcodeComputer) -> (u32, World) {
    let world = build_map(computer);
    (world.goal.unwrap().1, world)
}

fn solve_b(mut world: World) -> u32 {
    let start_pos = world.goal.unwrap().0;
    let mut buf1: Vec<(Point, Point)> = vec![(start_pos, start_pos)];
    let mut buf2: Vec<(Point, Point)> = Vec::new();

    world.tiles.retain(|_, tile| tile == &Tile::Floor);

    let mut time = 0;

    let mut heads = &mut buf1;
    let mut new_heads = &mut buf2;

    while !heads.is_empty() {
        time += 1;

        for (head, prev) in heads.drain(..) {
            for dir in &[(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let new_pos = add(&head, dir);
                if new_pos != prev && world.tiles.remove(&new_pos).is_some() {
                    new_heads.push((new_pos, head));
                }
            }
        }

        std::mem::swap(&mut heads, &mut new_heads);
    }

    time - 1
}

pub fn solve(lines: &[String]) -> Solution {
    let computer: IntcodeComputer = lines.into();
    let (a_solution, world) = solve_a(computer);
    (a_solution.to_string(), solve_b(world).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(rotate_cw(&(1, 0)), (0, -1));
        assert_eq!(rotate_ccw(&(1, 0)), (0, 1));
    }
}
