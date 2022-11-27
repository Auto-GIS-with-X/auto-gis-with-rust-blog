---
title: "2: Attributes of geometries"
eleventyNavigation:
  key: "2: Attributes of geometries"
  parent: Problems
  order: 1
---

Okay, next problem. Let's have a look at the question:

>1: Create a function called `centroid()` that has one parameter called `geom`. The function should take any kind of Shapely's geometric-object as an input, and return a centroid of that geometry. In addition, you should take care that the function is used as it should:
>
> - Inside the function, you should first check with assert - functionality that the input is a Shapely Point, LineString or Polygon geometry (see lesson 6 from the Geo-Python course and hints for help). If something else than a list is passed for the function, you should return an Error message: "Input should be a Shapely geometry!"

In essence, we need:

- A function, which only operates on valid geometry objects, that returns the centroid of a given geometry.

Sounds like we need a `geometry`trait. To paraphrase the chapter about traits in [The Book](https://doc.rust-lang.org/book/ch10-02-traits.html), a trait allows us to define shared functionality.

```rust
// geometry.rs

use crate::point::Point;

pub trait Geometry {
    fn get_centroid(&self) -> Point;
}
```

Now we need to implement this trait for our geometry objects.

## Compute the centroid of a `Point`

First of all, the centre point of a point is the point itself, so this should be easy. We just need to import the `Geometry` trait and the implement it for our `Point` type.

```diff-rust
// point.rs

+use crate::geometry::Geometry;
use num_traits::{self, NumCast};

#[derive(Debug, PartialEq, PartialOrd)]

...

+impl Geometry for Point {
+    /// Compute the geometric center of a geometry.
+    ///
+    /// For a `Point`, this is a reference to the `Point` itself.
+    ///
+    /// ```
+    /// use auto_gis_with_rust::point::Point;
+    ///
+    /// let point = Point::new(0.0, 1.0);
+    ///
+    /// assert_eq!(point.centroid(), &point);
+    /// ```
+    fn centroid(&self) -> &Point {
+        self
+    }
+}
```

Right, let's run the Doctest.

```sh
error[E0599]: no method named `centroid` found for struct `Point` in the current scope
 --> src/point.rs:39:18
  |
8 | assert_eq!(point.centroid(), &point);
  |                  ^^^^^^^^ method not found in `Point`
  |
 ::: /home/ed/repos/auto-gis-with-rust/src/geometry.rs:4:8
  |
4 |     fn centroid(&self) -> &Point;
  |        -------- the method is available for `Point` here
  |
  = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
  |
2 | use crate::auto_gis_with_rust::geometry::Geometry;
  |
```

Oh dear, we forgot to import the trait object in the Doctest. Let's fix that.

```diff-rust
// point.rs

    /// ```
+    /// use auto_gis_with_rust::geometry::Geometry;
    /// use auto_gis_with_rust::point::Point;
```

Try again.

```sh
running 1 test
test src/point.rs - point::Point::centroid (line 34) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.36s
```

Boom!

## Compute the centroid of a `LineString`

Okay, this is a little more complicated. Let's approach this iteratively.

The centroid of the most simple `LineString`, a line between two points, is the mid-point between those end-points. This most simple `LineString` is called a `LineSegment`, so let's define a `LineSegment` type.

```diff-rust
// line_string.rs

...

use crate::error::GeometryError;
use crate::helpers;

+#[derive(Debug, PartialEq, PartialOrd)]
+pub struct LineSegment([[f64; 2]; 2]);
+
+impl LineSegment {
+    /// A straight line connecting two points.
+    ///
+    /// # Examples:
+    ///
+    /// Construct a new `LineSegment` from a 2-element array of 2-element arrays.
+    ///
+    /// ```
+    /// use auto_gis_with_rust::line_string::LineSegment;
+    ///
+    /// let line_segment_1 = LineSegment::new([[0., 0.], [1., 1.]]);
+    /// let line_segment_2 = LineSegment::new([[0, 0], [1, 1]]);
+    ///
+    /// assert_eq!(line_segment_1, line_segment_2)
+    /// ```
+    pub fn new<T: NumCast>(coordinates: [[T; 2]; 2]) -> Self {
+        let float_coordinates: [[f64; 2]; 2] = coordinates.map(|coordinate| {
+            coordinate.map(|coordinate| -> f64 { num_traits::cast(coordinate).unwrap() })
+        });
+        LineSegment(float_coordinates)
+    }
+}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LineString(Vec<[f64; 2]>);
...
```
