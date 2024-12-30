#![allow(unused)]

mod geometry;
mod unset;

use std::mem::swap;

use geometry::*;
use image::{ImageBuffer, Rgb};
use rand::seq::SliceRandom;
//use unset::Unset;

const WIDTH: u32 = 2000;
const HEIGHT: u32 = 1000;
const NUM_COLOURS: usize = 3;
const COLOURS: [Rgb<u8>; NUM_COLOURS] = [
    Rgb([0xf5, 0xa9, 0xb8]),
    Rgb([0x5b, 0xce, 0xfa]),
    Rgb([255, 255, 255]),
];

fn main() {
    let chulls = draw_lines();
    let guide = colour(chulls.clone());
    println!("{:?}", guide);
    random_tour(chulls, guide);
}

fn random_tour(chull_set: Vec<Chull>, guide: Vec<usize>) {
    let colour = Rgb([255u8, 0, 0]);
    let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    let y = 50;
    let (xbeg, xend) = (10, 160);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = colour;
    }

    let mut rng = rand::thread_rng();
    for (idx, chull) in chull_set.iter().enumerate() {
        let colour = COLOURS[(guide[idx] - 1) % NUM_COLOURS];
        for triangle in chull.triangulate() {
            draw_triangle(&mut img, triangle, colour);
        }
    }

    img.save("out.png").unwrap();
}

fn draw_triangle(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, tri: Triangle, colour: Rgb<u8>) {
    let top = tri.top();
    let mid = tri.mid();
    let bot = tri.bot();
    let topmid = Line::from_points(top, mid);
    let midbot = Line::from_points(mid, bot);
    let bottop = Line::from_points(bot, top);
    let topy = top.y() as u32;
    let midy = mid.y() as u32;
    let boty = bot.y() as u32;
    for y in midy..=topy {
        let horline = Line::from_points(Point::new(0, y), Point::new(WIDTH, y));
        let inter1 = horline.intersect(topmid);
        let inter2 = horline.intersect(bottop);
        draw_horizontal_line(img, y, inter1.x() as u32, inter2.x() as u32, colour);
    }
    for y in boty..=midy {
        let horline = Line::from_points(Point::new(0, y), Point::new(WIDTH, y));
        let inter1 = horline.intersect(midbot);
        let inter2 = horline.intersect(bottop);
        draw_horizontal_line(img, y, inter1.x() as u32, inter2.x() as u32, colour);
    }
}

fn draw_horizontal_line(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    y: u32,
    xbeg: u32,
    xend: u32,
    colour: Rgb<u8>,
) {
    if y >= HEIGHT {
        return;
    }
    let (a, b) = if xbeg < xend {
        (xbeg, xend)
    } else {
        (xend, xbeg)
    };
    let a = std::cmp::max(a, 0);
    let b = std::cmp::min(b, WIDTH - 1);
    for x in a..=b {
        img[(x, y)] = colour;
    }
}

fn colour(chull_set: Vec<Chull>) -> Vec<usize> {
    let mut graph = Graph::new(chull_set);
    //println!("{:?}", graph.adjacency_matrix);
    let colour = graph.colour();
    colour
}

fn draw_lines() -> Vec<Chull> {
    let rect = Chull::new(vec![
        Point::new(0, 0),
        Point::new(0, HEIGHT),
        Point::new(WIDTH, HEIGHT),
        Point::new(WIDTH, 0),
    ]);
    let mut chull_set = Vec::new();
    chull_set.push(rect);
    for _ in 0..20 {
        let mut temp_chull_set = vec![];
        let a = random_point();
        let b = random_point();
        let random_line = Line::from_points(a, b);
        for chull in &chull_set {
            let (left, right) = chull.interesct(random_line);
            if !left.is_empty() {
                temp_chull_set.push(left);
            }
            if !right.is_empty() {
                temp_chull_set.push(right);
            }
        }
        swap(&mut temp_chull_set, &mut chull_set);
    }
    /*for chull in &chull_set {
        chull.print_for_desmos();
    }*/
    chull_set
}

fn random_point() -> Point {
    let a: f64 = rand::random();
    let b: f64 = rand::random();
    Point::new(a * (WIDTH as f64), b * (HEIGHT as f64))
}

#[derive(Debug)]
struct Graph {
    adjacency_matrix: Vec<Vec<usize>>,
    chulls: Vec<Chull>,
    colours: Vec<usize>,
}

impl Graph {
    pub fn new(chullset: Vec<Chull>) -> Self {
        let mut adj: Vec<Vec<usize>> = vec![];
        for (idx, chull) in chullset.iter().enumerate() {
            adj.push(vec![]);
            for (idx1, other) in chullset.iter().enumerate() {
                if idx == idx1 {
                    continue;
                }
                if chull.is_adj(other) {
                    adj[idx].push(idx1);
                }
            }
        }
        let n = &chullset.len();
        Self {
            adjacency_matrix: adj,
            chulls: chullset,
            colours: vec![0; *n],
        }
    }
    pub fn colour(&mut self) -> Vec<usize> {
        assert!(self.backtrack(0, &mut rand::thread_rng()));
        return self.colours.clone();
    }

    fn backtrack(&mut self, curr: usize, rng: &mut rand::rngs::ThreadRng) -> bool {
        if curr == self.colours.len() {
            return true;
        }
        let mut order = (1..=NUM_COLOURS).collect::<Vec<_>>();
        order.shuffle(rng);
        for colour in 1..=NUM_COLOURS {
            let mut flag = true;
            for nei in &self.adjacency_matrix[curr] {
                if self.colours[*nei] == colour {
                    flag = false;
                    break;
                }
            }
            if flag {
                self.colours[curr] = colour;
                if self.backtrack(curr + 1, rng) {
                    return true;
                }
                self.colours[curr] = 0;
            }
        }
        false
    }
}
