#[derive(Debug)]
enum Container {
    Tree([Box<Quadtree>; 4]),
    Leaf(Vec<TreeItem>),
}

impl Default for Container {
    fn default() -> Self {
        Self::Leaf(Vec::new())
    }
}

#[derive(Debug)]
struct Quadtree {
    tree: Container,
    depth: i32,
    capacity: i32,
}

impl Default for Quadtree {
    fn default() -> Self {
        Self {
            tree: Default::default(),
            depth: 4,
            capacity: 8,
        }
    }
}

struct Pos(f32, f32);

enum TreeItem {
    Integer(i32, Pos),
    Float(f32, Pos),
    String(String, Pos),
    Bool(bool, Pos),
}

impl Quadtree {
    fn new(depth: i32, capacity: i32) -> Self {
        Self {
            depth,
            capacity,
            ..Default::default()
        }
    }
    fn insert(&mut self, item: TreeItem) {
        match self.tree {
            Container::Leaf(ref mut vec) => vec.push(item),
            Container::Tree(ref mut vec) => for quad in vec {},
        }
    }
}

fn main() {
    let mut a = Quadtree::default();
    a.insert(TreeItem::new());

    println!("{:?}", "a");
}
