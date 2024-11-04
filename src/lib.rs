use core::cmp::{max, min};
use core::ops::{Add, AddAssign, Sub};

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Default)]
pub struct Path {
    path: Vec<Point>,
}

pub fn point(x: i64, y: i64) -> Point {
    Point { x, y }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign<Point> for Path {
    fn add_assign(&mut self, point: Point) {
        self.path.push(point);
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn get_area(p: &Path) -> f64 {
    let mut area = 0f64;
    let cnt = p.path.len();
    if cnt < 3 {
        return area;
    }

    // Trapezoid formula
    for i in 0..p.path.len() - 1 {
        area += ((p.path[i + 1].y + p.path[i].y) * (p.path[i].x - p.path[i + 1].x)) as f64;
    }
    area += ((p.path[cnt - 1].y + p.path[0].y) * (p.path[cnt - 1].x - p.path[0].x)) as f64;
    area * 0.5
}

pub fn add_point_to_path(p: Point, handle: &mut Path) {
    *handle += p;
}

pub fn is_point_in_polygon(q: Point, p: &Path) -> bool {
    if p.path.len() < 3 {
        return false;
    }
    let mut min_x = p.path[0].x;
    let mut max_x = p.path[0].x;
    let mut min_y = p.path[0].y;
    let mut max_y = p.path[0].y;
    for i in 1..p.path.len() {
        let c = p.path[i].clone();
        min_x = min(c.x, min_x);
        max_x = max(c.x, max_x);
        min_y = min(c.y, min_y);
        max_y = max(c.y, max_y);
    }
    if q.x < min_x || q.x > max_x || q.y < min_y || q.y > max_y {
        return false;
    }

    let mut inside = false;
    let mut i = 0;
    let mut j = p.path.len() - 1;
    while i < p.path.len() {
        if (p.path[i].y > q.y) != (p.path[j].y > q.y)
            && q.x
                < (p.path[j].x - p.path[i].x) * (q.y - p.path[i].y) / (p.path[j].y - p.path[i].y)
                    + p.path[i].x
        {
            inside = !inside;
        }
        j = i;
        i += 1;
    }

    inside
}

#[test]
fn test_point_add() {
    let p1 = Point { x: 1, y: 3 };
    let p2 = Point { x: 2, y: -3 };
    let ps = p1 + p2;
    assert_eq!(ps.x, 3);
    assert_eq!(ps.y, 0);
}
#[test]
fn test_point_sub() {
    let p1 = Point { x: 1, y: 3 };
    let p2 = Point { x: 2, y: -3 };
    let ps = p1 - p2;
    assert_eq!(ps.x, -1);
    assert_eq!(ps.y, 6);
}
#[test]
fn test_polygon_get_area() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = Point { x: 2, y: 3 };
    let p4 = Point { x: 1, y: 3 };
    let mut path = Path::default();
    path += p1;
    assert_eq!(0f64, get_area(&path));
    path += p2;
    assert_eq!(0f64, get_area(&path));
    path += p3;
    assert_eq!(0.5f64, get_area(&path));
    path += p4;
    assert_eq!(1.0f64, get_area(&path));
}
#[test]
fn test_get_area_positive() {
    let p: Path = Path {
        path: vec![
            Point { x: 0, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 0, y: 2 },
        ],
    };
    assert_eq!(2f64, get_area(&p));
}
#[test]
fn test_get_area_negative() {
    let p: Path = Path {
        path: vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 2 },
            Point { x: 2, y: 0 },
        ],
    };
    assert_eq!(-2f64, get_area(&p));
}
#[test]
fn test_point_in_polygon() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 2 };
    let p3 = Point { x: 2, y: 3 };
    let p4 = Point { x: 1, y: 3 };
    let p5 = Point { x: -2, y: 4 };
    let p6 = Point { x: 0, y: -2 };
    let mut path = Path::default();
    path += p1;
    let s = Point { x: 0, y: 2 };
    assert_eq!(false, is_point_in_polygon(s.clone(), &path));
    path += p2;
    assert_eq!(false, is_point_in_polygon(s.clone(), &path));
    path += p3;
    assert_eq!(false, is_point_in_polygon(s.clone(), &path));
    path += p4;
    assert_eq!(false, is_point_in_polygon(s.clone(), &path));
    path += p5;
    assert_eq!(false, is_point_in_polygon(s.clone(), &path));
    path += p6;
    assert_eq!(true, is_point_in_polygon(s.clone(), &path));
}
