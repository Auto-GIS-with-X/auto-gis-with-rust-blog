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

## MultiLineString

### WKT for `LineString`

Okay, `MultiLineString` next but before we do that, let's implement WKT for `LineString`.

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

## MultiPolygon