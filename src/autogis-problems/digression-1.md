---
title: "Digression 1: The Other GeoArrow Types"
layout: 'home.html'
---

To complete [Problem 1](/autogis-problems/problem-1.md) we had to define types to represent points, line strings, and polygons but the [GeoArrow spec](https://github.com/geoarrow/geoarrow/blob/main/format.md) also collections of those geometries in the form of `MultiPoint`, `MultiLineString`, and `MultiPolygon`. Let's define those too!

## MultiPoint

The GeoArrow definition is:

> An array of MultiPoints is represented as a nested list array, where each outer list is a single MultiPoint (i.e. a list of xy coordinates). The child name of the outer List should be "points".

Okay, so let's add a definition of a `MultiPoint` type to the bottom of our `point.rs` file.

```rust
// point.rs

...
#[derive(Debug, PartialEq, PartialOrd)]
pub struct MultiPoint(Vec<Point>);

impl MultiPoint {
    /// Construct a new `MultiPoint`.
    ///
    /// # Examples:
    ///
    /// Construct a new multi-point vector of `Point`s.
    ///
    /// ```
    /// use auto_gis_with_rust::point::{Point, MultiPoint};
    ///
    /// let point_0 = Point::new(0.0, 0.0);
    /// let point_1 = Point::new(1.0, 0.0);
    /// let multi_point_0 = MultiPoint(vec![point_0, point_1]);

    /// let point_2 = Point::new(0, 0);
    /// let point_3 = Point::new(1, 0);
    /// let multi_point_1 = MultiPoint(vec![point_2, point_3]);
    ///
    /// assert_eq!(multi_point_0, multi_point_1);
    /// ```
    pub fn new(points: Vec<Point>) -> Self {
        MultiPoint(points)
    }
}
```

Looks good but it doesn't compile. 

```sh
error[E0423]: cannot initialize a tuple struct which contains private fields
  --> src/point.rs:43:21
   |
8  | let multi_point_0 = MultiPoint(vec![point_0, point_1]);
   |                     ^^^^^^^^^^
   |
note: constructor is not visible here due to private fields
  --> /home/ed/repos/auto-gis-with-rust/src/point.rs:29:23
   |
29 | pub struct MultiPoint(Vec<Point>);
   |                       ^^^^^^^^^^ private field
```

Ah, okay, we need to add `pub` before `Vec<Point>` in the `struct` signature.

```diff-rust
// point.rs

...
#[derive(Debug, PartialEq, PartialOrd)]
-pub struct MultiPoint(Vec<Point>);
+pub struct MultiPoint(pub Vec<Point>);

impl MultiPoint {
...
```

Now it works but two things are bugging me:

1. Comparing a `Point` / `MultiPoint` built from floats to one built integers doesn't feel great. Wouldn't it be easier if we had a nice way of representing them that we could also use to assert their correctness?
1. Wouldn't it be better if we build a `Point` directly from a 2-element array and a `MultiPoint` from a vector of 2-element arrays?

I think [Well-Known Text (WKT)](https://portal.ogc.org/files/?artifact_id=25355) could be a nice solution to the first problem, so let's try that.

### WKT for `Point`

Essentially, what we're doing here is converting a `Point` to a string, so let's google that. Right, the [Rust docs](https://doc.rust-lang.org/std/string/trait.ToString.html) say we should implement `Display` instead:

> This trait is automatically implemented for any type which implements the Display trait. As such, ToString shouldn’t be implemented directly: Display should be implemented instead, and you get the ToString implementation for free.

```diff-rust
// point.rs

+use std::fmt;
+
use num_traits::{self, NumCast};
...
    /// let point_1 = Point::new(0, 1);
    ///
+    /// assert_eq!("POINT (0 1)", point_0.to_string());
+    ///
    /// assert_eq!(point_0, point_1);
...
}

+impl fmt::Display for Point {
+    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
+        write!(f, "POINT ({} {})", self.0[0], self.0[1])
+    }
+}
+
#[derive(Debug, PartialEq, PartialOrd)]
...
```

Cool, now we have a nice way of representing our points that will make it easier to write Doctests. Let's do the same for `MultiPoint`.

### WKT for `MultiPoint`

The problem with `MultiPoint`, when it comes to formatting as WKT, is that it can contain an arbitrary number of `Point`. We need something similar to `", ".join()` in Python. `itertools` has a `format_with` function that looks just the ticket but, for that to work, we need `MultiPoint` to be a `Iterator`.

This feels like cheating but it looks like we can simply expose the underlying `Vec`'s `.iter` method:

```diff-rust
// point.rs

-use std::fmt;
+use std::{fmt, slice::Iter};
...
    pub fn new(points: Vec<Point>) -> Self {
        MultiPoint(points)
    }
