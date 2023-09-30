use crate::rand::MersenneTwister;
use core::fmt;
use std::collections::HashMap;

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
    x: u32,
    y: u32,
}

impl Point {
    #[must_use]
    pub fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "x: {}, y: {}", self.x, self.y)?;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Size {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Size { width, height }
    }
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    position: Point,
    size: Size,
}

impl Rectangle {
    #[must_use]
    pub fn new(position: Point, size: Size) -> Self {
        Rectangle { position, size }
    }

    #[must_use]
    pub fn intersects(&self, other: &Rectangle) -> bool {
        let x_intersect: bool = ((self.position.x + self.size.width) > other.position.x)
            && (other.position.x + (other.size.width) > self.position.x);
        let y_intersect: bool = ((self.position.y + self.size.height) > other.position.y)
            && (other.position.y + (other.size.height) > self.position.y);
        x_intersect && y_intersect
    }
}

pub struct TreeNode {
    position: Point,
    size: Size,
    left_child: Option<Box<TreeNode>>,
    right_child: Option<Box<TreeNode>>,
    room: Option<Rectangle>,
    corridors: Vec<Rectangle>,
}

impl TreeNode {
    #[must_use]
    pub fn new(position: Point, size: Size) -> Self {
        TreeNode {
            position,
            size,
            left_child: None,
            right_child: None,
            room: None,
            corridors: Vec::new(),
        }
    }

    fn split(
        &mut self,
        rng: &mut MersenneTwister,
        min_room_size: Size,
        max_room_size: Size,
    ) -> bool {
        if self.left_child.is_some() || self.right_child.is_some() {
            return false;
        }

        let split_horizontal: bool;
        if (self.size.width > self.size.height) && (self.size.width * 100) / self.size.height >= 125
        {
            split_horizontal = false;
        } else if (self.size.height > self.size.width)
            && (self.size.height * 100) / self.size.width >= 125
        {
            split_horizontal = true;
        } else {
            split_horizontal = rng.generate_range(0, 1) == 1;
        }

        let side_length = if split_horizontal {
            rng.generate_range(min_room_size.height, max_room_size.height)
        } else {
            rng.generate_range(min_room_size.width, max_room_size.width)
        };

        if split_horizontal {
            if side_length >= self.size.height
                || side_length < min_room_size.height
                || self.size.height - side_length < min_room_size.height
            {
                return false;
            }

            self.left_child = Some(Box::new(TreeNode::new(
                Point::new(self.position.x, self.position.y),
                Size::new(self.size.width, side_length),
            )));
            self.right_child = Some(Box::new(TreeNode::new(
                Point::new(self.position.x, self.position.y + side_length),
                Size::new(self.size.width, self.size.height - side_length),
            )));
        } else {
            if side_length >= self.size.width
                || side_length < min_room_size.width
                || self.size.width - side_length < min_room_size.width
            {
                return false;
            }

            self.left_child = Some(Box::new(TreeNode::new(
                Point::new(self.position.x, self.position.y),
                Size::new(side_length, self.size.height),
            )));
            self.right_child = Some(Box::new(TreeNode::new(
                Point::new(self.position.x + side_length, self.position.y),
                Size::new(self.size.width - side_length, self.size.height),
            )));
        }

        true
    }

    fn is_leaf(&self) -> bool {
        self.left_child.is_none() && self.right_child.is_none()
    }

