use std::f64::consts::PI as pi;

pub trait Calculator {
    fn perimeter(&self) -> f64;
    fn area(&self) -> f64;
}

struct Rectangle {
    a: f64,
    b: f64,
}

struct Circle {
    r: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

#[allow(dead_code)]
enum Shape {
    Rec(Rectangle),
    Cir(Circle),
    Tri(Triangle),
}

impl Calculator for Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.perimeter(),
            Shape::Cir(c) => c.perimeter(),
            Shape::Tri(t) => t.perimeter(),
        }
    }

    fn area(&self) -> f64 {
        match self {
            Shape::Rec(r) => r.area(),
            Shape::Cir(c) => c.area(),
            Shape::Tri(t) => t.area(),
        }
    }
}

impl Calculator for Rectangle {
    fn perimeter(&self) -> f64 {
        2. * (self.a + self.b)
    }

    fn area(&self) -> f64 {
        self.a * self.b
    }
}

impl Calculator for Circle {
    fn perimeter(&self) -> f64 {
        2. * self.r * pi
    }

    fn area(&self) -> f64 {
        pi * self.r * self.r
    }
}

impl Calculator for Triangle {
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }

    fn area(&self) -> f64 {
        let p = self.perimeter() / 2.;
        (p * (p - self.a) * (p - self.b) * (p - self.c)).sqrt()
    }
}

fn main() {
    let r = Rectangle {
        a: 2., 
        b: 3.,
    };
    let shape1 = Shape::Rec(r);
    println!("Shape1: {}, {}", shape1.perimeter(), shape1.area());

    let c = Circle {
        r: 2.5, 
    };
    let shape2 = Shape::Cir(c);
    println!("Shape2: {}, {}", shape2.perimeter(), shape2.area());

    let t = Triangle {
        a: 3., 
        b: 4.,
        c: 5.,
    };
    let shape3 = Shape::Tri(t);
    println!("Shape3: {}, {}", shape3.perimeter(), shape3.area());
}