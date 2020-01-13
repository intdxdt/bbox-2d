use math_util::{feq, num, NumCast};
use rstar::{PointDistance, RTreeObject, AABB, Point as RPt};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
use std::ops;
use std::ops::Index;
use point::Point;

///MBR
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct MBR {
    pub minx: f64,
    pub miny: f64,
    pub maxx: f64,
    pub maxy: f64,
}

impl MBR {
    ///New MBR given ll (x1, y1) & ur(x2, y2)
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> MBR {
        MBR {
            minx: x1.min(x2),
            miny: y1.min(y2),
            maxx: x1.max(x2),
            maxy: y1.max(y2),
        }
    }

    ///New MBR given ll (x1, y1) & ur(x2, y2)
    pub fn new_raw(minx: f64, miny: f64, maxx: f64, maxy: f64) -> MBR {
        MBR { minx, miny, maxx, maxy }
    }

    ///New MBR from zero value
    pub fn new_default() -> MBR {
        MBR { minx: 0.0, miny: 0.0, maxx: 0.0, maxy: 0.0 }
    }

    ///New MBR from array of 4 coordinates [x1, y1, x2, y2]
    pub fn new_from_array(o: [f64; 4]) -> MBR { o.into() }

    ///New MBR from point
    pub fn new_from_pt(pt: [f64; 2]) -> MBR { pt.into() }

    ///New MBR from bounds ll (x1, y1) & ur(x2, y2)
    pub fn new_from_bounds(ll: [f64; 2], ur: [f64; 2]) -> MBR {
        MBR::new(ll[0], ll[1], ur[0], ur[1])
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
    pub fn width(&self) -> f64 { self.maxx - self.minx }

    ///Height of bounding box.
    #[inline]
    pub fn height(&self) -> f64 { self.maxy - self.miny }

    ///Computes area of bounding box.
    #[inline]
    pub fn area(&self) -> f64 {
        self.height() * self.width()
    }

    ///Bounding box as a closed polygon array.
    pub fn as_poly_array(&self) -> Vec<[f64; 2]> {
        vec![
            [self.minx, self.miny],
            [self.minx, self.maxy],
            [self.maxx, self.maxy],
            [self.maxx, self.miny],
            [self.minx, self.miny],
        ]
    }

    ///Lower left and upper right corners as an array [minx,miny, maxx,maxy]
    pub fn as_array(&self) -> [f64; 4] {
        [self.minx, self.miny, self.maxx, self.maxy]
    }

    ///Lower left and upper right corners as a tuple (minx,miny, maxx,maxy)
    pub fn as_tuple(&self) -> (f64, f64, f64, f64) {
        (self.minx, self.miny, self.maxx, self.maxy)
    }

    ///lower left and upper right as tuple [Point(minx,miny),Point(maxx,maxy)]
    #[inline]
    pub fn llur(self) -> [[f64; 2]; 2] {
        [self.ll(), self.ur()]
    }

    ///lower left - Point(minx,miny)
    #[inline]
    pub fn ll(self) -> [f64; 2] {
        [self.minx, self.miny]
    }

    ///upper right - Point(maxx,maxy)
    #[inline]
    pub fn ur(self) -> [f64; 2] {
        [self.maxx, self.maxy]
    }
    ///Compare equality of two bounding boxes
    #[inline]
    pub fn equals(&self, other: &Self) -> bool {
        feq(self.maxx, other.maxx)
            && feq(self.maxy, other.maxy)
            && feq(self.minx, other.minx)
            && feq(self.miny, other.miny)
    }

    ///Checks if bounding box can be represented as a point, width and height as 0.
    #[inline]
    pub fn is_point(&self) -> bool {
        let c = self.centre();
        feq(self.minx, c[0]) && feq(self.miny, c[1])
    }

    ///Contains bonding box
    ///is true if mbr completely contains other, boundaries may touch
    #[inline]
    pub fn contains(&self, other: &Self) -> bool {
        (other.minx >= self.minx)
            && (other.miny >= self.miny)
            && (other.maxx <= self.maxx)
            && (other.maxy <= self.maxy)
    }

    ///contains x, y
    #[inline]
    pub fn contains_xy(&self, x: f64, y: f64) -> bool {
        (x >= self.minx) && (x <= self.maxx) && (y >= self.miny) && (y <= self.maxy)
    }

    ///contains point
    #[inline]
    pub fn contains_point(&self, pt: [f64; 2]) -> bool {
        self.contains_xy(pt[0], pt[1])
    }

    ///Completely contains bonding box
    ///is true if mbr completely contains other without touching boundaries
    #[inline]
    pub fn completely_contains(&self, other: &Self) -> bool {
        (other.minx > self.minx)
            && (other.miny > self.miny)
            && (other.maxx < self.maxx)
            && (other.maxy < self.maxy)
    }

    ///completely_contains_xy is true if mbr completely contains location with {x, y}
    ///without touching boundaries
    #[inline]
    pub fn completely_contains_xy(&self, x: f64, y: f64) -> bool {
        (x > self.minx) && (x < self.maxx) && (y > self.miny) && (y < self.maxy)
    }

    ///completely_contains_point is true if mbr completely contains location with point{x, y}
    ///without touching boundaries
    #[inline]
    pub fn completely_contains_point(&self, pt: [f64; 2]) -> bool {
        self.completely_contains_xy(pt[0], pt[1])
    }

    ///Translate bounding box by change in dx and dy.
    pub fn translate(&self, dx: f64, dy: f64) -> MBR {
        MBR::new_raw(self.minx + dx, self.miny + dy, self.maxx + dx, self.maxy + dy)
    }

    ///Computes the center of minimum bounding box - (x, y)
    #[inline]
    pub fn centre(&self) -> [f64; 2] {
        [(self.minx + self.maxx) / 2.0, (self.miny + self.maxy) / 2.0]
    }

    ///Checks if bounding box intersects other
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        //not disjoint
        !(other.minx > self.maxx
            || other.maxx < self.minx
            || other.miny > self.maxy
            || other.maxy < self.miny)
    }