+
+    /// Returns an iterator of `Points`
+    ///
+    /// # Example
+    /// ```
+    /// use auto_gis_with_rust::point::{Point, MultiPoint};
+    ///
+    /// let point_0 = Point::new(0.0, 0.0);
+    /// let point_1 = Point::new(1, 0);
+    /// let multi_point = MultiPoint(vec![point_0, point_1]);
+    ///
+    /// assert_eq!("POINT (0 0)", multi_point.iter().next().unwrap().to_string())
+    /// ```
+    pub fn iter(&self) -> Iter<Point> {
+        self.0.iter()
+    }
}
```

We can then use call `format_with` on this `Iterator`:

```diff-rust
// point.rs

use std::{fmt, slice::Iter};

+use itertools::Itertools;
use num_traits::{self, NumCast};
...
    pub fn iter(&self) -> Iter<Point> {
        self.0.iter()
    }
}
+
+impl fmt::Display for MultiPoint {
+    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
+        let points = self.iter().format_with(", ", |point, f| {
+            f(&format_args!("{} {}", point.0[0], point.0[1]))
+        });
+        write!(f, "MULTIPOINT ({})", points)
+    }
+}
```

This allows us to simplify our docstrings:

```diff-rust
// point.rs

...
impl MultiPoint {
    /// Construct a new `MultiPoint`.
    ///
    /// # Examples:
    ///
    /// Construct a new multi-point vector of `Point`s.
    ///
    /// ```
    /// use auto_gis_with_rust::point::{Point, MultiPoint};
    ///
    /// let point_0 = Point::new(0.0, 0.0);
-    /// let point_1 = Point::new(1.0, 0.0);
-    /// let multi_point_0 = MultiPoint(vec![point_0, point_1]);
-
-    /// let point_2 = Point::new(0, 0);
-    /// let point_3 = Point::new(1, 0);
-    /// let multi_point_1 = MultiPoint(vec![point_2, point_3]);
-    ///
-    /// assert_eq!(multi_point_0, multi_point_1);
+    /// let point_1 = Point::new(1, 0);
+    /// let multi_point = MultiPoint(vec![point_0, point_1]);
+    ///
+    /// assert_eq!("MULTIPOINT (0 0, 1 0)", multi_point.to_string());
...
```

### From 2-element array for `Point`

A two-element array of any type that implements `NumCast` and `Copy` can be converted into a `Point`, so we just need to implement the `From` trait. 

```diff-rust
// point.rs

...
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "POINT ({} {})", self.0[0], self.0[1])
    }
}

+impl<T: NumCast + Copy> From<[T; 2]> for Point {
+    /// Construct a `Point` from a 2-element array.
+    ///
+    /// # Examples:
+    ///
+    /// ```
+    /// use auto_gis_with_rust::point::Point;
+    ///
+    /// let point = Point::from([0.0, 1.0]);
+    ///
+    /// assert_eq!("POINT (0 1)", point.to_string());
+    /// ```
+    fn from(coordinates: [T; 2]) -> Self {
+        Point::new(coordinates[0], coordinates[1])
+    }
+}
+
#[derive(Debug, PartialEq, PartialOrd)]
pub struct MultiPoint(pub Vec<Point>);
...
```

This doesn't appear to get us much but it makes the next bit a lot easier.

### From a vector of 2-element arrays for `MultiPoint`

```diff-rust
// point.rs

...
impl fmt::Display for MultiPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let points = self.iter().format_with(", ", |point, f| {
            f(&format_args!("({} {})", point.0[0], point.0[1]))
        });
        write!(f, "MULTIPOINT ({})", points)
    }
}
+
+impl<T: NumCast + Copy> From<Vec<[T; 2]>> for MultiPoint {
+    /// Construct a `MultiPoint` from a vector of 2-element arrays.
+    ///
+    /// # Examples:
+    ///
+    /// ```
+    /// use auto_gis_with_rust::point::MultiPoint;
+    ///
+    /// let multi_point = MultiPoint::from(vec![[0.0, 0.0], [1.0, 0.0]]);
+    ///
+    /// assert_eq!("MULTIPOINT ((0 0), (1 0))", multi_point.to_string());
+    /// ```
+    fn from(items: Vec<[T; 2]>) -> Self {
+        let points: Vec<Point> = items.into_iter().map(Point::from).collect();
+        MultiPoint::new(points)
+    }
+}
```

Okay, great, now we have a `MultiPoint`type that can be constructed from a vector of `Point`s or a vector of 2-element arrays and can be converted to WKT. 

## MultiLineString

### WKT for `LineString`

`MultiLineString` next but, before we do that, let's implement WKT for `LineString`.

```diff-rust
+use std::{fmt, slice::Iter};
+
+use itertools::Itertools;
use num_traits::NumCast;
...
impl LineString {
    /// Construct a new `LineString` from a vector of 2-element arrays.
    ///
    /// # Examples:
+    ///
+    /// ```
+    /// use auto_gis_with_rust::line_string::LineString;
+    ///
+    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 0.], [1., 1.]]).unwrap();
+    ///
+    /// assert_eq!("LINESTRING (0 0, 1 0, 1 1)", line_string_1.to_string());
+    /// ```
    ///
    /// Construct a new `LineString` from a vector of floats or a vector of integers.
    ///
    /// ```
    /// use auto_gis_with_rust::line_string::LineString;
    ///
