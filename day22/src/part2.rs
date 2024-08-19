use advent::coord::{ContainsCoord, Coord};

use crate::{Direction, Field};

const CUBE_FACES: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug, dbg_pls::DebugPls)]
#[repr(u8)]
enum Face {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

impl Face {
    fn neighbours_clockwise(face: Face) -> &'static [Face; 4] {
        use Face::*;

        match face {
            Top => &[Right, Front, Left, Back],
            Bottom => &[Back, Left, Front, Right],
            Left => &[Front, Bottom, Back, Top],
            Right => &[Top, Back, Bottom, Front],
            Front => &[Top, Right, Bottom, Left],
            Back => &[Left, Bottom, Right, Top],
        }
    }
}

struct Cube {
    face_len: usize,
    maps: [Vec<Vec<Field>>; CUBE_FACES],
    map_coords: [(usize, usize); CUBE_FACES],
    neighbours: [[usize; 4]; CUBE_FACES],
    instructions: Vec<Vec<usize>>,
}

impl Cube {
    fn new(input: &str) -> Cube {
        let (grid_lines, instruction_line) = super::split_grid_and_instruction(input);

        let grid = super::make_grid(&grid_lines);
        let instructions = super::parse_instruction_line(instruction_line);
        let face_len = Self::face_len(&grid);

        let (cube_map, map_coords, maps) = Self::build_cube_maps(&grid, face_len);
        let neighbours = WalkNeighbours::walk(&map_coords, &cube_map);

        Cube {
            face_len,
            maps,
            map_coords,
            neighbours,
            instructions,
        }
    }

    fn face_len(grid: &Vec<Vec<Field>>) -> usize {
        let cube_blocks = grid
            .iter()
            .map(|l| l.iter().filter(|&&el| el != Field::OutOfMap).count())
            .sum::<usize>();

        let face_length_f = ((cube_blocks / CUBE_FACES) as f64).sqrt() + f64::EPSILON;
        let face_length_u = face_length_f as usize;

        assert!((face_length_f - face_length_u as f64).abs() <= f64::EPSILON);
        face_length_u
    }

    fn build_cube_maps(
        grid: &Vec<Vec<Field>>,
        face_len: usize,
    ) -> (
        Vec<Vec<Option<usize>>>,
        [(usize, usize); CUBE_FACES],
        [Vec<Vec<Field>>; CUBE_FACES],
    ) {
        let cube_map_height = grid.len() / face_len;
        let cube_map_width = grid.iter().map(|l| l.len()).max().unwrap() / face_len;

        let mut cube_map = vec![vec![None; cube_map_width]; cube_map_height];
        let mut maps = Vec::with_capacity(CUBE_FACES);
        let mut coords = Vec::with_capacity(CUBE_FACES);

        for y in 0..cube_map_height {
            for x in 0..cube_map_width {
                let (ox, oy) = (x * face_len, y * face_len);
                if ox >= grid[oy].len() || grid[oy][ox] == Field::OutOfMap {
                    continue;
                }

                cube_map[y][x] = Some(maps.len());
                coords.push((y, x));

                maps.push(
                    grid.iter()
                        .skip(oy)
                        .take(face_len)
                        .map(|l| l.iter().skip(ox).take(face_len).copied().collect())
                        .collect(),
                );
            }
        }

        (
            cube_map,
            coords.try_into().unwrap(),
            maps.try_into().unwrap(),
        )
    }

    fn is_in_face(&self, coord: Coord) -> bool {
        coord.y >= 0
            && (coord.y as usize) < self.face_len
            && coord.x >= 0
            && (coord.x as usize) < self.face_len
    }

    fn modulo_face(&self, coord: Coord) -> Coord {
        Coord::new(
            (coord.x + self.face_len as isize) % self.face_len as isize,
            (coord.y + self.face_len as isize) % self.face_len as isize,
        )
    }