    fn generate(&mut self, rng: &mut MersenneTwister, min_room_size: Size, max_room_size: Size) {
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

    fn create_rooms(&mut self, rng: &mut MersenneTwister) {
        if self.left_child.is_some() || self.right_child.is_some() {
            if self.left_child.is_some() {
                self.left_child.as_mut().unwrap().create_rooms(rng);
            }
            if self.right_child.is_some() {
                self.right_child.as_mut().unwrap().create_rooms(rng);
            }

            if self.left_child.is_some() && self.right_child.is_some() {
                let l_room = self.left_child.as_ref().unwrap().get_room(rng);
                let r_room = self.right_child.as_ref().unwrap().get_room(rng);

                if let (Some(l), Some(r)) = (l_room, r_room) {
                    self.create_hall(rng, l, r);
                }
            }
        } else {
            let room_size = Size::new(
                rng.generate_range(3, self.size.width - 2),
                rng.generate_range(3, self.size.height - 2),
            );
            let room_pos = Point::new(
                rng.generate_range(1, self.size.width - room_size.width - 1),
                rng.generate_range(1, self.size.height - room_size.height - 1),
            );

            self.room = Some(Rectangle::new(
                Point::new(self.position.x + room_pos.x, self.position.y + room_pos.y),
                room_size,
            ));
        }
    }

    fn get_room(&self, rng: &mut MersenneTwister) -> Option<Rectangle> {
        if let Some(r) = self.room {
            Some(r)
        } else {
            let l_room = if self.left_child.is_some() {
                self.left_child.as_ref().unwrap().get_room(rng)
            } else {
                None
            };
            let r_room = if self.right_child.is_some() {
                self.right_child.as_ref().unwrap().get_room(rng)
            } else {
                None
            };

            if l_room.is_none() && r_room.is_none() {
                None
            } else if r_room.is_none() {
                l_room
            } else if l_room.is_none() {
                r_room
            } else if rng.generate_range(0, 1) == 1 {
                l_room
            } else {
                r_room
            }
        }
    }

    fn create_hall(&mut self, rng: &mut MersenneTwister, l_room: Rectangle, r_room: Rectangle) {
        self.corridors.clear();

        let point1 = Point::new(
            rng.generate_range(
                l_room.position.x + 1,
                l_room.position.x + l_room.size.width - 2,
            ),
            rng.generate_range(
                l_room.position.y + 1,
                l_room.position.y + l_room.size.height - 2,
            ),
        );
        let point2 = Point::new(
            rng.generate_range(
                r_room.position.x + 1,
                r_room.position.x + r_room.size.width - 2,
            ),
            rng.generate_range(
                r_room.position.y + 1,
                r_room.position.y + r_room.size.height - 2,
            ),
        );
        let width = i64::from(point2.x) - i64::from(point1.x);
        let heigh = i64::from(point2.y) - i64::from(point1.y);

        if width < 0 {
            if heigh < 0 {
                if rng.generate_range(0, 1) == 1 {
                    self.corridors.push(Rectangle::new(
                        Point::new(point2.x, point1.y),
                        Size::new(width.abs() as u32, 1),
                    ));
                    self.corridors.push(Rectangle::new(
                        Point::new(point2.x, point2.y),
                        Size::new(1, heigh.abs() as u32),
                    ));
                } else {
                    self.corridors.push(Rectangle::new(
                        Point::new(point2.x, point2.y),
                        Size::new(width.abs() as u32, 1),
                    ));
                    self.corridors.push(Rectangle::new(
                        Point::new(point1.x, point2.y),
                        Size::new(1, heigh.abs() as u32),
                    ));
                };
            } else if heigh > 0 {
                if rng.generate_range(0, 1) == 1 {
                    self.corridors.push(Rectangle::new(
                        Point::new(point2.x, point1.y),
                        Size::new(width.abs() as u32, 1),
                    ));
                    self.corridors.push(Rectangle::new(
                        Point::new(point2.x, point1.y),
                        Size::new(1, heigh.abs() as u32),
                    ));
                } else {
                    self.corridors.push(Rectangle::new(
                        Point::new(point2.x, point2.y),
                        Size::new(width.abs() as u32, 1),
                    ));
                    self.corridors.push(Rectangle::new(
                        Point::new(point1.x, point1.y),
                        Size::new(1, heigh.abs() as u32),
                    ));
                };
            } else {
                self.corridors.push(Rectangle::new(
                    Point::new(point2.x, point2.y),
                    Size::new(width.abs() as u32, 1),
                ));
            };
        } else if width > 0 {
            if heigh < 0 {
                if rng.generate_range(0, 1) == 1 {
                    self.corridors.push(Rectangle::new(
                        Point::new(point1.x, point2.y),
                        Size::new(width.abs() as u32, 1),
                    ));
                    self.corridors.push(Rectangle::new(
                        Point::new(point1.x, point2.y),
                        Size::new(1, heigh.abs() as u32),
                    ));
                } else {
                    self.corridors.push(Rectangle::new(
                        Point::new(point1.x, point1.y),
                        Size::new(width.abs() as u32, 1),
                    ));
                    self.corridors.push(Rectangle::new(
                        Point::new(point2.x, point2.y),
                        Size::new(1, heigh.abs() as u32),
                    ));
                };
            } else if heigh > 0 {
                if rng.generate_range(0, 1) == 1 {
                    self.corridors.push(Rectangle::new(
                        Point::new(point1.x, point1.y),
                        Size::new(width.abs() as u32, 1),
                    ));
                    self.corridors.push(Rectangle::new(
                        Point::new(point2.x, point1.y),
                        Size::new(1, heigh.abs() as u32),
                    ));
                } else {
                    self.corridors.push(Rectangle::new(
                        Point::new(point1.x, point2.y),
                        Size::new(width.abs() as u32, 1),
                    ));
                    self.corridors.push(Rectangle::new(
                        Point::new(point1.x, point1.y),
                        Size::new(1, heigh.abs() as u32),
                    ));
                };
            } else {
                self.corridors.push(Rectangle::new(
                    Point::new(point1.x, point1.y),
                    Size::new(width.abs() as u32, 1),
                ));
            };
        } else if heigh < 0 {
            self.corridors.push(Rectangle::new(
                Point::new(point2.x, point2.y),
                Size::new(1, heigh.abs() as u32),
            ));
        } else if heigh > 0 {
            self.corridors.push(Rectangle::new(
                Point::new(point1.x, point1.y),
                Size::new(1, heigh.abs() as u32),
            ));
        }
    }

    fn iter(&self) -> TreeNodeIterator {
        TreeNodeIterator::new(&self)
    }
}

pub struct BSPMap {
    size: Size,
    tiles: HashMap<Point, Tile>,
    min_room_size: Size,
    max_room_size: Size,
}

impl BSPMap {
    pub fn new(
        size: Size,
        mut seed: MersenneTwister,
        min_room_size: Size,
        max_room_size: Size,
    ) -> Result<Self, String> {
        if size.width < 20 || size.height < 20 {
            return Err(
                "Size of a BSP_Map needs to be greater than or equal width : 20, height : 20"
                    .to_string(),
            );
        }
        if min_room_size.width < 6 || min_room_size.height < 6 {
            return Err(
                "Minimum room size needs to be greater than or equal width : 6, height : 6"
                    .to_string(),
            );
        }
        if min_room_size.width >= max_room_size.width {
            return Err(
                "Minimum room size (width) needs to be less than maximum room size (width)."
                    .to_string(),
            );
        }
        if min_room_size.height >= max_room_size.height {
            return Err(
                "Minimum room size (height) needs to be less than maximum room size (height)."
                    .to_string(),
            );
        }
        if max_room_size.width >= size.width {
            return Err(
                "Maximum room size (width) must be less than map size (width).".to_string(),
            );
        }
        if max_room_size.height >= size.height {
            return Err(
                "Maximum room size (height) must be less than map size (height).".to_string(),
            );
        }

        let mut map = BSPMap {
            size,
            tiles: HashMap::new(),
            min_room_size,
            max_room_size,
        };

        map.place_rooms(&mut seed);

        map.init_walls();

        Ok(map)
    }