-    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 1.]]);
-    /// let line_string_2 = LineString::new(vec![[0, 0], [1, 1]]);
+    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 0.], [1., 1.]]).unwrap();
+    /// let line_string_2 = LineString::new(vec![[0, 0], [1, 0], [1, 1]]).unwrap();
    ///
    /// assert_eq!(line_string_1, line_string_2);
    /// ```
    pub fn new<T: NumCast>(coordinates: Vec<[T; 2]>) -> Result<Self, GeometryError> {
        ...
    }
+
+    /// Returns an iterator of 2 64-bit float arrays.
+    pub fn iter(&self) -> Iter<[f64; 2]> {
+        self.0.iter()
+    }
}

+impl fmt::Display for LineString {
+    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
+        let points = self.iter().format_with(", ", |point, f| {
+            f(&format_args!("{} {}", point[0], point[1]))
+        });
+        write!(f, "LINESTRING ({})", points)
+    }
+}
```

I'm also adding an extra segment to each of the `LineString`s in the example for reasons that I will get into later.

### `new` and WKT for `MultiLineString`

Okay, now that's done, let's get on with it. The GeoArrow spec for MultiLineString is as follows:

>An array of MultiLineStrings is represented as a nested list array with two levels of outer nesting: each element of the array (MultiLineString) is a list of LineStrings, which consist itself of a list xy vertices (see above). The child name of the outer list should be "linestrings"; the child name of the inner list should be "vertices".

To do this, we'll need a `MultiLineString` `struct` with a `new` constructor. I want to use WKT in the doctest, so we'll also need to implement `Display` and to display a `MultiLineString` composed of an arbitrary number of `LineStrings`, we'll need to be able to convert `MultiLineString` to an iterator we can call `itertools`' `.format_with` on. 

```diff-rust
// line_string.rs

...
impl fmt::Display for LineString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let points = self.iter().format_with(", ", |point, f| {
            f(&format_args!("{} {}", point[0], point[1]))
        });
        write!(f, "LINESTRING ({})", points)
    }
}
+
+#[derive(Debug, PartialEq, PartialOrd)]
+pub struct MultiLineString(Vec<LineString>);
+
+impl MultiLineString {
+    /// Construct a new `MultiLineString` from a vector of 'LineString's.
+    ///
+    /// # Examples:
+    ///
+    /// ```
+    /// use auto_gis_with_rust::line_string::{LineString, MultiLineString};
+    ///
+    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 0.], [1., 1.]]).unwrap();
+    /// let line_string_2 = LineString::new(vec![[1., 2.], [0., 2.], [0., 1.]]).unwrap();
+    ///
+    /// let multi_line_string = MultiLineString::new(vec![line_string_1, line_string_2]);
+    ///
+    /// assert_eq!("MULTILINESTRING ((0 0, 1 0, 1 1), (1 2, 0 2, 0 1))", multi_line_string.to_string());
+    /// ```
+    pub fn new(linestrings: Vec<LineString>) -> Self {
+        MultiLineString(linestrings)
+    }
+
+    /// Returns an iterator of `LineStrings`
+    ///
+    /// # Examples
+    ///
+    /// ```
+    /// use auto_gis_with_rust::line_string::{LineString, MultiLineString};
+    ///
+    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 0.], [1., 1.]]).unwrap();
+    /// let line_string_2 = LineString::new(vec![[1., 2.], [0., 2.], [0., 1.]]).unwrap();
+    ///
+    /// let multi_line_string = MultiLineString::new(vec![line_string_1, line_string_2]);
+    ///
+    /// assert_eq!("LINESTRING (0 0, 1 0, 1 1)", multi_line_string.iter().next().unwrap().to_string())
+    /// ```
+    pub fn iter(&self) -> Iter<LineString> {
+        self.0.iter()
+    }
+}
+
+impl fmt::Display for MultiLineString {
+    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
+        let line_strings = self
+            .iter()
+            .map(|line_string| {
+                line_string.iter().format_with(", ", |point, f| {
+                    f(&format_args!("{} {}", point[0], point[1]))
+                })
+            })
+            .format_with(", ", |line_string, f| f(&format_args!("({})", line_string)));
+        write!(f, "MULTILINESTRING ({})", line_strings)
+    }
+}
```

### From a vector of vectors of 2-element arrays for `MultiLineString`

It would also be useful if we could construct a `MultiLineString` from a vector of vectors of 2-element floats instead of needing a vector of `LineString`, so let's implement a converter. However, as `LineString::new` will fail if the inner vectors have less than 2 2-float arrays, this will need to be a `TryFrom`.

```diff-rust
// line_string.rs

