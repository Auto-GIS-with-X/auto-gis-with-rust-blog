---
title: "Problem 1: Creating basic geometries"
layout: 'home.html'
---

## Part 1: Creating points

> Create a function called `create_point_geom()` that has two parameters (`x_coord`, `y_coord`). Function should create a `shapely` `Point` geometry object and return that.

My interpretation of this is that I need a `Point` `struct` and a `new` constructor that takes `x` and `y` parameters, so let's get started:

```rust
// main.rs

#[derive(Debug)]
pub struct Point([f64; 2]);

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point([x, y])
    }
}

fn main() {
    let point = Point::new(0.0, 1.0);
    dbg!(point);
}
```

This is pretty basic. It works but it only handles `f64`.


We can handle any number by using the [`NumCast`](https://docs.rs/num-traits/0.2.15/num_traits/cast/trait.NumCast.html) trait from the [`num_traits`](https://crates.io/crates/num-traits) crate.


To add `num-traits` using the excellent [`cargo-edit`](https://crates.io/crates/cargo-edit) crate run the following in our terminal:

```sh
$ cargo add num-traits
```

Now the caller can pass any type that can be cast to a `f64`.

```diff-rust
// main.rs

#[derive(Debug)]
pub struct Point([f64; 2]);

impl Point {
-    fn new(x: f64, y: f64) -> Self {
+    fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
+        let x_float: f64 = num_traits::cast(x).unwrap();
+        let y_float: f64 = num_traits::cast(y).unwrap();
-        Point([x, y])
+        Point([x_float, y_float])
    }
}

fn main() {
    let point = Point::new(0, 1);
    dbg!(point);
}
```

That's better! Now the caller can pass any type that can be cast to `f64`, they can even pass two different types that can be cast to `f64`. I'm not sure why you'd want to do that but you can and that's the main thing. 

Now to add some documentation. One of my favourite things about Rust is that it supports [documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) out-of-the-box. This means that we can write examples and unit tests in one go. 

However, to compare the two `Point` objects we need to derive `PartialEq` and `PartialOrd`.

This is also a good time to move the `Point` struct and it's methods from `main.rs` to it's own file.

```diff-rust
// point.rs

use num_traits::{self, NumCast};

- #[derive(Debug)]
+ #[derive(Debug, PartialEq, PartialOrd)]
pub struct Point([f64; 2]);

impl Point {
+    /// Construct a new `Point`.
+    ///
+    /// # Examples:
+    ///
+    /// Construct a new point from x and y floats or x and y integers.
+    ///
+    /// ```
+    /// use auto_gis_with_rust::Point;
+    ///
+    /// let point_0 = Point::new(0.0, 1.0);
+    /// let point_1 = Point::new(0, 1);
+    ///
+    /// assert_eq!(point_0, point_1);
+    /// ```
    pub fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
        let x_float: f64 = num_traits::cast(x).unwrap();
        let y_float: f64 = num_traits::cast(y).unwrap();
        Point([x_float, y_float])
    }
}
```

And expose it via `lib.rs`.

```rust
// lib.rs

pub mod point;
```

Now we can import our `Point struct` in `main.rs`.

```diff-rust
// main.rs

- use num_traits::{self, NumCast};
+ use auto_gis_with_rust::point::Point;

- #[derive(Debug)]
- pub struct Point([f64; 2]);

- impl Point {
-    fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
-       let x_float: f64 = num_traits::cast(x).unwrap();
-       let y_float: f64 = num_traits::cast(y).unwrap();
-        Point([x_float, y_float])
-    }
- }

fn main() {
    let point = Point::new(0, 1);
    dbg!(point);
}
```

## Part 2: Creating LineStrings

> 2: Create a function called create_line_geom() that takes a list of Shapely Point objects as parameter called points and returns a LineString object of those input points. In addition, you should take care that the function is used as it should:
> - Inside the function, you should first check with assert -functionality that the input is a list (see lesson 6 from the Geo-Python course and hints for this exercise). If something else than a list is passed for the function, you should return an Error message: "Input should be a list!"
> - You should also check with assert that the input list contains at least two values. If not, return an Error message: "LineString object requires at least two Points!"
> - Optional: Finally, you should check with assert that all values in the input list are truly Shapely Points. If not, return an Error message: "All list values should be Shapely Point objects!"

So, we need:

- A LineString object
- A constructor that takes a list of "points"
- An error message if that list doesn't contain two or more items

We don't need to worry about checking that the input will be a list (or a `Vec` in our case) because Rust is a statically typed language and the compiler will complain if we try and call our `new` constructor on anything other than the type we declare in the function signature. However, as noted above, we will need to check that the `Vec` has two or more items because empty or single item `Vec`s are still valid `Vec`s.  

```rust
// line_string.rs

