use core::fmt;
use std::collections::HashMap;

use crate::rand::MersenneTwister;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    Floor,
    Wall,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Floor => write!(f, "0"),
            Tile::Wall => write!(f, "1"),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}\n", self.x, self.y)?;
        Ok(())
    }
}

pub struct BSPMap {
    size: Point,
    tiles: HashMap<Point, Tile>,
    rooms: Vec<Room>,
    min_room_size: Point,
    max_room_size: Point,
}
impl BSPMap {
    pub fn new(
        size: Point,
        mut seed: MersenneTwister,
        min_room_size: Point,
        max_room_size: Point,
    ) -> Result<Self, String> {
        if size.x < 20 || size.y < 20 {
            return Err(
                "Size of a BSP_Map needs to be greater than or equal x : 20, y : 20".to_string(),
            );
        }
        if min_room_size.x >= max_room_size.x {
            return Err(
                "Minimum room size (x) needs to be less than maximum room size (x).".to_string(),
            );
        }
        if min_room_size.y >= max_room_size.y {
            return Err(
                "Minimum room size (y) needs to be less than maximum room size (y).".to_string(),
            );
        }
        if max_room_size.x >= size.x {
            return Err("Maximum room size (x) must be less than map size (x).".to_string());
        }
        if max_room_size.y >= size.y {
            return Err("Maximum room size (y) must be less than map size (y).".to_string());
        }
        let mut map = BSPMap {
            size,
            tiles: HashMap::new(),
            rooms: Vec::new(),
            min_room_size,
            max_room_size,
        };
        map.place_rooms(&mut seed);

        for y in 0..size.y {
            map.tiles.insert(Point::new(0, y), Tile::Wall);
            map.tiles.insert(Point::new(map.size.x, y), Tile::Wall);
        }

        for x in 0..size.x {
            map.tiles.insert(Point::new(x, 0), Tile::Wall);
            map.tiles.insert(Point::new(x, map.size.y), Tile::Wall);
        }
        let mut walls: Vec<Point> = Vec::new();

        for tile in map.tiles.iter() {
            if map.tiles.get(&Point::new(tile.0.x + 1, tile.0.y)).is_none() {
                walls.push(Point::new(tile.0.x + 1, tile.0.y))
            }
            if map
                .tiles
                .get(&Point::new(tile.0.x + 1, tile.0.y + 1))
                .is_none()
            {
                walls.push(Point::new(tile.0.x + 1, tile.0.y + 1))
            }
            if map.tiles.get(&Point::new(tile.0.x, tile.0.y + 1)).is_none() {
                walls.push(Point::new(tile.0.x, tile.0.y + 1))
            }
            if tile.0.x != 0
                && map
                    .tiles
                    .get(&Point::new(tile.0.x - 1, tile.0.y + 1))
                    .is_none()
            {
                walls.push(Point::new(tile.0.x - 1, tile.0.y + 1))
            }
            if tile.0.x != 0 && map.tiles.get(&Point::new(tile.0.x - 1, tile.0.y)).is_none() {
                walls.push(Point::new(tile.0.x - 1, tile.0.y))
            }
            if tile.0.x != 0
                && tile.0.y != 0
                && map
                    .tiles
                    .get(&Point::new(tile.0.x - 1, tile.0.y - 1))
                    .is_none()
            {
                walls.push(Point::new(tile.0.x - 1, tile.0.y - 1))
            }
            if tile.0.y != 0 && map.tiles.get(&Point::new(tile.0.x, tile.0.y - 1)).is_none() {
                walls.push(Point::new(tile.0.x, tile.0.y - 1))
            }
            if tile.0.y != 0
                && map
                    .tiles
                    .get(&Point::new(tile.0.x + 1, tile.0.y - 1))
                    .is_none()
            {
                walls.push(Point::new(tile.0.x + 1, tile.0.y - 1))
            }
        }

        for wall in walls.iter() {
            map.tiles.insert(wall.clone(), Tile::Wall);
        }
        Ok(map)
    }

    pub fn get_tiles(&self) -> &HashMap<Point, Tile> {
        &self.tiles
    }

    pub fn get_size(&self) -> &Point {
        &self.size
    }

    pub fn get_rooms(&self) -> &Vec<Room> {
        &&self.rooms
    }
    fn place_rooms(&mut self, rng: &mut MersenneTwister) {
        let mut root = Leaf::new(Point { x: 0, y: 0 }, self.size);

        root.generate(rng, &self.min_room_size, &self.max_room_size);

        root.create_rooms(rng, &self.min_room_size);

        for leaf in root.iter() {
            if leaf.is_leaf() {
                if let Some(room) = leaf.get_room(rng) {
                    self.add_room(&room);
                }
            }

            for corridor in &leaf.corridors {
                self.add_room(&corridor);
            }
        }
    }