+use std::convert::TryFrom;
use std::{fmt, slice::Iter};
...
impl fmt::Display for MultiLineString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_strings = self
            .iter()
            .map(|line_string| {
                line_string.iter().format_with(", ", |point, f| {
                    f(&format_args!("{} {}", point[0], point[1]))
                })
            })
            .format_with(", ", |line_string, f| f(&format_args!("({})", line_string)));
        write!(f, "MULTILINESTRING ({})", line_strings)
    }
}
+
+impl<T: NumCast> TryFrom<Vec<Vec<[T; 2]>>> for MultiLineString {
+    type Error = GeometryError;
+
+    /// Tries to convert a vector of vectors of 2-float arrays into a `MultiLineString`.
+    ///
+    /// # Examples:
+    ///
+    /// ```
+    /// use std::convert::TryFrom;
+    /// use auto_gis_with_rust::line_string::{LineString, MultiLineString};
+    ///
+    /// let multi_line_string = MultiLineString::try_from(vec![
+    ///    vec![[0., 0.], [1., 0.], [1., 1.]],
+    ///    vec![[1., 2.], [0., 2.], [0., 1.]],
+    /// ]).unwrap();
+    ///
+    /// assert_eq!("MULTILINESTRING ((0 0, 1 0, 1 1), (1 2, 0 2, 0 1))", multi_line_string.to_string());
+    /// ```
+    fn try_from(vectors: Vec<Vec<[T; 2]>>) -> Result<Self, GeometryError> {
+        let line_strings: Result<Vec<LineString>, GeometryError> =
+            vectors.into_iter().map(LineString::new).collect();
+        Ok(MultiLineString::new(line_strings?))
+    }
+}
```

Fantastic, now we have a `MultiLineString`type that can be constructed from a vector of `LineString`s or a vector of vectors of 2-element arrays and can be converted to WKT. 

## MultiPolygon

You should know the drill by now, first we're going to implement WKT for `Polygon`, then define a `MultiPolygon` type with a `new` constructor and a WKT `Display`method, and finally ...

### WKT for `Polygon`

The example given by the [Open Geospatial Consortium's *OpenGIS® Implementation Standard for Geographic information - Simple feature access - Part 1: Common architecture*](http://www.opengis.net/doc/is/sfa/1.2.1) is:

> Polygon ((10 10, 10 20, 20 20, 20 15, 10 10))

Which represents:

> a Polygon with 1 exteriorRing and 0 interiorRings

This is essentially the same as the `MultiLineString` representation so, with luck, we can reuse the same pattern.

With that in mind, we'll need to do it implement WKT for `PolygonRing`, which means that the first thing we need to do it implement an `.iter`method for `PolygonRing`.

```diff-rust
// polygon.rs
use num_traits::NumCast;
+use std::slice::Iter;

use crate::error::GeometryError;
...
    pub fn new<T: NumCast>(coordinates: Vec<[T; 2]>) -> Result<Self, GeometryError> {
        let number_of_coordinates = coordinates.len();
        if number_of_coordinates < 3 {
            Err(GeometryError::TooFewCoords(number_of_coordinates))
        } else {
            let mut float_coordinates = helpers::get_float_coordinates(coordinates);
            if float_coordinates[0] != float_coordinates[number_of_coordinates - 1] {
                float_coordinates.push(float_coordinates[0]);
                Ok(PolygonRing(float_coordinates))
            } else {
                Ok(PolygonRing(float_coordinates))
            }
        }
    }
+    
+    /// Returns an iterator of 2 64-bit float arrays.
+    pub fn iter(&self) -> Iter<[f64; 2]> {
+        self.0.iter()
+    }
}
```

Easy-peazy, we can just copy the code from `line_string.rs`. But wait, that can't be right, this is shared functionality. The Rustacean way of doing that is with traits. 

```rust
// curve.rs

pub trait Curve {
    /// Returns an iterator of 2 64-bit float arrays.
    fn iter(&self) -> Iter<[f64; 2]> {
        self.0.iter()
    }
}
```

```diff-rust
// lib.rs

+pub mod curve;
pub mod error;
pub mod helpers;
pub mod line_string;
pub mod point;
pub mod polygon;
```

But wait, that won't work because the compiler cant' guaranetee that the `Trait`will be implemented on `struct`s that have a zeroth field or that that zeroth field with have a `.iter`method.

Back to the drawing-board!

Another option is is a implement `Deref`, this will allow us to call any underlying `vec`methods on our new types that wrap a `vec`.

```diff-rust
// polygon.rs

