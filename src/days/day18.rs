use crate::common::Solution;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Point = (usize, usize);

#[derive(Eq, PartialEq)]
enum Tile {
    Door(KeyId),
    Floor,
    Key(KeyId),
    Wall,
}

use Tile::{Door, Floor, Key, Wall};

#[derive(Clone, Copy, Eq, PartialEq)]
struct KeyId {
    value: u32,
}

impl KeyId {
    fn to_char(self) -> char {
        ((f64::from(self.value).log2().round() as u8) + b'a') as char
    }
}

impl From<char> for KeyId {
    fn from(c: char) -> Self {
        let basis = if c.is_ascii_uppercase() { 'A' } else { 'a' };
        KeyId {
            value: 2_u32.pow(c as u32 - basis as u32),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct KeySet {
    keys: u32,
}

impl KeySet {
    fn new() -> Self {
        KeySet { keys: 0 }
    }

    fn with(mut self, key: KeyId) -> Self {
        self.keys |= key.value;
        self
    }

    fn union(mut self, keys: KeySet) -> Self {
        self.keys |= keys.keys;
        self
    }

    fn contains(self, key: KeyId) -> bool {
        self.keys & key.value != 0
    }

    fn contains_all(self, keys: KeySet) -> bool {
        self.keys & keys.keys == keys.keys
    }
}

impl<K> std::iter::FromIterator<K> for KeySet
where
    K: Into<KeyId>,
{
    fn from_iter<I>(it: I) -> Self
    where
        I: IntoIterator<Item = K>,
    {
        KeySet {
            keys: it.into_iter().fold(0, |keys, key| keys | key.into().value),
        }
    }
}

impl std::fmt::Debug for KeySet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:b}", self.keys)
    }
}

#[derive(Eq, PartialEq)]
struct World {
    tiles: Vec<Vec<Tile>>,
    keys: KeySet,
}

struct Navigation<'world> {
    world: &'world World,
    moves: HashMap<Point, Vec<Route>>,
}

#[derive(Debug)]
struct Route {
    to: Point,
    collected_keys: KeySet,
    prerequired_keys: KeySet,
    len: usize,
}

impl<'world> Navigation<'world> {
    fn available_moves(&mut self, from: Point) -> &Vec<Route> {
        #[derive(Debug)]
        struct PartialRoute {
            pos: Point,
            prev_pos: Point,
            collected_keys: KeySet,
            prerequired_keys: KeySet,
            len: usize,
        }

        if self.moves.get(&from).is_none() {
            let mut moves: Vec<Route> = Vec::new();
            let mut visited: HashSet<(Point, KeySet)> = HashSet::new();
            let mut queue: VecDeque<PartialRoute> = VecDeque::new();
            queue.push_back(PartialRoute {
                pos: from,
                prev_pos: from,
                collected_keys: KeySet::new(),
                prerequired_keys: KeySet::new(),
                len: 0,
            });

            while let Some(proute) = queue.pop_front() {
                for &next_pos in [
                    (proute.pos.0 + 1, proute.pos.1),
                    (proute.pos.0, proute.pos.1 + 1),
                    (proute.pos.0 - 1, proute.pos.1),
                    (proute.pos.0, proute.pos.1 - 1),
                ]
                .iter()
                .filter(|p| **p != proute.prev_pos)
                .filter(|p| self.world.tiles[p.1][p.0] != Wall)
                {
                    let pos_visited_with_fewer_keys = visited.iter().any(|&(pos, prereq)| {
                        pos == next_pos && proute.prerequired_keys.contains_all(prereq)
                    });
                    if !pos_visited_with_fewer_keys {
                        let next_len = proute.len + 1;

                        match &self.world.tiles[next_pos.1][next_pos.0] {
                            Floor => {
                                queue.push_back(PartialRoute {
                                    pos: next_pos,
                                    prev_pos: proute.pos,
                                    collected_keys: proute.collected_keys,
                                    prerequired_keys: proute.prerequired_keys,
                                    len: next_len,
                                });

                                visited.insert((next_pos, proute.prerequired_keys));
                            }

                            Key(k) => {
                                moves.push(Route {
                                    to: next_pos,
                                    len: next_len,
                                    collected_keys: proute.collected_keys.with(*k),
                                    prerequired_keys: proute.prerequired_keys,
                                });

                                visited.insert((next_pos, proute.prerequired_keys));
                            }

                            Door(k) => {
                                let prereq = if proute.collected_keys.contains(*k) {
                                    proute.prerequired_keys
                                } else {
                                    proute.prerequired_keys.with(*k)
                                };
                                queue.push_back(PartialRoute {
                                    pos: next_pos,
                                    prev_pos: proute.pos,
                                    collected_keys: proute.collected_keys,
                                    prerequired_keys: prereq,
                                    len: next_len,
                                });

                                visited.insert((next_pos, prereq));
                            }

                            Wall => unreachable!(),
                        }
                    }
                }
            }

            self.moves.insert(from, moves);
        }

        self.moves.get(&from).unwrap()
    }
}

#[derive(Eq, PartialEq)]
struct State<'world> {
    world: &'world World,
    poss: Vec<Point>,
    collected: KeySet,
    len: usize,
}

