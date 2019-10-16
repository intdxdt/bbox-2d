use std::ops;
use point::Point;
use std::fmt::{Display, Formatter, Error};
use std::cmp::Ordering;
use math_util::feq;

///MBR
#[derive(Copy, Clone, Debug)]
pub struct MBR {
    pub minx: f64,
    pub miny: f64,
    pub maxx: f64,
    pub maxy: f64,
}

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
        let c = self.centre();
        self.minx == c.x && self.miny == c.y
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
    fn centre(&self) -> Point {
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
}

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

impl PartialOrd for MBR {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for MBR {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.wkt())
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
    use super::*;
    use math_util::round;


    #[test]
    fn test_construction() {
        let m0 = MBR::new(0.0, 0.0, 0.5, 0.2);
        let m1 = MBR::new(2.0, 2.0, -0.5, -0.2);
        let m = &m0 + &m1;
        assert_eq!(m, MBR::new(-0.5, -0.2, 2.0, 2.0));

        let m1 = MBR::new_raw(2.0, 2.0, -0.5, -0.2);
        assert_eq!(m1, MBR { minx: 2.0, miny: 2.0, maxx: -0.5, maxy: -0.2 });

        let m = MBR::new(2.0, 2.0, 0.5, 0.2);
        assert_eq!(m, (MBR { minx: 0.5, miny: 0.2, maxx: 2.0, maxy: 2.0 }));

        assert_eq!((m.width(), m.height(), m.area(), m.is_point()), (1.5, 1.8, 1.5 * 1.8, false));
        assert_eq!(m.as_tuple(), (0.5, 0.2, 2.0, 2.0));
        assert_eq!(m.as_array(), [0.5, 0.2, 2.0, 2.0]);

        let b = m.as_poly_array();
        assert_eq!((b[0], b[4], b.len()), (Point { x: 0.5, y: 0.2 }, Point { x: 0.5, y: 0.2 }, 5));

        let m1 = m.clone();
        assert_eq!((m1.equals(&m), m1.area()), (m1 == m, m.area()));
    }

    #[test]
    fn test_methods() {
        let m = MBR::new(2.0, 2.0, 0.5, 0.2);
        assert_eq!((m.width(), m.height(), m.area(), m.is_point()), (1.5, 1.8, 1.5 * 1.8, false));
        assert_eq!((0.5, 0.2, 2.0, 2.0), m.as_tuple());

        let b = m.as_poly_array();
        let b0 = Point::new(0.5, 0.2);
        let b1 = Point::new(0.5, 2.);
        let b2 = Point::new(2., 2.);
        let b3 = Point::new(2., 0.2);
        let b4 = Point::new(0.5, 0.2);

        assert_eq!(
            (b.len(), b[0], b[1], b[2], b[3], b[4], b[0]), (5, b0, b1, b2, b3, b4, b4)
        );

        let mut m1 = m;
        let m2 = m;
        assert_eq!(m1.area(), m.area());
        assert!(m1.equals(&m));
        assert_eq!(m1, m);

        assert_eq!(m2.area(), m.area());
        assert!(m2.equals(&m));
        assert_eq!(m2, m);
        m1.minx = -1.;
        assert_ne!(m2, m1);
    }