...
    pub fn new<T: NumCast>(coordinates: Vec<[T; 2]>) -> Result<Self, GeometryError> {
        let number_of_coordinates = coordinates.len();
        if number_of_coordinates < 3 {
            Err(GeometryError::TooFewCoords(number_of_coordinates))
        } else {
            let mut float_coordinates = helpers::get_float_coordinates(coordinates);
            if float_coordinates[0] != float_coordinates[number_of_coordinates - 1] {
                float_coordinates.push(float_coordinates[0]);
                Ok(PolygonRing(float_coordinates))
            } else {
                Ok(PolygonRing(float_coordinates))
            }
        }
    }
}
+
+impl Deref for PolygonRing {
+    type Target = Vec<[f64; 2]>;
+
+    fn deref(&self) -> &Self::Target {
+        &self.0
+    }
+}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Polygon(Vec<PolygonRing>);

impl Polygon {
    /// Construct a new `Polygon` from a vector of vectors of 2-element arrays.
    ///
    /// # Examples:
+    ///
+    /// ```
+    /// use auto_gis_with_rust::polygon::Polygon;
+    ///
+    /// let polygon_1 = Polygon::new(vec![vec![[0., 0.], [0., 1.], [1., 1.], [0., 0.]]]).unwrap();
+    ///
+    /// assert_eq!("POLYGON ((0 0, 0 1, 1 1, 0 0))", polygon_1.to_string());
+    /// ```
    ///
    /// Construct a new `Polygon` from a vector of vectors of floats or a vector of vectors of integers.
    ///
    /// ```
    /// use auto_gis_with_rust::polygon::Polygon;
    ///
    /// let polygon_1 = Polygon::new(vec![vec![[0., 0.], [0., 1.], [1., 1.], [0., 0.]]]).unwrap();
    /// let polygon_2 = Polygon::new(vec![vec![[0, 0], [0, 1], [1, 1]]]).unwrap();
    ///
    /// assert_eq!(polygon_1, polygon_2)
    /// ```
    pub fn new<T: NumCast>(rings: Vec<Vec<[T; 2]>>) -> Result<Self, GeometryError> {
           pub fn new<T: NumCast>(rings: Vec<Vec<[T; 2]>>) -> Result<Self, GeometryError> {
        let polygon_rings: Vec<PolygonRing> = rings
            .into_iter()
            .map(|ring| PolygonRing::new(ring).unwrap())
            .collect();
        Ok(Polygon(polygon_rings))
    }
}
+
+impl Deref for Polygon {
+    type Target = Vec<PolygonRing>;
+
+    fn deref(&self) -> &Self::Target {
+        &self.0
+    }
+}
+
+impl fmt::Display for Polygon {
+    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
+        let rings = self
+            .iter()
+            .map(|ring| {
+                ring.iter().format_with(", ", |point, f| {
+                    f(&format_args!("{} {}", point[0], point[1]))
+                })
+            })
+            .format_with(", ", |ring, f| f(&format_args!("({})", ring)));
+        write!(f, "POLYGON ({})", rings)
+    }
+}
```

Cool, that works, time to:

### Dereference all the things!

```diff-rust
-use std::{fmt, slice::Iter};
+use std::{fmt, ops::Deref};
...
    pub fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
        let x_float: f64 = num_traits::cast(x).unwrap();
        let y_float: f64 = num_traits::cast(y).unwrap();
        Point([x_float, y_float])
    }
}
+
+impl Deref for Point {
+    type Target = [f64; 2];
+
+    fn deref(&self) -> &Self::Target {
+        &self.0
+    }
+}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
-        write!(f, "POINT ({} {})", self.0[0], self.0[1])
+        write!(f, "POINT ({} {})", self[0], self[1])
    }
}
...
-    /// Returns an iterator of `Points`
-    ///
-    /// # Example
-    /// ```
-    /// use auto_gis_with_rust::point::{Point, MultiPoint};
-    ///
-    /// let point_0 = Point::new(0.0, 0.0);
-    /// let point_1 = Point::new(1, 0);
-    /// let multi_point = MultiPoint(vec![point_0, point_1]);
-    ///
-    /// assert_eq!("POINT (0 0)", multi_point.iter().next().unwrap().to_string())
-    /// ```
-    pub fn iter(&self) -> Iter<Point> {
-        self.0.iter()
-    }
}
+
+impl Deref for MultiPoint {
+    type Target = Vec<Point>;
+
+    fn deref(&self) -> &Self::Target {
+        &self.0
+    }
+}

