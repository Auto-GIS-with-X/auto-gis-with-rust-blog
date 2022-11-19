use core::slice::Iter;
use std::convert::From;
use std::vec::IntoIter;

use num_traits::NumCast;

use crate::error::GeometryError;
use crate::geometry::Geometry;
use crate::helpers;
use crate::point::Point;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LineSegment([[f64; 2]; 2]);

impl LineSegment {
    /// A straight line connecting two points.
    ///
    /// # Examples:
    ///
    /// Construct a new `LineSegment` from a 2-element array of 2-element arrays.
    ///
    /// ```
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment_1 = LineSegment::new([[0., 0.], [1., 1.]]);
    /// let line_segment_2 = LineSegment::new([[0, 0], [1, 1]]);
    ///
    /// assert_eq!(line_segment_1, line_segment_2)
    /// ```
    pub fn new<T: NumCast>(coordinates: [[T; 2]; 2]) -> Self {
        let float_coordinates: [[f64; 2]; 2] = coordinates.map(|coordinate| {
            coordinate.map(|coordinate| -> f64 { num_traits::cast(coordinate).unwrap() })
        });
        LineSegment(float_coordinates)
    }

    pub fn source(&self) -> Point {
        Point::new(self.0[0][0], self.0[0][1])
    }

    pub fn target(&self) -> Point {
        Point::new(self.0[1][0], self.0[1][1])
    }
}

impl Geometry for LineSegment {
    /// Compute the geometric center of a geometry.
    ///
    /// For a `LineSegment`, this is the midpoint.
    ///
    /// ```
    /// use auto_gis_with_rust::{
    ///     geometry::Geometry,
    ///     line_string::LineSegment,
    ///     point::Point
    /// };
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [1., 1.]]);
    /// let expected_centroid = Point::new(0.5, 0.5);
    ///
    /// assert_eq!(line_segment.centroid(), expected_centroid);
    /// ```
    fn centroid(&self) -> Point {
        let x1 = self.source().x();
        let y1 = self.source().y();
        let x2 = self.target().x();
        let y2 = self.target().y();
        let x = (x1 + x2) / 2f64;
        let y = (y1 + y2) / 2f64;
        Point::new(x, y)
    }

    /// Return the WKT representation of a geometry.
    ///
    /// ```
    /// use auto_gis_with_rust::geometry::Geometry;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [1., 1.]]);
    /// let expected_wkt = String::from("LINESTRING (0 0, 1 1)");
    ///
    /// assert_eq!(line_segment.wkt(), expected_wkt);
    /// ```
    fn wkt(&self) -> String {
        format!(
            "LINESTRING ({} {}, {} {})",
            self.source().x(),
            self.source().y(),
            self.target().x(),
            self.target().y()
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LineSegments(Vec<LineSegment>);

impl IntoIterator for LineSegments {
    type Item = LineSegment;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<LineString> for LineSegments {
    fn from(line_string: LineString) -> Self {
        let line_segments: Vec<LineSegment> = line_string
            .iter()
            .as_slice()
            .windows(2)
            .map(|pair| LineSegment::new([pair[0], pair[1]]))
            .collect();

        LineSegments(line_segments)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LineString(Vec<[f64; 2]>);

impl LineString {
    /// Construct a new `LineString` from a vector of 2-element arrays.
    ///
    /// # Examples:
    ///
    /// Construct a new `LineString` from a vector of floats or a vector of integers.
    ///
    /// ```
    /// use auto_gis_with_rust::line_string::LineString;
    ///
    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 1.], [2., 2.]]).unwrap();
    /// let line_string_2 = LineString::new(vec![[0, 0], [1, 1], [2, 2]]).unwrap();
    ///
    /// assert_eq!(line_string_1, line_string_2)
    /// ```
    pub fn new<T: NumCast>(coordinates: Vec<[T; 2]>) -> Result<Self, GeometryError> {
        let number_of_coordinates = coordinates.len();
        if number_of_coordinates < 2 {
            Err(GeometryError::TooFewCoords(number_of_coordinates))
        } else {
            let float_coordinates = helpers::get_float_coordinates(coordinates);
            Ok(LineString(float_coordinates))
        }
    }

    pub fn iter(&self) -> Iter<[f64; 2]> {
        self.0.iter()
    }
}
