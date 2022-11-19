use itertools::Itertools;
use std::slice::Iter;

use crate::geometry::Geometry;
use crate::helpers;
use num_traits::{self, NumCast};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Point([f64; 2]);

impl Point {
    /// Construct a new `Point`.
    ///
    /// # Examples:
    ///
    /// Construct a new point from x and y floats or x and y integers.
    ///
    /// ```
    /// use auto_gis_with_rust::point::Point;
    ///
    /// let point_0 = Point::new(0.0, 1.0);
    /// let point_1 = Point::new(0, 1);
    ///
    /// assert_eq!(point_0, point_1);
    /// ```
    pub fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
        let x_float: f64 = num_traits::cast(x).unwrap();
        let y_float: f64 = num_traits::cast(y).unwrap();
        Point([x_float, y_float])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }
}

impl Geometry for Point {
    /// Compute the geometric center of a geometry.
    ///
    /// For a `Point`, this is a clone of the `Point` itself.
    ///
    /// ```
    /// use auto_gis_with_rust::geometry::Geometry;
    /// use auto_gis_with_rust::point::Point;
    ///
    /// let point = Point::new(0.0, 1.0);
    ///
    /// assert_eq!(point.centroid(), point);
    /// ```
    fn centroid(&self) -> Point {
        self.clone()
    }

    /// Return the WKT representation of a geometry.
    ///
    /// For a `Point`, this is a clone of the `Point` itself.
    ///
    /// ```
    /// use auto_gis_with_rust::geometry::Geometry;
    /// use auto_gis_with_rust::point::Point;
    ///
    /// let point = Point::new(0.0, 1.0);
    /// let expected_wkt = String::from("POINT (0.0 1.0")
    ///
    /// assert_eq!(point.wkt(), expected_wkt);
    /// ```
    fn wkt(&self) -> String {
        format!("POINT ({} {})", self.x(), self.y())
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct MultiPoint(Vec<Point>);

impl MultiPoint {
    pub fn new<T: NumCast>(points: Vec<[T; 2]>) -> Self {
        let float_coordinates = helpers::get_float_coordinates(coordinates);
        let points: Vec<Point> = float_coordinates
            .iter()
            .map(|coordinate| Point::new(coordinate[0], coordinate[1]))
            .collect();
        MultiPoint(points)
    }

    pub fn iter(&self) -> Iter<Point> {
        self.0.iter()
    }
}

impl Geometry for MultiPoint {
    fn centroid(&self) -> Point {
        let xs: Vec<f64> = self.iter().map(Point::x).collect();
        let mean_x: f64 = xs.iter().sum::<f64>() / xs.iter().len() as f64;

        let ys: Vec<f64> = self.iter().map(Point::y).collect();
        let mean_y: f64 = ys.iter().sum::<f64>() / ys.iter().len() as f64;

        Point::new(mean_x, mean_y)
    }

    /// Return the WKT representation of a geometry.
    ///
    /// ```
    /// use auto_gis_with_rust::geometry::Geometry;
    /// use auto_gis_with_rust::point::MultiPoint;
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [1., 1.]]);
    /// let expected_wkt = String::from("LINESTRING (0 0, 1 1)");
    ///
    /// assert_eq!(line_segment.wkt(), expected_wkt);
    /// ```
    fn wkt(&self) -> String {
        let points = self.iter().format_with(", ", |point, f| {
            f(&format_args!("{} {}", point.x(), point.y()))
        });
        format!("MULTIPOINT ({})", points)
    }
}
