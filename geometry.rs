use crate::point::Point;

pub trait Geometry {
    fn centroid(&self) -> Point;

    fn wkt(&self) -> String;
}