use num_traits::NumCast;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LineString(Vec<[f64; 2]>);

impl LineString {
    pub fn new<T: NumCast>(points: Vec<[T; 2]>) -> Self {
        let float_points = points
            .into_iter()
            .map(|point| {
                point.map(|coordinate| {
                    let float_point: f64 = num_traits::cast(coordinate).unwrap();
                    float_point
                })
            })
            .collect();
        LineString(float_points)
    }
}
```

Now we have a new module, we need to import it:

```diff-rust
// lib.rs

+pub mod line_string;
pub mod point;
```

Like last time, let's add a docstring with an example we can test too:

```diff-rust
// line_string.rs

use num_traits::NumCast;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LineString(Vec<[f64; 2]>);

impl LineString {
+    /// Construct a new `LineString` from a vector of 2-element arrays.
+    ///
+    /// # Examples:
+    ///
+    /// Construct a new `LineString` from a vector of floats or a vector of integers.
+    ///
+    /// ```
+    /// use auto_gis_with_rust::line_string::LineString;
+    ///
+    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 1.]]);
+    /// let line_string_2 = LineString::new(vec![[0, 0], [1, 1]]);
+    ///
+    /// assert_eq!(line_string_1, line_string_2)
+    /// ```
    pub fn new<T: NumCast>(points: Vec<[T; 2]>) -> Self {
        let float_points = points
            .into_iter()
            .map(|point| {
                point.map(|coordinate| {
                    let float_point: f64 = num_traits::cast(coordinate).unwrap();
                    float_point
                })
            })
            .collect();
        LineString(float_points)
    }
}
```

So, we've got a `LineString` struct that takes a list of `points` and the doctest proves it does what we think it should. Let's handle the possibility that we receive less than 2 coordinates.

To do this, we need an error type. Luckily, `thiserror` has our back:

```rust
// error.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeometryError {
    #[error("too few coordinates, expected 2 or more, found {0})")]
    TooFewCoords(usize),
}
```

```diff-rust
// line_string.rs

use num_traits::NumCast;

+use crate::error::GeometryError;

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
-    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 1.]]);
-    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 1.]]);
+    /// let line_string_2 = LineString::new(vec![[0, 0], [1, 1]]).unwrap();
+    /// let line_string_2 = LineString::new(vec![[0, 0], [1, 1]]).unwrap();
    ///
    /// assert_eq!(line_string_1, line_string_2)
    /// ```
-    pub fn new<T: NumCast>(points: Vec<[T; 2]>) -> Self {
-        let float_points = points
-            .into_iter()
-            .map(|point| {
-                point.map(|coordinate| {
-                    let float_point: f64 = num_traits::cast(coordinate).unwrap();
-                    float_point
-                })
-            })
-            .collect();
-        LineString(float_points)
-    }
+    pub fn new<T: NumCast>(points: Vec<[T; 2]>) -> Result<Self, GeometryError> {
+        let number_of_points = points.len();
+        if number_of_points < 2 {
+            Err(GeometryError::TooFewCoords(number_of_points))
+        } else {
+            let float_points = points
+                .into_iter()
+                .map(|point| {
+                    point.map(|coordinate| {
+                        let float_point: f64 = num_traits::cast(coordinate).unwrap();
+                        float_point
+                    })
+                })
+                .collect();
+            Ok(LineString(float_points))
+        }
+    }
}
```

## Part 3

>3: Create a function called create_poly_geom() that has one parameter called coords. coords parameter should containt a list of coordinate tuples. The function should create and return a Polygon object based on these coordinates.
> - Inside the function, you should first check with assert -functionality that the input is a list (see lesson 6 and hints). If something else than a list is passed for the function, you should return an Error message: "Input should be a list!"
> - You should also check with assert that the input list contains at least three values. If not, return an Error message: "Polygon object requires at least three Points!"
> - Check the data type of the objects in the input list. All values in the input list should be tuples. If not, return an error message: "All list values should be coordinate tuples!" using assert.
> - Optional: Allow also an input containing a list of Shapely Point objects. If coords contanis a list of Shapely Point objects, return a polygon based on these points. If the input is neither a list of tuples, nor a list of Points, return an appropriate error message using assert.

So, we need:

- A polygon object
- A `new` constructor that takes a vector of coordinates
- An error message if that vector doesn't contain three or more items

Additionally, we need to check whether the first and last items in the vector are equal and, if they aren't, we need to close the ring.

- Assume CCW

`polygon.rs` {class="file-name"}

```rust
use num_traits::NumCast;

