use std::ops;
use float_eq::feq;
use point::Point;

///MBR
#[derive(Copy, Clone, Debug)]
pub struct MBR {
    pub minx: f64,
    pub miny: f64,
    pub maxx: f64,
    pub maxy: f64,
}


#[allow(dead_code)]
impl MBR {
    ///construct new MBR
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> MBR {
        MBR {
            minx: x1.min(x2),
            miny: y1.min(y2),
            maxx: x1.max(x2),
            maxy: y1.max(y2),
        }
    }

    pub fn new_raw(minx: f64, miny: f64, maxx: f64, maxy: f64) -> MBR {
        MBR { minx, miny, maxx, maxy }
    }

    pub fn new_default() -> MBR {
        MBR { minx: 0.0, miny: 0.0, maxx: 0.0, maxy: 0.0 }
    }

    pub fn new_from_pt(a: Point) -> MBR {
        MBR { minx: a.x, miny: a.y, maxx: a.x, maxy: a.y }
    }
    ///New bbox from lower left point(a) and upper right point(b)
    pub fn new_from_bounds(a: Point, b: Point) -> MBR {
        MBR::new(a.x, a.y, b.x, b.y)
    }

    ///bounding box.
    #[inline]
    pub fn bbox(&self) -> &Self { self }

    ///bounding box.
    #[inline]
    pub fn clone(&self) -> Self { *self }

    ///Width of bounding box.
    #[inline]
    pub fn width(&self) -> f64 {
        self.maxx - self.minx
    }

    ///Height of bounding box.
    #[inline]
    pub fn height(&self) -> f64 {
        self.maxy - self.miny
    }


    ///Computes area of bounding box.
    #[inline]
    pub fn area(&self) -> f64 {
        self.height() * self.width()
    }