    fn walk(&mut self) -> isize {
        let mut curr_pos = Coord::zero();
        let mut curr_face = 0usize;
        let mut curr_dir = Direction::East;

        for (right_idx, right_steps) in self.instructions.iter().enumerate() {
            if right_idx > 0 {
                curr_dir = curr_dir.turn_right();
            }

            for (left_idx, &steps) in right_steps.iter().enumerate() {
                if left_idx > 0 {
                    curr_dir = curr_dir.turn_left();
                }

                for _ in 0..steps {
                    eprintln!(
                        "{} (face {}, pos {}), direction: {:?}",
                        self.coords(curr_face, curr_pos),
                        curr_face,
                        curr_pos,
                        curr_dir
                    );

                    let next_pos = curr_pos + curr_dir.step();
                    if self.is_in_face(next_pos) {
                        match self.maps[curr_face][next_pos] {
                            Field::OutOfMap => unreachable!(),
                            Field::Empty => curr_pos = next_pos,
                            Field::Wall => break,
                        }

                        continue;
                    }

                    let next_face = self.neighbours[curr_face][curr_dir as usize];
                    let mut next_dir = curr_dir;
                    let mut next_pos = self.modulo_face(next_pos);

                    while self.neighbours[next_face][next_dir.opposite() as usize] != curr_face {
                        next_dir = next_dir.turn_right();
                        next_pos = Coord::new(self.face_len as isize - 1 - next_pos.y, next_pos.x)
                    }

                    match self.maps[next_face][next_pos] {
                        Field::OutOfMap => unreachable!(),
                        Field::Empty => {
                            eprintln!(
                                "  moving to {} (face {}, pos {}), direction: {:?}",
                                self.coords(next_face, next_pos),
                                next_face,
                                next_pos,
                                next_dir
                            );

                            curr_pos = next_pos;
                            curr_face = next_face;
                            curr_dir = next_dir;
                        }
                        Field::Wall => break,
                    }
                }
            }
        }

        eprintln!(
            "Curr face: {}, pos: {}, dir: {:?}, coords: {}",
            curr_face,
            curr_pos,
            curr_dir,
            self.coords(curr_face, curr_pos)
        );

        super::calc_coordinate(self.coords(curr_face, curr_pos), curr_dir)
    }

    fn coords(&self, face_idx: usize, pos: Coord) -> Coord {
        let (oy, ox) = self.map_coords[face_idx];
        pos + Coord::new((ox * self.face_len) as isize, (oy * self.face_len) as isize)
    }
}

struct WalkNeighbours<'a> {
    coords: &'a [(usize, usize); CUBE_FACES],
    map: &'a Vec<Vec<Option<usize>>>,
    neighbours: [[Option<Face>; 4]; CUBE_FACES],
    face_indices: [Option<usize>; CUBE_FACES],
    filled: [bool; CUBE_FACES],
}

impl<'a> WalkNeighbours<'a> {
    fn walk(
        coords: &'a [(usize, usize); CUBE_FACES],
        map: &'a Vec<Vec<Option<usize>>>,
    ) -> [[usize; 4]; CUBE_FACES] {
        let mut walk = WalkNeighbours {
            coords,
            map,
            neighbours: [[None; 4]; CUBE_FACES],
            face_indices: [None; CUBE_FACES],
            filled: [false; CUBE_FACES],
        };

        walk.rec(0, Face::Top, (Direction::East, Face::Right));
        walk.neighbours
            .map(|f| f.map(|n| walk.face_indices[n.unwrap() as usize].unwrap()))
    }

    fn rec(&mut self, curr_face_idx: usize, curr_face: Face, known_neighbour: (Direction, Face)) {
        if self.filled[curr_face_idx] {
            return;
        }

        self.fill_neighbours(curr_face, curr_face_idx, known_neighbour);
        self.filled[curr_face_idx] = true;

        let mut walk_in_dir = |direction: Direction| {
            let step = direction.step();
            let curr_coord = Coord::new(
                self.coords[curr_face_idx].1 as isize,
                self.coords[curr_face_idx].0 as isize,
            );
            let new_coord = curr_coord + step;

            if !self.map.contains_coord(new_coord) {
                return;
            }

            let Some(new_face_idx) = self.map[new_coord] else {
                return;
            };

            let new_face = self.neighbours[curr_face_idx][direction as usize].unwrap();
            self.rec(new_face_idx, new_face, (direction.opposite(), curr_face));
        };

        for dir in Direction::CLOCKWISE {
            walk_in_dir(dir);
        }
    }

    fn fill_neighbours(&mut self, face: Face, face_idx: usize, known_neighbour: (Direction, Face)) {
        self.face_indices[face as usize] = Some(face_idx);
        let (known_dir, known_face) = known_neighbour;

        let dirs = Direction::CLOCKWISE
            .iter()
            .cycle()
            .skip_while(|&&d| d != known_dir)
            .take(4);

        let faces = Face::neighbours_clockwise(face)
            .iter()
            .cycle()
            .skip_while(|&&f| f != known_face)
            .take(4);

        for (&dir, &face) in dirs.zip(faces) {
            self.neighbours[face_idx][dir as usize] = Some(face)
        }
    }
}

pub fn part2(input: &str) -> isize {
    let mut cube = Cube::new(input);
    cube.walk()
}
