use std::{fmt::Debug, ops::Deref};

#[derive(Debug, Clone, Copy)]
struct LeafPos(u64, u8);

impl LeafPos {
	fn new() -> Self {
		Self(0, 0)
	}

	fn child(&self, offset: u8) -> Self {
		assert_eq!(offset, 0b11 & offset);
		Self(self.0 + (offset as u64) << (2 * self.1), self.1 + 1)
	}

	fn pos(&self) -> (f32, f32) {
		let (mut x, mut y): (f32, f32) = (0.5, 0.5);
		for i in 0..self.1 {
			let offset = (self.0 >> (2 * i)) & 0b11;
			let change = 0.5 / (1 << i) as f32;
			match offset {
				0b00 => {
					x -= change;
					y -= change;
				}
				0b01 => {
					x += change;
					y -= change;
				}
				0b10 => {
					x -= change;
					y += change;
				}
				0b11 => {
					x += change;
					y += change;
				}
				_ => unreachable!(),
			}
		}
		(x, y)
	}

	fn size(&self) -> f32 {
		1.0 / (1 << self.1) as f32
	}
}

#[derive(Debug)]
enum Container {
	Tree([Box<Quad>; 4]),
	Leaf(Vec<Box<dyn Pos>>),
}

impl Container {
	fn push(&mut self, item: impl Pos + 'static) {
		match self {
			Container::Leaf(vec) => vec.push(Box::new(item)),
			_ => {}
		}
	}

	fn is_filled(&self) -> bool {
		if let Container::Leaf(vec) = self {
			vec.len() >= 8
		} else {
			false
		}
	}
}

impl Default for Container {
	fn default() -> Self {
		Self::Leaf(Vec::with_capacity(8))
	}
}

#[derive(Debug)]
struct Quad {
	container: Container,
	pos: LeafPos,
}

impl Quad {
	fn new_boxed() -> Box<Self> {
		Box::new(Default::default())
	}

	fn at(pos: LeafPos) -> Self {
		Self {
			container: Default::default(),
			pos,
		}
	}

	fn new_boxed_at(pos: LeafPos) -> Box<Self> {
		Box::new(Self::at(pos))
	}

	fn collides(&self, item: &dyn Pos) -> bool {
		let (x, y) = self.pos.pos();
		let s = self.pos.size();
		let (l, r, u, d) = (x - s, x + s, y - s, y + s);
		let (x2, y2) = item.pos();
		println!("{} {} {} {}", x, y, x2, y2);
		x2 >= l && x2 < r && y2 >= u && y2 < d
	}

	/// Returns `true` if the item is in the container or intersects with it
	fn collide(&mut self, item: &dyn Pos) -> Option<&mut Quad> {
		if !self.collides(item) {
			return None;
		}
		match self.container {
			Container::Tree(ref mut tree) => tree.iter_mut().find_map(|i| i.collide(item)),
			Container::Leaf(_) => Some(self),
		}
	}

	fn push(&mut self, item: impl Pos + 'static) {
		if let Some(quad) = self.collide(&item) {
			if quad.container.is_filled() {
				quad.subdivide();
			}
			quad.container.push(item)
		}
	}

	fn subdivide(&mut self) {
		if let Container::Leaf(vec) = &self.container {
			(*self).container = {
				let mut tree = Container::Tree([
					Quad::new_boxed_at(self.pos.child(0)),
					Quad::new_boxed_at(self.pos.child(1)),
					Quad::new_boxed_at(self.pos.child(2)),
					Quad::new_boxed_at(self.pos.child(3)),
				]);
				for item in vec {}
				tree
			};
		};
	}
}

impl Deref for Quad {
	type Target = Container;
	fn deref(&self) -> &Self::Target {
		&self.container
	}
}

impl Default for Quad {
	fn default() -> Self {
		Self {
			pos: LeafPos(0, 0),
			container: Default::default(),
		}
	}
}

trait Pos: Debug {
	fn x(&self) -> f32;
	fn y(&self) -> f32;
	fn pos(&self) -> (f32, f32) {
		(self.x(), self.y())
	}
}

#[derive(Debug)]
struct Point {
	x: f32,
	y: f32,
}

impl Pos for Point {
	fn x(&self) -> f32 {
		self.x
	}
	fn y(&self) -> f32 {
		self.y
	}
}

#[derive(Debug)]
struct Quadtree {
	tree: Quad,
}

impl Default for Quadtree {
	fn default() -> Self {
		Self {
			tree: Default::default(),
		}
	}
}

impl Quadtree {
	fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	fn insert(&mut self, item: impl Pos + 'static) {
		self.tree.push(item);
	}
}

fn main() {
	let mut a = Quadtree::default();
	for i in 0..20 {
		a.insert(Point { x: 0.2, y: 0.0 })
	}

	// let mut depth = 0;
	// let mut total_x = 0.0;
	// let mut total_y = 0.0;
	// let mut counter = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
	// // 1_048_575 variations
	// for i in 0..0b11111111111111111111 {
	// 	counter[depth] += 1;

	// 	let n = ContainerMetadata(i, depth as u8);
	// 	let (x, y) = n.pos();
	// 	total_x += x;
	// 	total_y += y;
	// 	if [0, 4, 20, 84, 340, 1364, 5460, 21844, 87380, 349524].contains(&i)
	// 	// if i == 4*(4^n-1)/3:
	// 	{
	// 		depth += 1;
	// 	}
	// }

	// println!(
	// 	"X: {}, Y: {}, Counter: {:?}",
	// 	total_x / 1_048_575.,
	// 	total_y / 1_048_575.,
	// 	counter
	// );

	println!("{:#?}", a);
}
