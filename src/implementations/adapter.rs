trait Rectangle {
    fn draw(&self);
}

#[derive(Debug)]
struct LegacyRectangle {
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize,
}

impl LegacyRectangle {
    fn new(x1: isize, y1: isize, x2: isize, y2: isize) -> Self {
        println!("creating LegacyRectangle");
        LegacyRectangle { x1, y1, x2, y2 }
    }
}

impl Rectangle for LegacyRectangle {
    fn draw(&self) {
        println!(
            "Draw from LegacyRectangle ({},{}) to ({},{})",
            self.x1, self.y1, self.x2, self.y2
        );
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct RectangleAdapter {
    rectangle: LegacyRectangle,
}

impl RectangleAdapter {
    fn new(top_left: &Point, bottom_right: &Point) -> RectangleAdapter {
        println!("creating RectangleAdapter");
        RectangleAdapter {
            rectangle: LegacyRectangle::new(top_left.x, top_left.y, bottom_right.x, bottom_right.y),
        }
    }
}

impl Rectangle for RectangleAdapter {
    fn draw(&self) {
        println!("draw from RectangleAdapter: {:?}", self);
        self.rectangle.draw()
    }
}

pub fn run_adapter_logic() {
    let top_left = Point { x: 10, y: 20 };
    let bottom_right = Point { x: 30, y: 40 };

    let rectangle = RectangleAdapter::new(&top_left, &bottom_right);
    rectangle.draw();
}