impl<'world> State<'world> {
    #[allow(dead_code)]
    fn print_state(&self) {
        println!(
            "{}",
            self.world
                .tiles
                .iter()
                .enumerate()
                .map(|(r, row)| row
                    .iter()
                    .enumerate()
                    .map(|(c, tile)| if self.poss.contains(&(c, r)) {
                        '@'.to_string()
                    } else {
                        match tile {
                            Wall => '#'.to_string(),
                            Floor => '.'.to_string(),
                            Key(a) => a.to_char().to_string(),
                            Door(a) => a.to_char().to_ascii_uppercase().to_string(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(""))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.len.cmp(&self.len)
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_world(lines: &[String]) -> (World, Point) {
    let mut player_pos = (0, 0);
    let mut keys = KeySet::new();
    let tiles = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Wall,
                    '.' => Floor,
                    '@' => {
                        player_pos = (x, y);
                        Floor
                    }
                    a => {
                        if a == a.to_ascii_uppercase() {
                            Door(a.into())
                        } else {
                            keys = keys.with(a.into());
                            Key(a.into())
                        }
                    }
                })
                .collect()
        })
        .collect();

    (World { tiles, keys }, player_pos)
}

fn dijkstra<'world>(world: &'world World, start_positions: &[Point]) -> Option<State<'world>> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut shortest_paths: HashMap<(KeySet, Point, Vec<Point>), usize> = HashMap::new();

    let mut navigation = Navigation {
        world: &world,
        moves: HashMap::new(),
    };

    queue.push(State {
        world,
        poss: start_positions.to_vec(),
        collected: KeySet::new(),
        len: 0,
    });

    while let Some(state) = queue.pop() {
        if state.collected == world.keys {
            return Some(state);
        } else {
            for (posi, pos) in state.poss.iter().enumerate() {
                let shortest = shortest_paths
                    .entry((state.collected, *pos, state.poss.clone()))
                    .or_insert(state.len + 1);
                if state.len < *shortest {
                    *shortest = state.len;

                    for route in navigation
                        .available_moves(*pos)
                        .iter()
                        .filter(|route| state.collected.contains_all(route.prerequired_keys))
                    {
                        let collected = state.collected.union(route.collected_keys);
                        let mut poss = state.poss.clone();
                        poss[posi] = route.to;

                        queue.push(State {
                            world,
                            poss,
                            collected,
                            len: state.len + route.len,
                        });
                    }
                }
            }
        }
    }
    None
}

fn solve_a(world: &World, pos: Point) -> usize {
    let found = dijkstra(world, &[pos]);
    found.unwrap().len
}

fn solve_b(mut world: World, pos: Point) -> usize {
    world.tiles[pos.1 - 1][pos.0] = Tile::Wall;
    world.tiles[pos.1][pos.0 - 1] = Tile::Wall;
    world.tiles[pos.1 + 1][pos.0] = Tile::Wall;
    world.tiles[pos.1][pos.0 + 1] = Tile::Wall;

    let pos = vec![
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0 + 1, pos.1 - 1),
    ];

    let found = dijkstra(&world, &pos);
    found.unwrap().len
}

pub fn solve(lines: &[String]) -> Solution {
    let (world, pos) = parse_world(lines);
    let a_solution = solve_a(&world, pos);
    let b_solution = solve_b(world, pos);
    (a_solution.to_string(), b_solution.to_string())
}

#[cfg(test)]
mod tests {
    use super::Point;
    use super::World;

    fn parse(input: &str) -> (World, Point) {
        let lines: Vec<String> = input.trim().lines().map(|l| l.trim().to_string()).collect();
        super::parse_world(&lines)
    }

    fn check_a(expected_output: usize, input: &str) {
        let (world, pos) = parse(input);
        let solution = super::solve_a(&world, pos);
        assert_eq!(solution, expected_output);
    }

    fn check_b(expected_output: usize, input: &str) {
        let (world, pos) = parse(input);
        let solution = super::solve_b(world, pos);
        assert_eq!(solution, expected_output);
    }

    #[test]
    fn example_a1() {
        let input = "
        #########
        #b.A.@.a#
        #########
        ";
        check_a(8, input);
    }

    #[test]
    fn example_a2() {
        let input = "
        ########################
        #f.D.E.e.C.b.A.@.a.B.c.#
        ######################.#
        #d.....................#
        ########################
        ";
        check_a(86, input);
    }

    #[test]
    fn example_a3() {
        let input = "
        ########################
        #...............b.C.D.f#
        #.######################
        #.....@.a.B.c.d.A.e.F.g#
        ########################
        ";
        check_a(132, input);
    }

    #[test]
    fn example_a4() {
        let input = "
        #################
        #i.G..c...e..H.p#
        ########.########
        #j.A..b...f..D.o#
        ########@########
        #k.E..a...g..B.n#
        ########.########
        #l.F..d...h..C.m#
        #################
        ";
        check_a(136, input);
    }

    #[test]
    fn example_a5() {
        let input = "
        ########################
        #@..............ac.GI.b#
        ###d#e#f################
        ###A#B#C################
        ###g#h#i################
        ########################
        ";
        check_a(81, input);
    }

    #[test]
    fn example_b1() {
        let input = "
        #######
        #a.#Cd#
        ##...##
        ##.@.##
        ##...##
        #cB#Ab#
        #######
        ";
        check_b(8, input);
    }

    #[test]
    fn example_b2() {
        let input = "
        ###############
        #d.ABC.#.....a#
        ######...######
        ######.@.######
        ######...######
        #b.....#.....c#
        ###############
        ";
        check_b(24, input);
    }

    #[test]
    fn example_b3() {
        let input = "
        #############
        #DcBa.#.GhKl#
        #.###...#I###
        #e#d#.@.#j#k#
        ###C#...###J#
        #fEbA.#.FgHi#
        #############
        ";
        check_b(32, input);
    }

    #[test]
    fn example_b4() {
        let input = "
        #############
        #g#f.D#..h#l#
        #F###e#E###.#
        #dCba...BcIJ#
        #####.@.#####
        #nK.L...G...#
        #M###N#H###.#
        #o#m..#i#jk.#
        #############
        ";
        check_b(72, input);
    }
}