use crate::error::GeometryError;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Polygon(Vec<[f64; 2]>);

impl Polygon {
    /// Construct a new `Polygon` from a vector of 2-element arrays.
    ///
    /// # Examples:
    ///
    /// Construct a new `Polygon` from a vector of floats or a vector of integers.
    ///
    /// ```
    /// use auto_gis_with_rust::polygon::Polygon;
    ///
    /// let polygon_1 = Polygon::new(vec![[0., 0.], [0., 1.], [1., 1.], [0., 0.]]).unwrap();
    /// let polygon_2 = Polygon::new(vec![[0, 0], [0, 1], [1, 1]]).unwrap();
    ///
    /// assert_eq!(polygon_1, polygon_2)
    /// ```
    pub fn new<T: NumCast>(coordinates: Vec<[T; 2]>) -> Result<Self, GeometryError> {
        let number_of_coordinates = coordinates.len();
        if number_of_coordinates < 3 {
            Err(GeometryError::TooFewCoords(number_of_coordinates))
        } else {
            let mut float_coordinates: Vec<[f64; 2]> = coordinates
                .into_iter()
                .map(|coordinate| {
                    coordinate.map(|coordinate| {
                        let float_coordinate: f64 = num_traits::cast(coordinate).unwrap();
                        float_coordinate
                    })
                })
                .collect();
            if float_coordinates[0] != float_coordinates[number_of_coordinates - 1] {
                float_coordinates.push(float_coordinates[0]);
                Ok(Polygon(float_coordinates))
            } else {
                Ok(Polygon(float_coordinates))
            }
        }
    }
}
```

- Update `lib.rs`

`lib.rs` {class="file-name"}

```diff-rust
pub mod error;
pub mod line_string;
pub mod point;
+pub mod polygon;
```

- One problem, this approach doesn't account for internal rings.

`polygon.rs` {class="file-name"}

```diff-rust
use num_traits::NumCast;

use crate::error::GeometryError;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct PolygonRing(Vec<[f64; 2]>);

impl PolygonRing {
-    /// Construct a new `Polygon` from a vector of 2-element arrays.
+    /// Construct a new `PolygonRing` from a vector of 2-element arrays.
    ///
    /// # Examples:
    ///
-    /// Construct a new `Polygon` from a vector of floats or a vector of integers.
+    /// Construct a new `PolygonRing` from a vector of floats or a vector of integers.
    ///
    /// ```
-    /// use auto_gis_with_rust::polygon::Polygon;
+    /// use auto_gis_with_rust::polygon::PolygonRing;
    ///
-    /// let polygon_1 = Polygon::new(vec![[0., 0.], [0., 1.], [1., 1.], [0., 0.]]).unwrap();
-    /// let polygon_2 = Polygon::new(vec![[0, 0], [0, 1], [1, 1]]).unwrap();
+    /// let polygon_ring_1 = PolygonRing::new(vec![[0., 0.], [0., 1.], [1., 1.], [0., 0.]]).unwrap();
+    /// let polygon_ring_2 = PolygonRing::new(vec![[0, 0], [0, 1], [1, 1]]).unwrap();
    ///
-    /// assert_eq!(polygon_1, polygon_2)
+    /// assert_eq!(polygon_ring_1, polygon_ring_2)
    /// ```
    pub fn new<T: NumCast>(coordinates: Vec<[T; 2]>) -> Result<Self, GeometryError> {
        let number_of_coordinates = coordinates.len();
        if number_of_coordinates < 3 {
            Err(GeometryError::TooFewCoords(number_of_coordinates))
        } else {
            let mut float_coordinates: Vec<[f64; 2]> = coordinates
                .into_iter()
                .map(|coordinate| {
                    coordinate.map(|coordinate| {
                        let float_coordinate: f64 = num_traits::cast(coordinate).unwrap();
                        float_coordinate
                    })
                })
                .collect();
            if float_coordinates[0] != float_coordinates[number_of_coordinates - 1] {
                float_coordinates.push(float_coordinates[0]);
-                Ok(Polygon(float_coordinates))
+                Ok(PolygonRing(float_coordinates))
            } else {
-                Ok(Polygon(float_coordinates))
+                Ok(PolygonRing(float_coordinates))
            }
        }
    }
}
+
+#[derive(Debug, PartialEq, PartialOrd)]
+pub struct Polygon(Vec<PolygonRing>);
+
+impl Polygon {
+    /// Construct a new `Polygon` from a vector of vectors of 2-element arrays.
+    ///
+    /// # Examples:
+    ///
+    /// Construct a new `Polygon` from a vector of vectors of floats or a vector of vectors of integers.
+    ///
+    /// ```
+    /// use auto_gis_with_rust::polygon::Polygon;
+    ///
+    /// let polygon_1 = Polygon::new(vec![vec![[0., 0.], [0., 1.], [1., 1.], [0., 0.]]]).unwrap();
+    /// let polygon_2 = Polygon::new(vec![vec![[0, 0], [0, 1], [1, 1]]]).unwrap();
+    ///
+    /// assert_eq!(polygon_1, polygon_2)
+    /// ```
+    pub fn new<T: NumCast>(rings: Vec<Vec<[T; 2]>>) -> Result<Self, GeometryError> {
+        let polygon_rings: Vec<PolygonRing> = rings
+            .into_iter()
+            .map(|ring| PolygonRing::new(ring).unwrap())
+            .collect();
+        Ok(Polygon(polygon_rings))
+    }
}
```

Okay, so this covers the spirit of the first problem, if not the letter. We have method that returns a `Point` from an x and y corrdinate, a method that returns a `LineString` from a vector of coordinates, and a method that returns a `Polygon` from a vector of vectors of coordinates.

One piece of refactoring: I'm using the same pattern to convert a vector of two item arrays of generics that implement `NumCast` into a vector of two item arrays of floats in both `LineString::new` and `PolygonRing::new`. Let's abstract that pattern into a funtion.

`helpers.rs` {class="file-name"}

```rust
use num_traits::{self, NumCast};

