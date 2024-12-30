#![allow(unused)] // x cinco minutos nomas

// Hey, this code sucks. It's an objective fact.
// I gave myself 24 hours to finish this so that I could actually finish it.
// IF this code actually gets used, I'll replace it with a working geometry library someday.
// Until then, enjoy this ad hoc, informally specified, bug ridden, slow implementation of half of a geometry notebook.

use core::mem::swap;
use core::ops::{Add, Div, Mul, Sub};
use std::cmp::Ordering;
use std::fmt::Display;

type Num = f64;
const EPS: Num = 1e-9;

#[derive(Debug, Clone, Copy, PartialOrd)]
pub struct Point(f64, f64);

impl Point {
    pub fn new(x: impl Into<Num>, y: impl Into<Num>) -> Self {
        Self(x.into(), y.into())
    }
    pub const fn x(self) -> Num {
        self.0
    }
    pub const fn y(self) -> Num {
        self.1
    }
    pub const fn tuple(self) -> (Num, Num) {
        (self.x(), self.y())
    }
    pub fn cross(a: Self, b: Self) -> Num {
        a.x().mul_add(b.y(), -b.x() * a.y())
    }
}

impl Add<Self> for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

impl Sub<Self> for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl Mul<Num> for Point {
    type Output = Self;
    fn mul(self, rhs: Num) -> Self::Output {
        Self::new(self.0 * rhs, self.1 * rhs)
    }
}
impl Mul<Point> for Num {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}
impl Div<Num> for Point {
    type Output = Self;
    fn div(self, rhs: Num) -> Self::Output {
        Self::new(self.0 / rhs, self.1 / rhs)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl PartialEq<Self> for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < EPS && (self.1 - other.1).abs() < EPS
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for Point {}

#[derive(Debug, Clone, Copy)]
pub struct Segment(Point, Point);

impl PartialEq<Segment> for Segment {
    fn eq(&self, other: &Segment) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
    fn ne(&self, other: &Segment) -> bool {
        !self.eq(other)
    }
}

impl Eq for Segment {}

impl Segment {
    pub const fn new(a: Point, b: Point) -> Self {
        Self(a, b)
    }
}

impl From<Segment> for (Point, Point) {
    fn from(value: Segment) -> Self {
        (value.0, value.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line(Segment);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Middle,
    Right,
}

impl Line {
    pub const fn new(seg: Segment) -> Self {
        Self(seg)
    }
    pub fn from_points(a: Point, b: Point) -> Self {
        Self(Segment::new(a, b))
    }
    pub fn direction(self, p: Point) -> Direction {
        let (a, b) = self.0.into();
        let cross = Point::cross((p - a), (p - b));
        if cross > EPS {
            Direction::Left
        } else if cross < -EPS {
            Direction::Right
        } else {
            Direction::Middle
        }
    }
    pub fn old_intersect(self, l: Self) -> Point {
        let (a, b) = l.into();
        let mut left = a;
        let mut right = b;
        match self.direction(left) {
            Direction::Left => {}
            Direction::Right => swap(&mut left, &mut right),
            Direction::Middle => return a,
        };
        println!("{:?} {:?}", self, l);
        match self.direction(right) {
            Direction::Left => panic!("Line points should cross the line to intersect"),
            Direction::Right => {}
            Direction::Middle => return b,
        }
        for _ in 0..100 {
            let mid = (left + right) / 2.into();
            match self.direction(mid) {
                Direction::Left => left = mid,
                Direction::Right => right = mid,
                Direction::Middle => return mid,
            }
        }
        panic!("Lines were parallel"); // this should be unreachable due to previous panic!
    }
    pub fn intersect(self, l: Self) -> Point {
        let (p1, q1) = self.into();
        let d1 = q1 - p1;
        let (p2, q2) = l.into();
        let d2 = q2 - p2;
        let alpha = Point::cross((p2 - p1), d2) / Point::cross(d1, d2);
        p1 + alpha * d1
    }
}

impl From<Line> for (Point, Point) {
    fn from(value: Line) -> Self {
        value.0.into()
    }
}

#[derive(Debug, Clone)]
pub struct Chull(Vec<Point>);

impl Chull {
    pub const fn new(points: Vec<Point>) -> Self {
        Self(points)
    }
    pub fn interesct(&self, line: Line) -> (Self, Self) {
        let n = self.0.len();
        let mut left: Vec<Point> = vec![];
        let mut right: Vec<Point> = vec![];
        for i in 0..n {
            let (a, b) = (self.0[i], self.0[(i + 1) % n]);
            let dir1 = line.direction(a);
            let dir2 = line.direction(b);
            match dir1 {
                Direction::Left => left.push(a),
                Direction::Right => right.push(a),
                Direction::Middle => {}
            }
            if dir1 != dir2 {
                let intersection = line.intersect(Line::from_points(a, b));
                left.push(intersection);
                right.push(intersection);
            }
        }
        (Self(left), Self(right))
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn print_for_desmos(&self) {
        let n = self.0.len();
        print!("polygon(");
        let mut b = false;
        for pnt in &self.0 {
            if b {
                print!(",")
            } else {
                b = true;
            }
            print!("{}", pnt);
        }
        println!(")");
    }

    pub fn is_adj(&self, other: &Self) -> bool {
        for i in 0..self.0.len() {
            let seg1 = Segment::new(self.0[i], self.0[(i + 1) % self.0.len()]);
            for j in 0..other.0.len() {
                let seg2 = Segment::new(other.0[j], other.0[(j + 1) % other.0.len()]);
                if seg1 == seg2 {
                    return true;
                }
            }
        }
        false
    }

    pub fn triangulate(&self) -> Vec<Triangle> {
        let n = self.0.len();
        if n < 3 {
            return vec![];
        }
        let point_zero = self.0[0];
        let mut ans = vec![];
        for idx in 2..n {
            ans.push(Triangle::new(point_zero, self.0[idx - 1], self.0[idx]));
        }
        ans
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Triangle(Point, Point, Point);

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        let mut arr = [a, b, c];
        arr.sort_by(Self::cmp);
        Self(arr[2], arr[1], arr[0])
    }
    fn cmp(a: &Point, b: &Point) -> Ordering {
        if a.y() > b.y() {
            return Ordering::Greater;
        }
        if a.y() == b.y() {
            return a.x().partial_cmp(&b.x()).unwrap();
        }
        Ordering::Less
    }
    pub const fn top(self) -> Point {
        self.0
    }
    pub const fn mid(self) -> Point {
        self.1
    }
    pub const fn bot(self) -> Point {
        self.2
    }
}