    fn place_rooms(&mut self, rng: &mut MersenneTwister) {
        let mut root = TreeNode::new(Point { x: 0, y: 0 }, self.size);

        root.generate(rng, self.min_room_size, self.max_room_size);
        root.create_rooms(rng);

        for node in root.iter() {
            if node.is_leaf() {
                if let Some(room) = node.get_room(rng) {
                    self.add_room(&room);
                }
            }

            for corridor in &node.corridors {
                self.add_room(corridor);
            }
        }
    }

    fn add_room(&mut self, room: &Rectangle) {
        for x in 0..room.size.width {
            for y in 0..room.size.height {
                self.tiles.insert(
                    Point::new(room.position.x + x, room.position.y + y),
                    Tile::Floor,
                );
            }
        }
    }

    fn init_walls(&mut self) {
        for y in 0..self.size.height {
            self.tiles.insert(Point::new(0, y), Tile::Wall);
            self.tiles
                .insert(Point::new(self.size.width - 1, y), Tile::Wall);
        }

        for x in 0..self.size.width {
            self.tiles.insert(Point::new(x, 0), Tile::Wall);
            self.tiles
                .insert(Point::new(x, self.size.height - 1), Tile::Wall);
        }

        let mut walls: Vec<Point> = Vec::new();

        for tile in &self.tiles {
            if self
                .tiles
                .get(&Point::new(tile.0.x + 1, tile.0.y))
                .is_none()
            {
                walls.push(Point::new(tile.0.x + 1, tile.0.y));
            }
            if self
                .tiles
                .get(&Point::new(tile.0.x + 1, tile.0.y + 1))
                .is_none()
            {
                walls.push(Point::new(tile.0.x + 1, tile.0.y + 1));
            }
            if self
                .tiles
                .get(&Point::new(tile.0.x, tile.0.y + 1))
                .is_none()
            {
                walls.push(Point::new(tile.0.x, tile.0.y + 1));
            }
            if tile.0.x != 0
                && self
                    .tiles
                    .get(&Point::new(tile.0.x - 1, tile.0.y + 1))
                    .is_none()
            {
                walls.push(Point::new(tile.0.x - 1, tile.0.y + 1));
            }
            if tile.0.x != 0
                && self
                    .tiles
                    .get(&Point::new(tile.0.x - 1, tile.0.y))
                    .is_none()
            {
                walls.push(Point::new(tile.0.x - 1, tile.0.y));
            }
            if tile.0.x != 0
                && tile.0.y != 0
                && self
                    .tiles
                    .get(&Point::new(tile.0.x - 1, tile.0.y - 1))
                    .is_none()
            {
                walls.push(Point::new(tile.0.x - 1, tile.0.y - 1));
            }
            if tile.0.y != 0
                && self
                    .tiles
                    .get(&Point::new(tile.0.x, tile.0.y - 1))
                    .is_none()
            {
                walls.push(Point::new(tile.0.x, tile.0.y - 1));
            }
            if tile.0.y != 0
                && self
                    .tiles
                    .get(&Point::new(tile.0.x + 1, tile.0.y - 1))
                    .is_none()
            {
                walls.push(Point::new(tile.0.x + 1, tile.0.y - 1));
            }
        }

        for wall in &walls {
            self.tiles.insert(*wall, Tile::Wall);
        }
    }
}

impl fmt::Display for BSPMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.size.width {
            for col in 0..self.size.height {
                match self.tiles.get(&Point::new(row, col)) {
                    Some(x) => write!(f, "{x}")?,
                    None => write!(f, "x")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct TreeNodeIterator<'a> {
    current_node: Option<&'a TreeNode>,
    right_nodes: Vec<&'a TreeNode>,
}

impl<'a> TreeNodeIterator<'a> {
    fn new(root: &'a TreeNode) -> TreeNodeIterator<'a> {
        let mut iter = TreeNodeIterator {
            right_nodes: vec![],
            current_node: None,
        };

        iter.add_left_subtree(root);
        iter
    }

    fn add_left_subtree(&mut self, node: &'a TreeNode) {
        if let Some(ref left) = node.left_child {
            self.right_nodes.push(&*left);
        }
        if let Some(ref right) = node.right_child {
            self.right_nodes.push(&*right);
        }

        self.current_node = Some(node);
    }
}

impl<'a> Iterator for TreeNodeIterator<'a> {
    type Item = &'a TreeNode;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_node.take();
        if let Some(rest) = self.right_nodes.pop() {
            self.add_left_subtree(rest);
        }

        match result {
            Some(node) => Some(&*node),
            None => None,
        }
    }
}
