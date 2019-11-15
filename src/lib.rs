use math_util::{feq, num, NumCast};
use rstar::{PointDistance, RTreeObject, AABB};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
use std::ops;

use coordinate::Coordinate;
use geom_2d::Point;
use std::ops::Index;

///MBR
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MBR {
    pub ll: Point,
    pub ur: Point,
}

impl MBR {
    ///New MBR given ll & ur
    pub fn new(ll: Point, ur: Point) -> MBR {
        MBR {
            ll: ll.min_of_bounds(&ur),
            ur: ll.max_of_bounds(&ur),
        }
    }

    ///New MBR as given ll & ur
    pub fn new_raw(ll: Point, ur: Point) -> MBR {
        MBR { ll, ur }
    }

    ///New MBR from zero value
    pub fn new_default() -> MBR {
        MBR::new_raw(Point::new_origin(), Point::new_origin())
    }

    ///New MBR from array of 4 coordinates [x1, y1, x2, y2]
    pub fn new_from_array(o: [f64; 4]) -> MBR {
        MBR::new(o[0..2].into(), o[2..4].into())
    }

    ///New MBR from point
    pub fn new_from_pt(a: Point) -> MBR {
        MBR::new_raw(a, a)
    }

    ///Bounding box.
    #[inline]
    pub fn bbox(&self) -> &Self {
        self
    }

    ///Bounding box.
    #[inline]
    pub fn copy(&self) -> Self {
        *self
    }

    ///Width of bounding box.
    #[inline]
    pub fn width(&self) -> f64 {
        self.ur.x - self.ll.x
    }

    ///Height of bounding box.
    #[inline]
    pub fn height(&self) -> f64 {
        self.ur.y - self.ll.y
    }

    ///Computes area of bounding box.
    #[inline]
    pub fn area(&self) -> f64 {
        self.height() * self.width()
    }

    ///Bounding box as a closed polygon array.
    pub fn as_poly_array(&self) -> Vec<Point> {
        let (minx, miny) = self.ll.as_tuple();
        let (maxx, maxy) = self.ur.as_tuple();
        vec![
            Point { x: minx, y: miny },
            Point { x: minx, y: maxy },
            Point { x: maxx, y: maxy },
            Point { x: maxx, y: miny },
            Point { x: minx, y: miny },
        ]
    }

    ///Lower left and upper right corners as an array [minx,miny, maxx,maxy]
    pub fn as_array(&self) -> [f64; 4] {
        [self.ll.x, self.ll.y, self.ur.x, self.ur.y]
    }

    ///Lower left and upper right corners as a tuple (minx,miny, maxx,maxy)
    pub fn as_tuple(&self) -> (f64, f64, f64, f64) {
        (self.ll.x, self.ll.y, self.ur.x, self.ur.y)
    }

    ///lower left and upper right as tuple [Point(minx,miny),Point(maxx,maxy)]
    #[inline]
    pub fn llur(self) -> [Point; 2] {
        [self.ll, self.ur]
    }

    ///Compare equality of two bounding boxes
    #[inline]
    pub fn equals(&self, other: &Self) -> bool {
        self.ll.equals(&other.ll) && self.ur.equals(&other.ur)
    }

    ///Checks if bounding box can be represented as a point, width and height as 0.
    #[inline]
    pub fn is_point(&self) -> bool {
        self.centre().equals(&self.ll)
    }

    ///Contains bonding box
    ///is true if mbr completely contains other, boundaries may touch
    #[inline]
    pub fn contains(&self, other: &Self) -> bool {
        (other.ll.x >= self.ll.x)
            && (other.ll.y >= self.ll.y)
            && (other.ur.x <= self.ur.x)
            && (other.ur.y <= self.ur.y)
    }
    ///contains x, y
    #[inline]
    pub fn contains_xy(&self, x: f64, y: f64) -> bool {
        (x >= self.ll.x) && (x <= self.ur.x) && (y >= self.ll.y) && (y <= self.ur.y)
    }

    ///Completely contains bonding box
    ///is true if mbr completely contains other without touching boundaries
    #[inline]
    pub fn completely_contains(&self, other: &Self) -> bool {
        (other.ll.x > self.ll.x)
            && (other.ll.y > self.ll.y)
            && (other.ur.x < self.ur.x)
            && (other.ur.y < self.ur.y)
    }

    ///completely_contains_xy is true if mbr completely contains location with {x, y}
    ///without touching boundaries
    #[inline]
    pub fn completely_contains_xy(&self, x: f64, y: f64) -> bool {
        (x > self.ll.x) && (x < self.ur.x) && (y > self.ll.y) && (y < self.ur.y)
    }

    ///Translate bounding box by change in dx and dy.
    pub fn translate(&self, dx: f64, dy: f64) -> MBR {
        let delta = Point { x: dx, y: dy };
        MBR::new_raw(self.ll.add(&delta), self.ur.add(&delta))
    }