    #[test]
    fn test_ops_1() {
        let mut m00 = MBR::new_default();
        m00.expand_to_include_xy(2., 2.);

        let mut n00 = MBR::new_default();
        n00.expand_to_include_xy(-2., -2.);

        let mut m0 = MBR::new(1., 1., 1., 1.);
        m0.expand_by_delta(1., 1.);

        let m1 = MBR::new(0., 0., 2., 2.);
        let m2 = MBR::new(4., 5., 8., 9.);
        let m3 = MBR::new(4., 5., 8., 9.);
        let m4 = MBR::new(5., 0., 8., 2.);
        let m5 = MBR::new(5., 11., 8., 9.);
        let m6 = MBR::new(0., 0., 2., -2.);
        let m7 = MBR::new(-2., 1., 4., -2.);
        let mut vects = vec![m1, m2, m4, m5, m6, m7, m3];
        vects.sort();


        let m0123 = MBR::new(0., 2., 1., 3.);
        let clone_m0123 = m0123;


        let r1: [f64; 4] = [0., 0., 2., 2.];
        assert!(m1.as_array() == r1);
        assert_eq!(clone_m0123, m0123);
        assert!(m0.equals(&m1));
        assert_eq!(*m0.bbox(), m0);
        assert!(m00.equals(&m1));
        assert_ne!(m1, m2);


        assert!(m00.intersects(&n00));
        let nm00 = m00.intersection(&n00);
        assert!(nm00.is_some());

        let bln1 = nm00.unwrap().minx == 0.0 && nm00.unwrap().miny == 0.0;
        let bln2 = nm00.unwrap().maxx == 0.0 && nm00.unwrap().maxy == 0.0;
        assert!(bln1);
        assert!(bln2);
        assert!(nm00.unwrap().is_point());

        assert!(!m1.intersects(&m2));
        let null_mbr = m1.intersection(&m2);
        assert!(null_mbr.is_none());

        let _m13 = [1.7, 1.5, 2., 2.];
        let _m23 = [4., 5., 5., 9.];


        assert!(m2.intersects(&m5));
        assert!(m7.intersects(&m6));
        assert!(m6.intersects(&m7));

        let m67 = m6.intersection(&m7);
        let m76 = m7.intersection(&m6);

        if m67.is_some() {
            let v = m67.unwrap();
            assert!(v.area() > 0.0);
        }

        assert!(m67.unwrap().equals(&m6));
        assert!(m67.unwrap().equals(&m76.unwrap()));

        let m25 = m2.intersection(&m5);

        assert_eq!(m25.unwrap().width(), m5.width());
        assert_eq!(m25.unwrap().height(), 0.0);

        let d = 2f64.hypot(3.);
        assert_eq!(m1.distance(&m2), d);
        assert_eq!(m1.distance_square(&m2), round(d * d, 12));
    }