    ///intersects point
    #[inline]
    pub fn intersects_point(&self, pt: &[f64]) -> bool {
        self.intersects_xy(pt[0], pt[1])
    }

    ///intersects point with x, y
    #[inline]
    pub fn intersects_xy(&self, x: f64, y: f64) -> bool {
        self.contains_xy(x, y)
    }

    /// Intersects bounds
    pub fn intersects_bounds(&self, pt1: &[f64], pt2: &[f64]) -> bool {
        let minq = pt1[0].min(pt2[0]);
        let maxq = pt1[0].max(pt2[0]);

        if self.minx > maxq || self.maxx < minq {
            return false;
        }

        let minq = pt1[1].min(pt2[1]);
        let maxq = pt1[1].max(pt2[1]);

        // not disjoint
        !(self.miny > maxq || self.maxy < minq)
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
        let minx = if self.minx > other.minx { self.minx } else { other.minx };
        let miny = if self.miny > other.miny { self.miny } else { other.miny };
        let maxx = if self.maxx < other.maxx { self.maxx } else { other.maxx };
        let maxy = if self.maxy < other.maxy { self.maxy } else { other.maxy };

        Some(MBR { minx, miny, maxx, maxy })
    }

    ///Expand include other bounding box
    pub fn expand_to_include(&mut self, other: &Self) -> &mut MBR {
        self.minx = other.minx.min(self.minx);
        self.miny = other.miny.min(self.miny);

        self.maxx = other.maxx.max(self.maxx);
        self.maxy = other.maxy.max(self.maxy);
        self
    }

    ///Expand to include point(x, y)
    pub fn expand_to_include_point(&mut self, pt: [f64; 2]) -> &mut Self {
        self.expand_to_include_xy(pt[0], pt[1])
    }