    ///Computes the center of minimum bounding box - (x, y)
    #[inline]
    pub fn centre(&self) -> Point {
        Point {
            x: (self.ll.x + self.ur.x) / 2.0,
            y: (self.ll.y + self.ur.y) / 2.0,
        }
    }

    ///Checks if bounding box intersects other
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        //not disjoint
        !(other.ll.x > self.ur.x
            || other.ur.x < self.ll.x
            || other.ll.y > self.ur.y
            || other.ur.y < self.ll.y)
    }

    ///intersects point
    #[inline]
    pub fn intersects_point(&self, pt: &Point) -> bool {
        self.contains_xy(pt.x, pt.y)
    }

    ///intersects point with x, y
    #[inline]
    pub fn intersects_xy(&self, x: f64, y: f64) -> bool {
        self.contains_xy(x, y)
    }

    /// Intersects bounds
    pub fn intersects_bounds(&self, pt1: &Point, pt2: &Point) -> bool {
        let minq = pt1.x.min(pt2.x);
        let maxq = pt1.x.max(pt2.x);

        if self.ll.x > maxq || self.ur.x < minq {
            return false;
        }

        let minq = pt1.y.min(pt2.y);
        let maxq = pt1.y.max(pt2.y);

        // not disjoint
        !(self.ll.y > maxq || self.ur.y < minq)
    }

    ///Test for disjoint between two mbrs
    #[inline]
    pub fn disjoint(&self, m: &Self) -> bool {
        !self.intersects(m)
    }

    ///Computes the intersection of two bounding box
    pub fn intersection(&self, other: &Self) -> Option<MBR> {
        if !self.intersects(other) {
            return None;
        }
        let minx = if self.ll.x > other.ll.x {
            self.ll.x
        } else {
            other.ll.x
        };
        let miny = if self.ll.y > other.ll.y {
            self.ll.y
        } else {
            other.ll.y
        };
        let maxx = if self.ur.x < other.ur.x {
            self.ur.x
        } else {
            other.ur.x
        };
        let maxy = if self.ur.y < other.ur.y {
            self.ur.y
        } else {
            other.ur.y
        };

        Some(MBR::new_raw(
            Point { x: minx, y: miny },
            Point { x: maxx, y: maxy },
        ))
    }

    ///Expand include other bounding box
    pub fn expand_to_include(&mut self, other: &Self) -> &mut MBR {
        self.ll.x = other.ll.x.min(self.ll.x);
        self.ll.y = other.ll.y.min(self.ll.y);

        self.ur.x = other.ur.x.max(self.ur.x);
        self.ur.y = other.ur.y.max(self.ur.y);
        self
    }

    ///Expand to include x,y
    pub fn expand_to_include_xy(&mut self, x: f64, y: f64) -> &mut Self {
        if x < self.ll.x {
            self.ll.x = x
        } else if x > self.ur.x {
            self.ur.x = x
        }

        if y < self.ll.y {
            self.ll.y = y
        } else if y > self.ur.y {
            self.ur.y = y
        }
        self
    }

    ///Expand by delta in x and y
    pub fn expand_by_delta(&mut self, dx: f64, dy: f64) -> &mut MBR {
        let (minx, miny) = (self.ll.x - dx, self.ll.y - dy);
        let (maxx, maxy) = (self.ur.x + dx, self.ur.y + dy);

        self.ll.x = minx.min(maxx);
        self.ll.y = miny.min(maxy);
        self.ur.x = minx.max(maxx);
        self.ur.y = miny.max(maxy);

        self
    }

    ///computes dx and dy for computing hypot
    pub fn distance_dxdy(&self, other: &Self) -> (f64, f64) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        // find closest edge by x
        if self.ur.x < other.ll.x {
            dx = other.ll.x - self.ur.x
        } else if self.ll.x > other.ur.x {
            dx = self.ll.x - other.ur.x
        }

        // find closest edge by y
        if self.ur.y < other.ll.y {
            dy = other.ll.y - self.ur.y
        } else if self.ll.y > other.ur.y {
            dy = self.ll.y - other.ur.y
        }

        (dx, dy)
    }

    ///distance computes the distance between two mbrs
    pub fn distance(&self, other: &Self) -> f64 {
        if self.intersects(other) {
            return 0.0;
        }

        let (dx, dy) = self.distance_dxdy(other);
        dx.hypot(dy)
    }

    ///distance square computes the squared distance
    ///between bounding boxes
    pub fn distance_square(&self, other: &Self) -> f64 {
        if self.intersects(other) {
            return 0.0;
        }
        let (dx, dy) = self.distance_dxdy(other);
        (dx * dx) + (dy * dy)
    }

    ///WKT string
    pub fn wkt(&self) -> String {
        format!(
            "POLYGON (({lx} {ly},{lx} {uy},{ux} {uy},{ux} {ly},{lx} {ly}))",
            lx = self.ll.x,
            ly = self.ll.y,
            ux = self.ur.x,
            uy = self.ur.y
        )
    }
}