    #[test]
    fn test_ops2() {
        let mut m00 = MBR::new_default();
        m00.expand_to_include_xy(2., 2.);

        let mut n00 = MBR::new_default();
        n00.expand_to_include_xy(-2., -2.);

        let mut m0 = MBR::new(1., 1., 1., 1.);
        m0.expand_by_delta(1., 1.);
        let mut m0_pt = MBR::new_from_pt(Point::new(0., 0.));

        let m1 = MBR::new(0., 0., 2., 2.);
        let m2 = MBR::new(4., 5., 8., 9.);

        let m1_bounds = MBR::new_from_bounds(Point::new(0., 0.), Point::new(2., 2.));
        let m2_bounds = MBR::new_from_bounds(Point::new(4., 5.), Point::new(8., 9.));

        let m3 = MBR::new(1.7, 1.5, 5., 9.);
        let m4 = MBR::new(5., 0., 8., 2.);
        let m5 = MBR::new(5., 11., 8., 9.);
        let m6 = MBR::new(0., 0., 2., -2.);
        let m7 = MBR::new(-2., 1., 4., -2.);
        let m8 = MBR::new(-1., 0., 1., -1.5);
        let m9 = MBR::new(-1., 0., 100., 10.5);
        let mut vects = vec![m9, m1, m2, m3, m4, m5, m6, m7, m8];
        vects.sort();

        let p = Point::new(1.7, 1.5);
        let p0 = Point::new(1.7, 0.0);

        let m0123 = MBR::new(0., 2., 1., 3.);
        let clone_m0123 = m0123;


        //SECTION (Constructs)
        assert_ne!(m0, m0_pt);
        assert_eq!(m0.llur().0, Point::new(0., 0.));
        assert_eq!(m0.llur().1, Point::new(2., 2.));
        m0_pt.expand_to_include_xy(2., 2.);
        assert_eq!(m0, m0_pt);
        assert_eq!(m0.llur(), m0_pt.llur());

        assert_eq!(m1, m1_bounds);
        assert_eq!(m2, m2_bounds);

        //SECTION (Equals)
        let r1 = [0., 0., 2., 2.];
        assert!(m1.as_array() == r1);
        assert_eq!(clone_m0123, m0123);
        assert!(m0.equals(&m1));
        assert_eq!(*m0.bbox(), m0);
        assert!(m00.equals(&m1));
        assert_ne!(m1, m2);

//    SECTION("intersects , distance") 
        assert!(m1.intersects_xy(p.x, p.y));
        assert!(m1.intersects_xy(p0.x, p0.y));
        assert!(m1.intersects_point(&p));
        assert!(m1.intersects_point(&p0));

        assert!(m00.intersects(&n00));
        let nm00 = m00.intersection(&n00);
        assert_ne!(nm00, None);

        let bln1 = nm00.unwrap().minx == 0.0 && nm00.unwrap().miny == 0.0;
        let bln2 = nm00.unwrap().maxx == 0.0 && nm00.unwrap().maxy == 0.0;
        assert!(bln1);
        assert!(bln2);
        assert!(nm00.unwrap().is_point());

        assert!(!m1.intersects(&m2));
        let null_mbr = m1.intersection(&m2);
        assert!(!null_mbr.is_some());
        assert!(m1.intersects(&m3));
        assert!(m2.intersects(&m3));

        let m13 = m1.intersection(&m3);
        let m23 = m2.intersection(&m3);
        let _m13 = [1.7, 1.5, 2., 2.];
        let _m23 = [4., 5., 5., 9.];

        assert!(_m13 == m13.unwrap().as_array());
        assert!(_m23 == m23.unwrap().as_array());

        assert!(m3.intersects(&m4));
        assert!(m2.intersects(&m5));
        assert!(m7.intersects(&m6));
        assert!(m6.intersects(&m7));

        let m67 = &m6 & &m7;
        let m76 = &m7 & &m6;
        let m78 = &m7 & &m8;

        assert!(m67.unwrap().equals(&m6));
        assert!(m67.unwrap().equals(&m76.unwrap()));
        assert!(m78.unwrap().equals(&m8));

        let m25 = m2.intersection(&m5);
        let m34 = m3.intersection(&m4);

        assert_eq!(m25.unwrap().width(), m5.width());
        assert_eq!(m25.unwrap().height(), 0.0);
        assert_eq!(m34.unwrap().width(), 0.0);
        assert_eq!(m34.unwrap().height(), 0.5);
        assert_eq!(m3.distance(&m4), 0.0);

        let d = 2f64.hypot(3.);
        assert_eq!(m1.distance(&m2), d);
        assert_eq!(m1.distance_square(&m2), round(d * d, 12));
        assert_eq!(m1.distance(&m3), 0.0);
        assert_eq!(m1.distance_square(&m3), 0.0);

        let a = MBR::new(-7.703505430214746, 3.0022503796012305, -5.369812194018422, 5.231449888803689);
        assert_eq!(m1.distance(&a), (-5.369812194018422f64).hypot(3.0022503796012305 - 2.));

        let b = MBR::new(-4.742849832055231, -4.1033230559816065, -1.9563504455521576, -2.292098454754609);
        assert_eq!(m1.distance(&b), (-1.9563504455521576f64).hypot(-2.292098454754609));

//    SECTION("contains, disjoint , contains completely") 
        let p1 = Point::new(-5.95, 9.28);
        let p2 = Point::new(-0.11, 12.56);
        let p3 = Point::new(3.58, 11.79);
        let p4 = Point::new(-1.16, 14.71);

        let mp12 = MBR::new(p1.x, p1.y, p2.x, p2.y);
        let mp34 = MBR::new(p3.x, p3.y, p4.x, p4.y);

        // intersects but segment are disjoint
        assert!(mp12.intersects(&mp34));
        assert!(mp12.intersects_bounds(&p3, &p4));
        assert!(!mp12.intersects_bounds(&Point::new(m1.minx, m1.miny), &Point::new(m1.maxx, m1.maxy)));
        assert!(!mp12.intersects_xy(p3.x, p3.y));
        assert!(m1.contains_xy(1., 1.));

        let mbr11 = MBR::new(1., 1., 1.5, 1.5);
        let mbr12 = MBR::new(1., 1., 2., 2.);
        let mbr13 = MBR::new(1., 1., 2.000045, 2.00001);
        let mbr14 = MBR::new(2.000045, 2.00001, 4.000045, 4.00001);

        assert!(m1.contains(&mbr11));
        assert!(m1.contains(&mbr12));
        assert!(!m1.contains(&mbr13));
        assert!(!m1.disjoint(&mbr13));// False
        assert!(m1.disjoint(&mbr14)); // True disjoint

        assert!(m1.contains_xy(1.5, 1.5));
        assert!(m1.contains_xy(2., 2.));

        assert!(m1.completely_contains(&mbr11));
        assert!(m1.completely_contains_xy(1.5, 1.5));
        assert!(m1.completely_contains_xy(1.5, 1.5));
        assert!(!m1.completely_contains_xy(2., 2.));
        assert!(!m1.completely_contains(&mbr12));
        assert!(!m1.completely_contains(&mbr13));


//    SECTION("translate, expand by, area")
        let mut ma = MBR::new(0., 0., 2., 2.);
        let mb = MBR::new(-1., -1., 1.5, 1.9);
        let mc = MBR::new(1.7, 1.5, 5., 9.);
        let mut md = ma;
        let ma_mc_plus = &ma + &mc;
        let ma_mc = &ma | &mc;
        let md_mb = &md | &mb;
        assert_eq!(ma_mc, ma_mc_plus);
        assert!(ma_mc.equals(&ma_mc));
        ma.expand_to_include(&mc);
        md.expand_to_include(&mb);
        assert!(ma.equals(&ma_mc));
        assert!(md.equals(&md_mb));

        let mut arr = [0., 0., 5., 9.];
        let polyarr = vec!(
            Point::new(0., 0.), Point::new(0., 9.),
            Point::new(5., 9.), Point::new(5., 0.), Point::new(0., 0.));
        assert!(ma.as_array() == arr);  //ma modified by expand
        let mut i = 0usize;
        for &o in ma.as_poly_array().iter() {
            assert_eq!(o, polyarr[i]); //ma modified by expand
            i += 1;
        }

        arr = [1.7, 1.5, 5., 9.];
        assert!(mc.as_array() == arr); //should not be touched
        arr = [-1., -1., 2., 2.];
        assert!(md.as_array() == arr);//ma modified by expand

        //mc area
        assert_eq!(mc.area(), 24.75);

        let mt = m1.translate(1., 1.);
        let mut mby = m1;
        mby.expand_by_delta(-3., -3.);

        let m1c = m1.centre();
        let mtc = mt.centre();

        let pt = Point::new(1., 1.);
        assert_eq!(m1c, pt);
        let pt = Point::new(2., 2.);
        assert_eq!(mtc, pt);
        arr = [1., 1., 3., 3.];
        assert!(mt.as_array() == arr);
        arr = [-1., -1., 3., 3.];
        assert!(mby.as_array() == arr);


//    SECTION("wkt string")
        assert_eq!(m1.wkt(), "POLYGON ((0 0,0 2,2 2,2 0,0 0))".to_string());
        assert_eq!(format!("{}", m1), "POLYGON ((0 0,0 2,2 2,2 0,0 0))".to_string());
    }
}