    ///Bounding box as a closed polygon array.
    pub fn as_poly_array(&self) -> Vec<Point> {
        vec![
            Point { x: self.minx, y: self.miny },
            Point { x: self.minx, y: self.maxy },
            Point { x: self.maxx, y: self.maxy },
            Point { x: self.maxx, y: self.miny },
            Point { x: self.minx, y: self.miny },
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

    ///lower left and upper right as tuple ((minx,miny),(maxx,maxy))
    #[inline]
    pub fn llur(self) -> (Point, Point) {
        (Point { x: self.minx, y: self.miny }, Point { x: self.maxx, y: self.maxy })
    }

    ///Compare equality of two bounding boxes
    #[inline]
    pub fn equals(&self, other: &Self) -> bool {
        feq(self.maxx, other.maxx) && feq(self.maxy, other.maxy) &&
            feq(self.minx, other.minx) && feq(self.miny, other.miny)
    }

    ///Checks if bounding box can be represented as a point, width and height as 0.
    #[inline]
    pub fn is_point(&self) -> bool {
        feq(self.height(), 0.0) && feq(self.width(), 0.0)
    }

    ///Contains bonding box
    ///is true if mbr completely contains other, boundaries may touch
    #[inline]
    pub fn contains(&self, other: &Self) -> bool {
        (other.minx >= self.minx) &&
            (other.miny >= self.miny) &&
            (other.maxx <= self.maxx) &&
            (other.maxy <= self.maxy)
    }
    ///contains x, y
    #[inline]
    pub fn contains_xy(&self, x: f64, y: f64) -> bool {
        (x >= self.minx) &&
            (x <= self.maxx) &&
            (y >= self.miny) &&
            (y <= self.maxy)
    }

    ///Completely contains bonding box
    ///is true if mbr completely contains other without touching boundaries
    #[inline]
    pub fn completely_contains(&self, other: &Self) -> bool {
        (other.minx > self.minx) &&
            (other.miny > self.miny) &&
            (other.maxx < self.maxx) &&
            (other.maxy < self.maxy)
    }

    ///completely_contains_xy is true if mbr completely contains location with {x, y}
    ///without touching boundaries
    #[inline]
    pub fn completely_contains_xy(&self, x: f64, y: f64) -> bool {
        (x > self.minx) &&
            (x < self.maxx) &&
            (y > self.miny) &&
            (y < self.maxy)
    }


    ///Translate bounding box by change in dx and dy.
    pub fn translate(&self, dx: f64, dy: f64) -> MBR {
        MBR::new(self.minx + dx, self.miny + dy, self.maxx + dx, self.maxy + dy)
    }

    ///Computes the center of minimum bounding box - (x, y)
    #[inline]
    fn center(&self) -> Point {
        Point { x: (self.minx + self.maxx) / 2.0, y: (self.miny + self.maxy) / 2.0 }
    }


    ///Checks if bounding box intersects other
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        //not disjoint
        !(other.minx > self.maxx || other.maxx < self.minx ||
            other.miny > self.maxy || other.maxy < self.miny)
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

        if self.minx > maxq || self.maxx < minq {
            return false;
        }

        let minq = pt1.y.min(pt2.y);
        let maxq = pt1.y.max(pt2.y);

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
    fn distance_dxdy(&self, other: &Self) -> (f64, f64) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        // find closest edge by x
        if self.maxx < other.minx {
            dx = other.minx - self.maxx
        } else if self.minx > other.maxx {
            dx = self.minx - other.maxx
        }

        // find closest edge by y
        if self.maxy < other.miny {
            dy = other.miny - self.maxy
        } else if self.miny > other.maxy {
            dy = self.miny - other.maxy
        }

        return (dx, dy);
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


    pub fn wkt(&self) -> String {
        format!("POLYGON (({lx} {ly},{lx} {uy},{ux} {uy},{ux} {ly},{lx} {ly}))",
                lx = self.minx, ly = self.miny, ux = self.maxx, uy = self.maxy)
    }
}

impl Eq for MBR {}

impl PartialEq for MBR {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

impl<'a, 'b> ops::BitAnd<&'b MBR> for &'a MBR {
    type Output = Option<MBR>;
    fn bitand(self, rhs: &'b MBR) -> Self::Output {
        self.intersection(rhs)
    }
}

impl<'a, 'b> ops::BitOr<&'b MBR> for &'a MBR {
    type Output = MBR;
    fn bitor(self, rhs: &'b MBR) -> Self::Output {
        self + rhs
    }
}

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

//Todo: complete test with coverage
#[cfg(test)]
mod mbr_tests {
    use super::MBR;

    #[test]
    fn test_mbr() {
        let m0 = MBR::new(0.0, 0.0, 0.5, 0.2);
        let m1 = MBR::new(2.0, 2.0, -0.5, -0.2);
        let m = &m0 + &m1;
        println!("{:?}", m);
        assert_eq!(m.minx, -0.5);
        assert_eq!(m.minx, -0.5);
        assert_eq!(m.miny, -0.2);
        assert_eq!(m.maxx, 2.0);
        assert_eq!(m.maxy, 2.0);

        let m1 = MBR::new_raw(2.0, 2.0, -0.5, -0.2);
        assert_eq!(m1.minx, 2.0);
        assert_eq!(m1.miny, 2.0);
        assert_eq!(m1.maxx, -0.5);
        assert_eq!(m1.maxy, -0.2);

        let m = MBR::new(2.0, 2.0, 0.5, 0.2);
        assert_eq!(m.minx, 0.5);
        assert_eq!(m.miny, 0.2);
        assert_eq!(m.maxx, 2.0);
        assert_eq!(m.maxy, 2.0);

        assert_eq!(m.height(), 1.8);
        assert_eq!(m.width(), 1.5);
        assert_eq!(m.area(), 1.5 * 1.8);
        assert!(!m.is_point());
        assert_eq!(m.as_tuple(), (0.5, 0.2, 2.0, 2.0));

        let b = m.as_poly_array();
        assert_eq!(b.len(), 5);
        assert_eq!(b[0].x, b[4].x);
        assert_eq!(b[0].y, b[4].y);


        let m1 = m.clone();
        assert_eq!(m1.area(), m.area());
        assert_ne!(0.1 + 0.2, 0.3);
        assert!(m1.equals(&m));

        println!("{}", m.wkt());
    }
}