impl fmt::Display for MultiPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let points = self.iter().format_with(", ", |point, f| {
-            f(&format_args!("({} {})", point.0[0], point.0[1]))
+            f(&format_args!("({} {})", point[0], point[1]))
        });
        write!(f, "MULTIPOINT ({})", points)
    }
}
```

```diff-rust
// line_string.rs
use std::convert::TryFrom;
-use std::{fmt, slice::Iter};
+use std::fmt;
+use std::ops::Deref;
...
    pub fn new<T: NumCast>(coordinates: Vec<[T; 2]>) -> Result<Self, GeometryError> {
        let number_of_coordinates = coordinates.len();
        if number_of_coordinates < 2 {
            Err(GeometryError::TooFewCoords(number_of_coordinates))
        } else {
            let float_coordinates = helpers::get_float_coordinates(coordinates);
            Ok(LineString(float_coordinates))
        }
    }
-
-    /// Returns an iterator of 2 64-bit float arrays.
-    pub fn iter(&self) -> Iter<[f64; 2]> {
-        self.0.iter()
-    }
}
+
+impl Deref for LineString {
+    type Target = Vec<[f64; 2]>;
+
+    fn deref(&self) -> &Self::Target {
+        &self.0
+    }
+}
...
    pub fn new(linestrings: Vec<LineString>) -> Self {
        MultiLineString(linestrings)
    }
-
-    /// Returns an iterator of `LineStrings`
-    ///
-    /// # Examples
-    ///
-    /// ```
-    /// use auto_gis_with_rust::line_string::{LineString, MultiLineString};
-    ///
-    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 0.], [1., 1.]]).unwrap();
-    /// let line_string_2 = LineString::new(vec![[1., 2.], [0., 2.], [0., 1.]]).unwrap();
-    ///
-    /// let multi_line_string = MultiLineString::new(vec![line_string_1, line_string_2]);
-    ///
-    /// assert_eq!("LINESTRING (0 0, 1 0, 1 1)", multi_line_string.iter().next().unwrap().to_string())
-    /// ```
-    pub fn iter(&self) -> Iter<LineString> {
-        self.0.iter()
-    }
}
+
+impl Deref for MultiLineString {
+    type Target = Vec<LineString>;
+
+    fn deref(&self) -> &Self::Target {
+        &self.0
+    }
+}
```

```sh
...
   Doc-tests auto-gis-with-rust

running 12 tests
test src/line_string.rs - line_string::LineString::new (line 19) ... ok
test src/line_string.rs - line_string::MultiLineString::new (line 73) ... ok
test src/line_string.rs - line_string::LineString::new (line 29) ... ok
test src/helpers.rs - helpers::get_float_coordinates (line 7) ... ok
test src/point.rs - point::Point::from (line 52) ... ok
test src/point.rs - point::MultiPoint::new (line 74) ... ok
test src/line_string.rs - line_string::MultiLineString::try_from (line 117) ... ok
test src/point.rs - point::MultiPoint::from (line 110) ... FAILED
test src/point.rs - point::Point::new (line 16) ... ok
test src/polygon.rs - polygon::Polygon::new (line 58) ... ok
test src/polygon.rs - polygon::PolygonRing::new (line 18) ... ok
test src/polygon.rs - polygon::Polygon::new (line 68) ... ok

failures:

