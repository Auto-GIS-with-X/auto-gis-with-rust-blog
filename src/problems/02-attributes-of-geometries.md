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

In essence, we need a function, which only operates on valid geometry objects, that returns the centroid of a given geometry.

Sounds like we need a `geometry`trait. To paraphrase the chapter about traits in [The Book](https://doc.rust-lang.org/book/ch10-02-traits.html), a trait allows us to define shared functionality.

```rust
// geometry.rs

use crate::point::Point;

pub trait Geometry {
    fn get_centroid(&self) -> Point;
}
```

Add this module to the library:

```diff-rust
pub mod error;
+pub mod geometry;
pub mod helpers;
pub mod line_string;
pub mod point;
pub mod polygon;
```

Now we need to implement this trait for our geometry objects.

## Compute the centroid of a `Point`

First of all, the centroid of a point is the point itself, so this should be easy. We just need to import the `Geometry` trait and the implement it for our `Point` type.

However, we want to return a new `Point`so first, let's implement some convenience methods that will make it easier to do that.

```diff-rust
// point.rs

// ...

    pub fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
        let x_float: f64 = num_traits::cast(x).unwrap();
        let y_float: f64 = num_traits::cast(y).unwrap();
        Point([x_float, y_float])
    }
+    
+    /// Return the x-coordinate value for this `Point`.
+    ///
+    /// # Examples:
+    ///
+    /// ```
+    /// use auto_gis_with_rust::point::Point;
+    ///
+    /// let point = Point::new(0.0, 1.0);
+    /// let x = point.x();
+    ///
+    /// assert_eq!(x, 0f64);
+    /// ```
+    pub fn x(&self) -> f64 {
+        self[0]
+    }
+    
+    /// Return the y-coordinate value for this `Point`.
+    ///
+    /// # Examples:
+    ///
+    /// ```
+    /// use auto_gis_with_rust::point::Point;
+    ///
+    /// let point = Point::new(0.0, 1.0);
+    /// let y = point.y();
+    ///
+    /// assert_eq!(y, 1f64);
+    /// ```
+    pub fn y(&self) -> f64 {
+        self[1]
+    }
}

implement_deref!(Point, [f64; 2]);

// ...
```

Now we have these convenience methods, we might as well use them.

```diff-rust
// point.rs

// ...

implement_deref!(Point, [f64; 2]);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
-        write!(f, "POINT ({} {})", self[0], self[1])
+        write!(f, "POINT ({} {})", self.x(), self.y())
    }
}

impl<T: NumCast + Copy> From<[T; 2]> for Point {

// ...

```


```diff-rust
// point.rs

+use crate::geometry::Geometry;
use num_traits::{self, NumCast};

#[derive(Debug, PartialEq, PartialOrd)]

// ...

    fn from(coordinates: [T; 2]) -> Self {
        Point::new(coordinates[0], coordinates[1])
    }
}

+impl Geometry for Point {
+    /// Compute the geometric center of a geometry.
+    ///
+    /// For a `Point`, this is a new `Point` with the same coordinates.
+    ///
+    /// ```
+    /// use auto_gis_with_rust::point::Point;
+    ///
+    /// let point = Point::new(0.0, 1.0);
+    /// let expected_centroid = Point::new(0.0, 1.0);
+    ///
+    /// assert_eq!(point.centroid(), expected_centroid);
+    /// ```
+    fn centroid(&self) -> Point {
+        Point::new(self.x(), self.y())
+    }
+}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct MultiPoint(pub Vec<Point>);

// ...

```

Right, let's run the Doctest.

```sh
error[E0599]: no method named `centroid` found for struct `Point` in the current scope
 --> src/point.rs:39:18
  |
8 | assert_eq!(point.centroid(), expected_centroid);
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

## Compute the centroid of a `MultiPoint`

```diff-rust
// geometry.rs

// ...

pub trait Geometry {
    fn centroid(&self) -> Point;
}
+
+pub trait GeometryCollection<T: Geometry> {
+    fn number_of_geometries(&self) -> usize;
+
+    fn geometry_number(&self, number: usize) -> T;
+}
```

```diff-rust
// point.rs

// ...

use num_traits::{self, NumCast};

-use crate::geometry::Geometry;
+use crate::geometry::{Geometry, GeometryCollection};
use crate::implement_deref;

// ...

    fn from(items: Vec<[T; 2]>) -> Self {
        let points: Vec<Point> = items.into_iter().map(Point::from).collect();
        MultiPoint::new(points)
    }
}

+impl GeometryCollection<Point> for MultiPoint {
+    /// Returns the number of `Point`s in this `MultiPoint` collection.
+    ///
+    /// # Examples:
+    ///
+    /// ```
+    /// use auto_gis_with_rust::geometry::GeometryCollection;
+    /// use auto_gis_with_rust::point::MultiPoint;
+    ///
+    /// let multi_point = MultiPoint::from(vec![[0.0, 0.0], [1.0, 0.0]]);
+    /// let points = multi_point.number_of_geometries();
+    ///
+    /// assert_eq!(points, 2);
+    /// ```
+    fn number_of_geometries(&self) -> usize {
+        self.len()
+    }
+
+    /// Returns the Nth `Point` in this `MultiPoint` collection.
+    ///
+    /// # Examples:
+    ///
+    /// ```
+    /// use auto_gis_with_rust::geometry::GeometryCollection;
+    /// use auto_gis_with_rust::point::MultiPoint;
+    ///
+    /// let multi_point = MultiPoint::from(vec![[0.0, 0.0], [1.0, 0.0]]);
+    /// let point_0 = multi_point.geometry_n(0);
+    ///
+    /// assert_eq!("POINT (0 0)", point_0.to_string());
+    /// ```
+    fn geometry_n(&self, number: usize) -> Point {
+        self[number]
+    }
+}
```

Oh no, it won't compile:

```sh
error[E0507]: cannot move out of index of `Vec<Point>`
   --> src/point.rs:197:9
    |
197 |         self[number]
    |         ^^^^^^^^^^^^ move occurs because value has type `Point`, which does not implement the `Copy` trait
```

But, as is often the case, the compiler has told us exactly what to do to fix this. Let's derive `Copy` for `Point`.

```diff-rust
// point.rs

// ...

use crate::implement_deref;

-#[derive(Debug, PartialEq, PartialOrd)]
+#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point([f64; 2]);

// ...

```

And just like that:

```sh
running 1 test
test src/point.rs - point::MultiPoint::geometry_n (line 187) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 19 filtered out; finished in 0.49s
```

```diff-rust
// point.rs

// ...

    fn geometry_n(&self, number: usize) -> Point {
        self[number]
    }
}
+
+impl Geometry for MultiPoint {
+    /// Compute the geometric center of a geometry.
+    ///
+    /// For a `MultiPoint`, this is a new `Point` with the mean x and y coordinates of all the points in the collection.
+    ///
+    /// ```
+    /// use auto_gis_with_rust::geometry::Geometry;
+    /// use auto_gis_with_rust::point::MultiPoint;
+    ///
+    /// let multi_point = MultiPoint::from(vec![[0., 0.], [1., 0.]]);
+    ///
+    /// assert_eq!(multi_point.centroid().to_string(), "POINT (0.5 0)");
+    /// ```
+    fn centroid(&self) -> Point {
+        let points = self.number_of_geometries() as f64;
+        let sum_x: f64 = self.iter().map(|point| point.x()).sum();
+        let sum_y: f64 = self.iter().map(|point| point.y()).sum();
+        Point::new(sum_x / points, sum_y / points)
+    }
+}
```


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