pub struct Boxes {
    pub boxes: Vec<MBR>
}


impl<T> From<(T, T, T, T)> for MBR
    where
        T: NumCast + Copy,
{
    fn from(tup: (T, T, T, T)) -> Self {
        MBR::new(
            Point {
                x: num::cast(tup.0).unwrap(),
                y: num::cast(tup.1).unwrap(),
            },
            Point {
                x: num::cast(tup.2).unwrap(),
                y: num::cast(tup.3).unwrap(),
            },
        )
    }
}

impl<T> From<(T, T)> for MBR
    where
        T: NumCast + Copy,
{
    fn from(tup: (T, T)) -> Self {
        let p = Point {
            x: num::cast(tup.0).unwrap(),
            y: num::cast(tup.1).unwrap(),
        };
        MBR { ll: p, ur: p }
    }
}

impl<T> From<[T; 4]> for MBR
    where
        T: NumCast + Copy,
{
    fn from(array: [T; 4]) -> Self {
        MBR::new(
            Point {
                x: num::cast(array[0]).unwrap(),
                y: num::cast(array[1]).unwrap(),
            },
            Point {
                x: num::cast(array[2]).unwrap(),
                y: num::cast(array[3]).unwrap(),
            },
        )
    }
}

impl<T> From<[T; 2]> for MBR
    where
        T: NumCast + Copy,
{
    fn from(array: [T; 2]) -> Self {
        let p = Point {
            x: num::cast(array[0]).unwrap(),
            y: num::cast(array[1]).unwrap(),
        };
        MBR { ll: p, ur: p }
    }
}


impl<T> From<Vec<[T; 4]>> for Boxes
    where
        T: NumCast + Copy,
{
    fn from(items: Vec<[T; 4]>) -> Self {
        let mut boxes = vec![];
        for array in items {
            boxes.push(array.into())
        }
        Boxes { boxes }
    }
}


impl From<Point> for MBR {
    fn from(pt: Point) -> Self {
        MBR::new_from_pt(pt)
    }
}

impl From<AABB<Point>> for MBR {
    fn from(aab: AABB<Point>) -> Self {
        MBR::new_raw(aab.lower(), aab.upper())
    }
}

impl From<AABB<[f64; 2]>> for MBR {
    fn from(aab: AABB<[f64; 2]>) -> Self {
        MBR::new_raw(aab.lower().into(), aab.upper().into())
    }
}

impl Index<usize> for Boxes {
    type Output = MBR ;
    fn index(&self, i: usize) -> &Self::Output {
        &self.boxes[i]
    }
}

///Bounding Box Trait
pub trait BBox {
    fn bbox(&self) -> &MBR;
}

///Eq for MBR
impl Eq for MBR {}

///PartialEq for MBR
impl PartialEq for MBR {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

///Ord for MBR
impl Ord for MBR {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut d = self.ll.x - other.ll.x;
        if feq(d, 0.0) {
            d = self.ll.y - other.ll.y;
        }
        if feq(d, 0.0) {
            Ordering::Equal
        } else if d < 0.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

///PartialOrd for MBR
impl PartialOrd for MBR {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

///Display for MBR
impl Display for MBR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.wkt())
    }
}

///ops::BitAnd for MBR
impl<'a, 'b> ops::BitAnd<&'b MBR> for &'a MBR {
    type Output = Option<MBR>;
    fn bitand(self, rhs: &'b MBR) -> Self::Output {
        self.intersection(rhs)
    }
}

///ops::BitOr for MBR
impl<'a, 'b> ops::BitOr<&'b MBR> for &'a MBR {
    type Output = MBR;
    fn bitor(self, rhs: &'b MBR) -> Self::Output {
        self + rhs
    }
}

///ops::Add for MBR
impl<'a, 'b> ops::Add<&'b MBR> for &'a MBR {
    type Output = MBR;
    fn add(self, rhs: &'b MBR) -> MBR {
        MBR {
            ll: self.ll.min_of_bounds(&rhs.ll),
            ur: self.ur.max_of_bounds(&rhs.ur),
        }
    }
}

///RTreeObject for MBR
impl RTreeObject for MBR {
    type Envelope = AABB<Point>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.ll, self.ur)
    }
}

///PointDistance for MBR
impl PointDistance for MBR {
    fn distance_2(&self, pt: &Point) -> f64 {
        self.distance_square(&MBR::new_raw(*pt, *pt))
    }
}

///BBox for MBR
impl BBox for MBR {
    fn bbox(&self) -> &MBR {
        return self.bbox();
    }
}

#[cfg(test)]
mod mbr_tests;