---- src/point.rs - point::MultiPoint::from (line 110) stdout ----
Test executable failed (exit status: 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"MULTIPOINT (0 0, 1 0)"`,
 right: `"MULTIPOINT ((0 0), (1 0))"`', src/point.rs:8:1
stack backtrace:
...
```

Wait, what?!

```diff-rust
// point.rs

...
impl<T: NumCast + Copy> From<Vec<[T; 2]>> for MultiPoint {
    /// Construct a `MultiPoint` from a vector of 2-element arrays.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::point::MultiPoint;
    ///
    /// let multi_point = MultiPoint::from(vec![[0.0, 0.0], [1.0, 0.0]]);
    ///
-    /// assert_eq!("MULTIPOINT (0 0, 1 0)", multi_point.to_string());
+    /// assert_eq!("MULTIPOINT ((0 0), (1 0))", multi_point.to_string());
    /// ```
    fn from(items: Vec<[T; 2]>) -> Self {
        let points: Vec<Point> = items.into_iter().map(Point::from).collect();
        MultiPoint::new(points)
    }
...
```

```sh
...
   Doc-tests auto-gis-with-rust

running 12 tests
test src/line_string.rs - line_string::LineString::new (line 19) ... ok
test src/line_string.rs - line_string::MultiLineString::new (line 73) ... ok
test src/helpers.rs - helpers::get_float_coordinates (line 7) ... ok
test src/line_string.rs - line_string::LineString::new (line 29) ... ok
test src/point.rs - point::MultiPoint::new (line 74) ... ok
test src/point.rs - point::Point::from (line 52) ... ok
test src/point.rs - point::MultiPoint::from (line 110) ... ok
test src/line_string.rs - line_string::MultiLineString::try_from (line 117) ... ok
test src/point.rs - point::Point::new (line 16) ... ok
test src/polygon.rs - polygon::Polygon::new (line 58) ... ok
test src/polygon.rs - polygon::PolygonRing::new (line 18) ... ok
test src/polygon.rs - polygon::Polygon::new (line 68) ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.92s
```

Much better, but something doesn't feel right, we're basically copy-pasting the implement-`Deref`code all over the place and that's a sure sign we need some form of abstraction.

Macros to the rescue!

Instead of copy-pasting the implement-`Deref`code, we can define an implement `Deref`macro that will generate the implementation block for us.

```diff-rust
// helpers.rs
// ...
pub fn get_float_coordinates<T: NumCast>(coordinates: Vec<[T; 2]>) -> Vec<[f64; 2]> {
    let float_coordinates: Vec<[f64; 2]> = coordinates
        .into_iter()
        .map(|coordinate| {
            coordinate.map(|coordinate| -> f64 { num_traits::cast(coordinate).unwrap() })
        })
        .collect();
    float_coordinates
}
+
+#[macro_export]
+macro_rules! implement_deref {
+    ($type:ty, $target:ty) => {
+        impl Deref for $type {
+            type Target = $target;
+
+            fn deref(&self) -> &Self::Target {
+                &self.0
+            }
+        }
+    };
+}
```

Now we just have replace our implement `Deref`blocks with macro calls:

```diff-rust
// point.rs

use num_traits::{self, NumCast};
+
+use crate::implement_deref;
+
#[derive(Debug, PartialEq, PartialOrd)]

// ...

-impl Deref for Point {
-    type Target = [f64; 2];
-
-    fn deref(&self) -> &Self::Target {
-        &self.0
-    }
-}
+implement_deref!(Point, [f64; 2]);

// ...

-impl Deref for MultiPoint {
-    type Target = Vec<Point>;
-
-    fn deref(&self) -> &Self::Target {
-        &self.0
-    }
-}
+implement_deref!(MultiPoint, Vec<Point>);

// ...
```

```diff-rust
// line_string.rs

// ..

use crate::error::GeometryError;
-use crate::helpers;
+use crate::{helpers, implement_deref};

#[derive(Debug, PartialEq, PartialOrd)]

// ...

-impl Deref for LineString {
-    type Target = Vec<[f64; 2]>;
-
-    fn deref(&self) -> &Self::Target {
-        &self.0
-    }
-}
+implement_deref!(LineString, Vec<[f64; 2]>);

// ...

-impl Deref for MultiLineString {
-    type Target = Vec<LineString>;
-
-    fn deref(&self) -> &Self::Target {
-        &self.0
-    }
-}
+implement_deref!(MultiLineString, Vec<LineString>);

// ...

```

```diff-rust
// polygon.rs

// ..

use crate::error::GeometryError;
-use crate::helpers;
+use crate::{helpers, implement_deref};

#[derive(Debug, PartialEq, PartialOrd)]

// ...

-impl Deref for PolygonRing {
-    type Target = Vec<[f64; 2]>;
-
-    fn deref(&self) -> &Self::Target {
-        &self.0
-    }
-}
+implement_deref!(PolygonRing, Vec<[f64; 2]>);

// ...

-impl Deref for Polygon {
-    type Target = Vec<PolygonRing>;
-
-    fn deref(&self) -> &Self::Target {
-        &self.0
-    }
-}
+implement_deref!(Polygon, Vec<PolygonRing>);

// ...

-impl Deref for MultiPolygon {
-    type Target = Vec<Polygon>;
-
-    fn deref(&self) -> &Self::Target {
-        &self.0
-    }
-}
+implement_deref!(MultiPolygon, Vec<Polygon>);

// ...

```

### `new`and WKT for `MultiPolygon`

```diff-rust
// polygon.rs

// ...

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rings = self
            .iter()
            .map(|ring| {
                ring.iter().format_with(", ", |point, f| {
                    f(&format_args!("{} {}", point[0], point[1]))
                })
            })
            .format_with(", ", |ring, f| f(&format_args!("({})", ring)));
        write!(f, "POLYGON ({})", rings)
    }
}
+
+#[derive(Debug, PartialEq, PartialOrd)]
+pub struct MultiPolygon(Vec<Polygon>);
+
+impl MultiPolygon {
+    /// Construct a new `MultiPolygon` from a vector of `Polygon`s.
+    ///
+    /// # Examples:
+    ///
+    /// ```
+    /// use auto_gis_with_rust::polygon::{MultiPolygon, Polygon};
+    ///
+    /// let polygon_1 = Polygon::new(vec![vec![[0., 0.], [0., 1.], [1., 1.], [1., 0.], [0., 0.]]]).unwrap();
+    /// let polygon_2 = Polygon::new(vec![vec![[1, 1], [1, 2], [2, 2], [2, 1]]]).unwrap();
+    ///
+    /// let multi_polygon = MultiPolygon::new(vec![polygon_1, polygon_2]);
+    ///
+    /// assert_eq!("MULTIPOLYGON (((0 0, 0 1, 1 1, 1 0, 0 0)), ((1 1, 1 2, 2 2, 2 1, 1 1)))", multi_polygon.to_string());
+    /// ```
+    pub fn new(polygons: Vec<Polygon>) -> Self {
+        MultiPolygon(polygons)
+    }
+}
+
+implement_deref!(MultiPolygon, Vec<Polygon>);
+
+impl fmt::Display for MultiPolygon {
+    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
+        let polygons = self
+            .iter()
+            .map(|polygon| {
+                polygon
+                    .iter()
+                    .map(|ring| {
+                        ring.iter().format_with(", ", |point, f| {
+                            f(&format_args!("{} {}", point[0], point[1]))
+                        })
+                    })
+                    .format_with(", ", |ring, f| f(&format_args!("({})", ring)))
+            })
+            .format_with(", ", |polygon, f| f(&format_args!("({})", polygon)));
+        write!(f, "MULTIPOLYGON ({})", polygons)
+    }
+}

