#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn new() -> Self {
        Polyline { points: Vec::new() }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    fn remove_point(&mut self, index: usize) {
        self.points.remove(index);
    }

    fn get_point(&self, index: usize) -> Option<&Point> {
        self.points.get(index)
    }

    fn points(&self) -> &Vec<Point> {
        &self.points
    }

    fn points_mut(&mut self) -> &mut Vec<Point> {
        &mut self.points
    }
}

fn main() {
    let a = Point { x: 1.0, y: 2.0 };
    println!("Point: {:?}", a);
    let b = a.clone();

    let mut polyline = Polyline::new();
    polyline.add_point(a);
    polyline.add_point(b);

    polyline.add_point(Point { x: 10.0, y: 11.0 });

    polyline.remove_point(0);
    println!("Polyline point: {:?}", polyline.get_point(0).unwrap());
    println!("Polyline points: {:?}", polyline.points());
    polyline.points_mut().pop();
    println!("Polyline points: {:?}", polyline.points());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_copy_and_default() {
        let p1 = Point::default();
        let p2 = p1; // Should compile, as Point must implement Copy.
        assert_eq!(p1.x, 0.0);
        assert_eq!(p1.y, 0.0);
        assert_eq!(p2.x, 0.0);
        assert_eq!(p2.y, 0.0);
    }

    #[test]
    fn polyline_is_cloneable() {
        let mut polyline = Polyline::new();
        polyline.add_point(Point { x: 1.0, y: 2.0 });
        polyline.add_point(Point { x: 3.0, y: 4.0 });

        let cloned = polyline.clone();
        assert_eq!(cloned.get_point(0).unwrap().x, 1.0);
        assert_eq!(cloned.get_point(0).unwrap().y, 2.0);
        assert_eq!(cloned.get_point(1).unwrap().x, 3.0);
        assert_eq!(cloned.get_point(1).unwrap().y, 4.0);
    }

    #[test]
    fn polyline_add_and_remove_points() {
        let mut polyline = Polyline::new();

        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = Point { x: 3.0, y: 4.0 };

        polyline.add_point(p1);
        polyline.add_point(p2);

        assert_eq!(polyline.get_point(0).unwrap().x, 1.0);
        assert_eq!(polyline.get_point(1).unwrap().x, 3.0);

        polyline.remove_point(0);

        assert_eq!(polyline.get_point(0).unwrap().x, 3.0);
        assert!(polyline.get_point(1).is_none());
    }

    #[test]
    fn polyline_cannot_be_empty() {
        let polyline = Polyline::new();
        assert!(polyline.get_point(0).is_none());
    }
}
