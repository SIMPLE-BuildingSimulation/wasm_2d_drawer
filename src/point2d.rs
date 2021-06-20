use wasm_bindgen::prelude::*;

/// Very simple structure: a 2D Point, with x and
/// y components
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

/// A point inside the Canvas
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct CanvasPoint2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// Calculates the squared distance to another point
    pub fn squared_distance_to(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

#[wasm_bindgen]
impl Point2D {
    /// Builds a new Point2D
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[wasm_bindgen]
impl CanvasPoint2D {
    /// Builds a new Point2D
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squared_distance() {
        let a = Point2D { x: 0., y: 0. };
        let b = Point2D { x: 0., y: 0. };
        assert_eq!(a.squared_distance_to(&b), 0.0);

        let b = Point2D { x: 0., y: 1. };
        assert_eq!(a.squared_distance_to(&b), 1.0);

        let b = Point2D { x: 0., y: 2. };
        assert_eq!(a.squared_distance_to(&b), 4.0);

        let b = Point2D { x: 1., y: 0. };
        assert_eq!(a.squared_distance_to(&b), 1.0);

        let b = Point2D { x: 2., y: 0. };
        assert_eq!(a.squared_distance_to(&b), 4.0);
    }
}
