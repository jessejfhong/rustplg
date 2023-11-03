use std::cmp::PartialOrd;

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// by providing a concret tye f64 here, distance_from_origin is only implemented
// for Point<f64> instance
impl Point<f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub struct Rectangle<T, U> {
    width: T,
    length: U,
}

impl<T, U> Rectangle<T, U> {
    pub fn new(w: T, l: U) -> Self {
        Self {
            width: w,
            length: l,
        }
    }
}