/// Convert a vector of two-item arrays of generics that implement `NumCast` into a vector of two-item arrays of floats.
///
/// Examples:
///
/// ```
/// use auto_gis_with_rust::helpers::get_float_coordinates;
///
/// let output = get_float_coordinates(vec![[0, 0], [0, 1], [1, 1]]);
/// let expected = vec![[0., 0.], [0., 1.], [1., 1.]];
///
/// assert_eq!(output, expected)
/// ```
pub fn get_float_coordinates<T: NumCast>(coordinates: Vec<[T; 2]>) -> Vec<[f64; 2]> {
    let float_coordinates: Vec<[f64; 2]> = coordinates
        .into_iter()
        .map(|coordinate| {
            coordinate.map(|coordinate| -> f64 { num_traits::cast(coordinate).unwrap() })
        })
        .collect();
    float_coordinates
}
```

`polygon.rs` {class="file-name"}

```diff-rust
use num_traits::NumCast;

use crate::error::GeometryError;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct PolygonRing(Vec<[f64; 2]>);

impl PolygonRing {
    /// Construct a new `PolygonRing` from a vector of 2-element arrays.
    ///
    /// # Examples:
    ///
    /// Construct a new `PolygonRing` from a vector of floats or a vector of integers.
    ///
    /// ```
    /// use auto_gis_with_rust::polygon::PolygonRing;
    ///
    /// let polygon_ring_1 = PolygonRing::new(vec![[0., 0.], [0., 1.], [1., 1.], [0., 0.]]).unwrap();
    /// let polygon_ring_2 = PolygonRing::new(vec![[0, 0], [0, 1], [1, 1]]).unwrap();
    ///
    /// assert_eq!(polygon_ring_1, polygon_ring_2)
    /// ```
    pub fn new<T: NumCast>(coordinates: Vec<[T; 2]>) -> Result<Self, GeometryError> {
        let number_of_coordinates = coordinates.len();
        if number_of_coordinates < 3 {
            Err(GeometryError::TooFewCoords(number_of_coordinates))
        } else {
-            let mut float_coordinates: Vec<[f64; 2]> = coordinates
-                .into_iter()
-                .map(|coordinate| {
-                    coordinate.map(|coordinate| {
-                        let float_coordinate: f64 = num_traits::cast(coordinate).unwrap();
-                        float_coordinate
-                    })
-                })
-                .collect();
+            let mut float_coordinates = helpers::get_float_coordinates(coordinates);
            if float_coordinates[0] != float_coordinates[number_of_coordinates - 1] {
                float_coordinates.push(float_coordinates[0]);
                Ok(PolygonRing(float_coordinates))
            } else {
                Ok(PolygonRing(float_coordinates))
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Polygon(Vec<PolygonRing>);

impl Polygon {
    /// Construct a new `Polygon` from a vector of vectors of 2-element arrays.
    ///
    /// # Examples:
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
        let polygon_rings: Vec<PolygonRing> = rings
            .into_iter()
            .map(|ring| PolygonRing::new(ring).unwrap())
            .collect();
        Ok(Polygon(polygon_rings))
    }
}
```