    pub fn add_room(&mut self, room: &Room) {
        for x in 0..room.size.x {
            for y in 0..room.size.y {
                self.tiles.insert(
                    Point::new(room.position.x + x, room.position.y + y),
                    Tile::Floor,
                );
            }
        }
        self.rooms.push(room.clone());
    }
}

impl fmt::Display for BSPMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..=self.size.x {
            for col in 0..=self.size.y {
                match self.tiles.get(&Point::new(row, col)) {
                    Some(x) => write!(f, "{}", x)?,
                    None => write!(f, "x")?,
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Room {
    position: Point,
    size: Point,
}

impl Room {
    pub fn new(position: Point, size: Point) -> Self {
        Room { position, size }
    }
    pub fn intersects(&self, other: &Room) -> bool {
        let x_intersect: bool = ((self.position.x + self.size.x) > other.position.x)
            && (other.position.x + (other.size.x) > self.position.x);
        let y_intersect: bool = ((self.position.y + self.size.y) > other.position.y)
            && (other.position.y + (other.size.y) > self.position.y);
        x_intersect && y_intersect
    }
}

pub struct Leaf {
    position: Point,
    size: Point,
    left_child: Option<Box<Leaf>>,
    right_child: Option<Box<Leaf>>,
    room: Option<Room>,
    corridors: Vec<Room>,
}

impl Leaf {
    pub fn new(position: Point, size: Point) -> Self {
        Leaf {
            position,
            size,
            left_child: None,
            right_child: None,
            room: None,
            corridors: Vec::new(),
        }
    }
    pub fn split(
        &mut self,
        rng: &mut MersenneTwister,
        min_room_size: &Point,
        max_room_size: &Point,
    ) -> bool {
        if self.left_child.is_some() || self.right_child.is_some() {
            return false;
        }

        let split_horizontal: bool;

        if (self.size.x > self.size.y) && (self.size.x as u64 * 100 / self.size.y as u64) >= 125 {
            split_horizontal = false;
        } else if (self.size.y > self.size.x)
            && (self.size.y as u64 * 100 / self.size.x as u64) >= 125
        {
            split_horizontal = true;
        } else {
            split_horizontal = rng.generate_range(0, 1) == 1;
        };

        let split = if split_horizontal {
            rng.generate_range(min_room_size.x as u32, max_room_size.x as u32)
        } else {
            rng.generate_range(min_room_size.y as u32, max_room_size.y as u32)
        } as usize;

        if split_horizontal {
            if split >= self.size.y {
                return false;
            }
            self.left_child = Some(Box::new(Leaf::new(
                Point::new(self.position.x, self.position.y),
                Point::new(self.size.x, split),
            )));
            self.right_child = Some(Box::new(Leaf::new(
                Point::new(self.position.x, self.position.y + split),
                Point::new(self.size.x, self.size.y - split),
            )));
        } else {
            if split >= self.size.x {
                return false;
            }
            self.left_child = Some(Box::new(Leaf::new(
                Point::new(self.position.x, self.position.y),
                Point::new(split, self.size.y),
            )));

            self.right_child = Some(Box::new(Leaf::new(
                Point::new(self.position.x + split, self.position.y),
                Point::new(self.size.x - split, self.size.y),
            )));
        }
        true
    }

    fn is_leaf(&self) -> bool {
        self.left_child.is_none() && self.right_child.is_none()
    }

    fn generate(
        &mut self,
        rng: &mut MersenneTwister,
        min_room_size: &Point,
        max_room_size: &Point,
    ) {
        if self.is_leaf() && self.split(rng, min_room_size, max_room_size) {
            self.left_child
                .as_mut()
                .unwrap()
                .generate(rng, min_room_size, max_room_size);
            self.right_child
                .as_mut()
                .unwrap()
                .generate(rng, min_room_size, max_room_size);
        }
    }

    fn create_rooms(&mut self, rng: &mut MersenneTwister, min_room_size: &Point) {
        if let Some(ref mut room) = self.left_child {
            room.as_mut().create_rooms(rng, min_room_size);
        };

        if let Some(ref mut room) = self.right_child {
            room.as_mut().create_rooms(rng, min_room_size);
        };

        if self.is_leaf() {
            let width: usize;
            if min_room_size.x >= self.size.x {
                width = min_room_size.x;
            } else {
                width = rng.generate_range(min_room_size.x as u32, self.size.x as u32) as usize;
            }

            let height: usize;
            if min_room_size.y >= self.size.y {
                height = min_room_size.y;
            } else {
                height = rng.generate_range(min_room_size.y as u32, self.size.y as u32) as usize;
            }
            let x: usize;
            if (self.size.x as i64 - width as i64) <= 0 {
                x = 0
            } else {
                x = rng.generate_range(0, (self.size.x - width) as u32) as usize;
            }

            let y: usize;
            if self.size.y as i64 - height as i64 <= 0 {
                y = 0
            } else {
                y = rng.generate_range(0, (self.size.y - height) as u32) as usize;
            }
            self.room = Some(Room::new(
                Point::new(x + self.position.x, y + self.position.y),
                Point::new(width, height),
            ));
        }
        if let (Some(ref mut left), Some(ref mut right)) =
            (&mut self.left_child, &mut self.right_child)
        {
            create_corridors(rng, left, right);
        };
    }

    fn get_room(&self, rng: &mut MersenneTwister) -> Option<Room> {
        if self.is_leaf() {
            return self.room;
        }

        let mut left_room: Option<Room> = None;
        let mut right_room: Option<Room> = None;

        if let Some(ref room) = self.left_child {
            left_room = room.get_room(rng);
        }

        if let Some(ref room) = self.right_child {
            right_room = room.get_room(rng);
        }
        match (left_room, right_room) {
            (None, None) => None,
            (Some(room_left), Some(room_right)) => {
                if rng.generate_range(0, 1) == 1 {
                    Some(room_left)
                } else {
                    Some(room_right)
                }
            }
            (_, Some(room)) | (Some(room), _) => Some(room),
        }
    }

    fn iter(&self) -> LeafIterator {
        LeafIterator::new(&self)
    }
}

fn create_corridors(rng: &mut MersenneTwister, left: &mut Box<Leaf>, right: &mut Box<Leaf>) {
    if let (Some(left_room), Some(right_room)) = (left.get_room(rng), right.get_room(rng)) {
        let left_point = Point::new(
            rng.generate_range(
                left_room.position.x as u32,
                (left_room.position.x + left_room.size.x) as u32,
            ) as usize,
            rng.generate_range(
                left_room.position.y as u32,
                (left_room.position.y + left_room.size.y) as u32,
            ) as usize,
        );

        let right_point = Point::new(
            rng.generate_range(
                right_room.position.x as u32,
                (right_room.position.x + right_room.size.x) as u32,
            ) as usize,
            rng.generate_range(
                right_room.position.y as u32,
                (right_room.position.y + right_room.size.y) as u32,
            ) as usize,
        );

        if left_point.y <= right_point.y {
            left.corridors
                .push(vert_corridor(left_point.x, left_point.y, right_point.y));
        } else {
            left.corridors
                .push(vert_corridor(left_point.x, right_point.y, left_point.y));
        }

        if left_point.x <= right_point.x {
            left.corridors
                .push(horz_corridor(left_point.x, right_point.y, right_point.x));
        } else {
            left.corridors
                .push(horz_corridor(right_point.x, right_point.y, left_point.x));
        }
    };
}

fn horz_corridor(start_x: usize, start_y: usize, end_x: usize) -> Room {
    Room::new(
        Point {
            x: start_x,
            y: start_y,
        },
        Point {
            x: end_x - start_x + 1,
            y: 1,
        },
    )
}

fn vert_corridor(start_x: usize, start_y: usize, end_y: usize) -> Room {
    Room::new(
        Point {
            x: start_x,
            y: start_y,
        },
        Point {
            x: 1,
            y: end_y - start_y,
        },
    )
}

struct LeafIterator<'a> {
    current_node: Option<&'a Leaf>,
    right_nodes: Vec<&'a Leaf>,
}

impl<'a> LeafIterator<'a> {
    fn new(root: &'a Leaf) -> LeafIterator<'a> {
        let mut iter = LeafIterator {
            right_nodes: vec![],
            current_node: None,
        };

        iter.add_left_subtree(root);
        iter
    }

    fn add_left_subtree(&mut self, node: &'a Leaf) {
        if let Some(ref left) = node.left_child {
            self.right_nodes.push(&*left);
        }
        if let Some(ref right) = node.right_child {
            self.right_nodes.push(&*right);
        }

        self.current_node = Some(node);
    }
}

impl<'a> Iterator for LeafIterator<'a> {
    type Item = &'a Leaf;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_node.take();
        if let Some(rest) = self.right_nodes.pop() {
            self.add_left_subtree(rest);
        }

        match result {
            Some(leaf) => Some(&*leaf),
            None => None,
        }
    }
}