// ...

```

### From a vector of vectors of vectors of 2-element arrays for `MultiPolygon`

```diff-rust
// polygon.rs

// ...

impl fmt::Display for MultiPolygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let polygons = self
            .iter()
            .map(|polygon| {
                polygon
                    .iter()
                    .map(|ring| {
                        ring.iter().format_with(", ", |point, f| {
                            f(&format_args!("{} {}", point[0], point[1]))
                        })
                    })
                    .format_with(", ", |ring, f| f(&format_args!("({})", ring)))
            })
            .format_with(", ", |polygon, f| f(&format_args!("({})", polygon)));
        write!(f, "MULTIPOLYGON ({})", polygons)
    }
}
+
+impl<T: NumCast> TryFrom<Vec<Vec<Vec<[T; 2]>>>> for MultiPolygon {
+    type Error = GeometryError;
+
+    /// Tries to convert a vector of vectors of vectors of 2-float arrays into a `MultiPolygon`.
+    ///
+    /// # Examples:
+    ///
+    /// ```
+    /// use std::convert::TryFrom;
+    /// use auto_gis_with_rust::polygon::MultiPolygon;
+    ///
+    /// let multi_polygon_1 = MultiPolygon::try_from(vec![
+    ///     vec![
+    ///         vec![[0., 0.], [0., 1.], [1., 1.], [1., 0.], [0., 0.]],
+    ///     ],
+    ///     vec![
+    ///         vec![[1., 1.], [1., 2.], [2., 2.], [2., 1.]],
+    ///     ],
+    /// ]).unwrap();
+    ///
+    /// assert_eq!("MULTIPOLYGON (((0 0, 0 1, 1 1, 1 0, 0 0)), ((1 1, 1 2, 2 2, 2 1, 1 1)))", multi_polygon_1.to_string());
+    /// ```
+    ///
+    /// Or tries to convert a vector of vectors of vectors of 2-integer arrays into a `MultiPolygon`.
+    ///
+    /// ```
+    /// # use std::convert::TryFrom;
+    /// # use auto_gis_with_rust::polygon::MultiPolygon;
+    /// #    
+    /// # let multi_polygon_1 = MultiPolygon::try_from(vec![
+    /// #     vec![
+    /// #         vec![[0., 0.], [0., 1.], [1., 1.], [1., 0.], [0., 0.]],
+    /// #     ],
+    /// #     vec![
+    /// #         vec![[1., 1.], [1., 2.], [2., 2.], [2., 1.]],
+    /// #     ],
+    /// # ]).unwrap();
+    ///
+    /// let multi_polygon_2 = MultiPolygon::try_from(vec![
+    ///     vec![
+    ///         vec![[0, 0], [0, 1], [1, 1], [1, 0], [0, 0]],
+    ///     ],
+    ///     vec![
+    ///         vec![[1, 1], [1, 2], [2, 2], [2, 1]],
+    ///     ],
+    /// ]).unwrap();
+    ///
+    /// assert_eq!(multi_polygon_1, multi_polygon_2);
+    /// ```
+    fn try_from(vectors: Vec<Vec<Vec<[T; 2]>>>) -> Result<Self, GeometryError> {
+        let polygons: Result<Vec<Polygon>, GeometryError> =
+            vectors.into_iter().map(Polygon::new).collect();
+        Ok(MultiPolygon::new(polygons?))
+    }
+}
```

Right, that's enough for today, we have a `MultiPolygon`type that can be constructed from a vector of `Polygon`s or a vector of vectors of vectors of 2-element arrays and can be converted to WKT. 