    ///Expand to include x,y
    pub fn expand_to_include_xy(&mut self, x: f64, y: f64) -> &mut Self {
        if x < self.minx {
            self.minx = x
        } else if x > self.maxx {
            self.maxx = x
        }

        if y < self.miny {
            self.miny = y
        } else if y > self.maxy {
            self.maxy = y
        }
        self
    }

    ///Expand by delta in x and y
    pub fn expand_by_delta(&mut self, dx: f64, dy: f64) -> &mut MBR {
        let (minx, miny) = (self.minx - dx, self.miny - dy);
        let (maxx, maxy) = (self.maxx + dx, self.maxy + dy);

        self.minx = minx.min(maxx);
        self.miny = miny.min(maxy);
        self.maxx = minx.max(maxx);
        self.maxy = miny.max(maxy);

        self
    }

    ///computes dx and dy for computing hypot
    pub fn distance_dxdy(&self, other: &Self) -> (f64, f64) {
        // find closest edge by x
        let dx = if self.maxx < other.minx {
            other.minx - self.maxx
        } else if self.minx > other.maxx {
            self.minx - other.maxx
        } else { 0.0 };

        // find closest edge by y
        let dy = if self.maxy < other.miny {
            other.miny - self.maxy
        } else if self.miny > other.maxy {
            self.miny - other.maxy
        } else { 0.0 };

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
            "POLYGON(({lx} {ly},{lx} {uy},{ux} {uy},{ux} {ly},{lx} {ly}))",
            lx = self.minx,
            ly = self.miny,
            ux = self.maxx,
            uy = self.maxy
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
            num::cast(tup.0).unwrap(),
            num::cast(tup.1).unwrap(),
            num::cast(tup.2).unwrap(),
            num::cast(tup.3).unwrap(),
        )
    }
}

impl<T> From<(T, T)> for MBR
    where
        T: NumCast + Copy,
{
    fn from(tup: (T, T)) -> Self {
        let x: f64 = num::cast(tup.0).unwrap();
        let y: f64 = num::cast(tup.1).unwrap();
        MBR { minx: x, miny: y, maxx: x, maxy: y }
    }
}

impl<T> From<[T; 4]> for MBR
    where
        T: NumCast + Copy,
{
    fn from(array: [T; 4]) -> Self {
        MBR::new(
            num::cast(array[0]).unwrap(),
            num::cast(array[1]).unwrap(),
            num::cast(array[2]).unwrap(),
            num::cast(array[3]).unwrap(),
        )
    }
}

impl<T> From<[T; 2]> for MBR
    where
        T: NumCast + Copy,
{
    fn from(array: [T; 2]) -> Self {
        let x: f64 = num::cast(array[0]).unwrap();
        let y: f64 = num::cast(array[1]).unwrap();
        MBR { minx: x, miny: y, maxx: x, maxy: y }
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

impl From<AABB<[f64; 2]>> for MBR {
    fn from(aabb: AABB<[f64; 2]>) -> Self {
        MBR::new_from_bounds(aabb.lower(), aabb.upper())
    }
}

impl Index<usize> for Boxes {
    type Output = MBR;
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
        let mut d = self.minx - other.minx;
        if feq(d, 0.0) {
            d = self.miny - other.miny;
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
            minx: self.minx.min(rhs.minx),
            miny: self.miny.min(rhs.miny),
            maxx: self.maxx.max(rhs.maxx),
            maxy: self.maxy.max(rhs.maxy),
        }
    }
}

///RTreeObject for MBR
impl RTreeObject for MBR {
    type Envelope = AABB<Point>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(self.ll().into(), self.ur().into())
    }
}

///PointDistance for MBR
impl PointDistance for MBR {
    fn distance_2(&self, pt: &Point) -> f64 {
        self.distance_square(&MBR::new_from_pt(pt.as_array()))
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
