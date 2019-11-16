use super::*;
use math_util::round;
use rstar::Envelope;
use serde_json;

#[test]
fn test_construction() {
    let m_a: MBR = (0.5, 0.2).into();
    let m_b: MBR = [0.5, 0.2].into();
    assert!(m_a.equals(&m_b));
    assert_eq!(m_a.as_tuple(), (0.5, 0.2, 0.5, 0.2));
    assert_eq!(m_b.as_array(), [0.5, 0.2, 0.5, 0.2]);
    assert!(m_a.is_point());
    assert!(m_b.is_point());

    let aabb_a: AABB<[f64; 2]> = AABB::from_corners([0.0, 0.0], [0.5, 0.2]);
    let aabb_b: AABB<[f64; 2]> = AABB::from_corners([2.0, 2.0], [-0.5, -0.2]);
    let m_a: MBR = aabb_a.into();
    let m_b: MBR = aabb_b.into();
    assert_eq!(aabb_a.area(), m_a.area());
    assert_eq!(aabb_b.area(), m_b.area());
    let serialized = serde_json::to_string(&m_b).unwrap();

    assert_eq!(
        serialized,
        String::from(r#"{"minx":-0.5,"miny":-0.2,"maxx":2.0,"maxy":2.0}"#)
    );

    let data: Boxes = vec![
        [-115, 45, -105, 55], [105, 45, 115, 55], [105, -55, 115, -45], [-115, -55, -105, -45],
    ].into();

    let pts = vec![[-125., -25.], [-125., 25.], [125., 25.], [125., -25.], [-125., -25.]];

    let mut sec_box: MBR = pts[0].into();
    for p in pts {
        sec_box.expand_to_include_point(p);
    }
    for b in data.boxes.iter() {
        assert!(sec_box.disjoint(b))
    }
    assert!(data[0].disjoint(&sec_box));
    assert!(data[1].disjoint(&sec_box));
    assert!(data[2].disjoint(&sec_box));
    assert!(data[3].disjoint(&sec_box));


    let m0 = MBR::new(0.0, 0.0, 0.5, 0.2);
    let m1 = MBR::new(2.0, 2.0, -0.5, -0.2);

    let serialized = serde_json::to_string(&m1).unwrap();

    assert_eq!(
        serialized,
        String::from(r#"{"minx":-0.5,"miny":-0.2,"maxx":2.0,"maxy":2.0}"#)
    );
    assert_eq!(m0.envelope().area(), m0.area());
    assert_eq!(m1.envelope().area(), m1.area());

    let m = &m0 + &m1;
    assert_eq!(m, MBR::new(-0.5, -0.2, 2.0, 2.0));

    let m1 = MBR::new_raw(2.0, 2.0, -0.5, -0.2);
    assert_eq!(
        m1,
        MBR {
            minx: 2.0,
            miny: 2.0,
            maxx: -0.5,
            maxy: -0.2,
        }
    );

    let m = MBR::new(2.0, 2.0, 0.5, 0.2);
    assert_eq!(
        m, MBR::from((0.5, 0.2, 2.0, 2.0)),
    );

    assert_eq!(
        (m.width(), m.height(), m.area(), m.is_point()),
        (1.5, 1.8, 1.5 * 1.8, false)
    );
    assert_eq!(m.as_tuple(), MBR::from((0.5, 0.2, 2.0, 2.0)).as_tuple());
    assert_eq!(m.as_array(), MBR::from([0.5, 0.2, 2.0, 2.0]).as_array());

    let b = m.as_poly_array();
    assert_eq!(
        (b[0], b[4], b.len()),
        ([0.5, 0.2], [0.5, 0.2], 5)
    );

    let m1 = m.copy();
    assert_eq!((m1.equals(&m), m1.area()), (m1 == m, m.area()));
}

#[test]
fn test_methods() {
    let m: MBR = [2.0, 2.0, 0.5, 0.2].into();
    assert_eq!(
        (m.width(), m.height(), m.area(), m.is_point()),
        (1.5, 1.8, 1.5 * 1.8, false)
    );
    assert_eq!((0.5, 0.2, 2.0, 2.0), m.as_tuple());

    let b = m.as_poly_array();
    let b0 = [0.5, 0.2];
    let b1 = [0.5, 2.];
    let b2 = [2., 2.];
    let b3 = [2., 0.2];
    let b4 = [0.5, 0.2];

    assert_eq!(
        (b.len(), b[0], b[1], b[2], b[3], b[4], b[0]),
        (5, b0, b1, b2, b3, b4, b4)
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

    let mut m0: MBR = [1., 1., 1., 1.].into();
    m0.expand_by_delta(1., 1.);

    let m1 = MBR::new_from_array([0., 0., 2., 2.]);
    let m2 = MBR::new_from_array([4., 5., 8., 9.]);
    let m3 = MBR::new_from_array([4., 5., 8., 9.]);
    let m4 = MBR::new_from_array([5., 0., 8., 2.]);
    let m5 = MBR::new_from_array([5., 11., 8., 9.]);
    let m6 = MBR::new_from_array([0., 0., 2., -2.]);
    let m7 = MBR::new_from_array([-2., 1., 4., -2.]);
    let m8 = MBR::new_from_array([4., 2., 4., 2.]);
    let mut vects = vec![m1, m2, m4, m5, m6, m7, m3];
    vects.sort();

    let m0123 = MBR::new_from_array([0., 2., 1., 3.]);
    let clone_m0123 = m0123;

    let r1: [f64; 4] = [0., 0., 2., 2.];
    assert_eq!(m1.as_array(), r1);
    assert_eq!(clone_m0123, m0123);
    assert!(m0.equals(&m1));
    assert_eq!(*m0.bbox(), m0);
    assert_eq!((&m0 as &dyn BBox).bbox(), m0.bbox());
    assert!(m00.equals(&m1));
    assert_ne!(m1, m2);

    assert!(m00.intersects(&n00));
    let nm00 = m00.intersection(&n00);
    assert!(nm00.is_some());

    let bln1 = nm00.unwrap().ll() == [0.0, 0.0];
    let bln2 = nm00.unwrap().ur() == [0.0, 0.0];
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

    if let Some(v) = m67 {
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
    assert_eq!(m1.distance_2(&m8.ll()), m1.distance_square(&m8));
    assert_eq!(m1.distance_2(&m8.ur()), m1.distance_square(&m8));

    let a = MBR::new_from_array([0., 0., 2., 0.]);
    let b = MBR::new_from_array([4., 0., 7., 0.]);
    assert_eq!(a.distance(&b), 2.0);
    let a = MBR::new_from_array([0., 0., 0., 2.]);
    let b = MBR::new_from_array([0., 4., 0., 7.]);
    assert_eq!(a.distance(&b), 2.0);
}

#[test]
fn test_ops2() {
    let mut m00 = MBR::new_default();
    m00.expand_to_include_xy(2., 2.);

    let mut n00 = MBR::new_default();
    n00.expand_to_include_xy(-2., -2.);

    let mut m0 = MBR::new_from_array([1., 1., 1., 1.]);
    m0.expand_by_delta(1., 1.);
    let mut m0_pt: MBR = (0., 0.).into();

    let m1 = MBR::new_from_array([0., 0., 2., 2.]);
    let m2 = MBR::new_from_array([4., 5., 8., 9.]);

    let m1_bounds = MBR::new(0., 0., 2., 2.);
    let m2_bounds = MBR::new(4., 5., 8., 9.);

    let m3 = [1.7, 1.5, 5., 9.].into();
    let m4 = (5., 0., 8., 2.).into();
    let m5 = MBR::new_from_array([5., 11., 8., 9.]);
    let m6 = MBR::new_from_array([0., 0., 2., -2.]);
    let m7 = MBR::new_from_array([-2., 1., 4., -2.]);
    let m8 = MBR::new_from_array([-1., 0., 1., -1.5]);
    let m9 = MBR::new_from_array([-1., 0., 100., 10.5]);
    let mut vects = vec![m9, m1, m2, m3, m4, m5, m6, m7, m8];
    vects.sort();

    let p = [1.7, 1.5];
    let p0 = [1.7, 0.0];

    let m0123 = MBR::new_from_array([0., 2., 1., 3.]);
    let clone_m0123 = m0123;

    //SECTION (Constructs)
    assert_ne!(m0, m0_pt);
    assert_eq!(m0.llur(), [[0., 0.], [2., 2.]]);
    m0_pt.expand_to_include_xy(2., 2.);
    assert_eq!(m0, m0_pt);
    assert_eq!(m0.llur(), m0_pt.llur());

    assert_eq!(m1, m1_bounds);
    assert_eq!(m2, m2_bounds);

    //SECTION (Equals)
    let r1 = [0., 0., 2., 2.];
    assert_eq!(m1.as_array(), r1);
    assert_eq!(clone_m0123, m0123);
    assert!(m0.equals(&m1));
    assert_eq!(*m0.bbox(), m0);
    assert!(m00.equals(&m1));
    assert_ne!(m1, m2);

    //    SECTION("intersects , distance")
    assert!(m1.intersects_xy(p[0], p[1]));
    assert!(m1.intersects_xy(p0[0], p0[1]));
    assert!(m1.intersects_point(&p));
    assert!(m1.intersects_point(&p0));

    assert!(m00.intersects(&n00));
    let nm00 = m00.intersection(&n00);
    assert_ne!(nm00, None);

    let bln1 = nm00.unwrap().ll() == [0., 0.];
    let bln2 = nm00.unwrap().ur() == [0., 0.];
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

    assert_eq!(_m13, m13.unwrap().as_array());
    assert_eq!(_m23, m23.unwrap().as_array());

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


    let a = MBR::new_from_array([
        -7.703505430214746,
        3.0022503796012305,
        -5.369812194018422,
        5.231449888803689,
    ]);
    assert_eq!(
        m1.distance(&a),
        (-5.369812194018422f64).hypot(3.0022503796012305 - 2.)
    );

    let b = MBR::new_from_array([
        -4.742849832055231,
        -4.1033230559816065,
        -1.9563504455521576,
        -2.292098454754609,
    ]);
    assert_eq!(
        m1.distance(&b),
        (-1.9563504455521576f64).hypot(-2.292098454754609)
    );

    //    SECTION("contains, disjoint , contains completely")
    let p1 = [-5.95, 9.28];
    let p2 = [-0.11, 12.56];
    let p3 = [3.58, 11.79];
    let p4 = [-1.16, 14.71];

    let mp12 = MBR::new_from_bounds(p1, p2);
    let mp34 = MBR::new_from_bounds(p3, p4);

    // intersects but segment are disjoint
    assert!(mp12.intersects(&mp34));
    assert!(mp12.intersects_bounds(&p3, &p4));
    assert!(!mp12.intersects_bounds(&m1.ll(), &m1.ur()));
    assert!(!mp12.intersects_xy(p3[0], p3[1]));
    assert!(m1.contains_point([1., 1.]));

    let mbr11 = [1., 1., 1.5, 1.5].into();
    let mbr12 = [1, 1, 2, 2].into();
    let mbr13 = (1., 1., 2.000045, 2.00001).into();
    let mbr14 = MBR::new_from_array([2.000045, 2.00001, 4.000045, 4.00001]);

    assert!(m1.contains(&mbr11));
    assert!(m1.contains(&mbr12));
    assert!(!m1.contains(&mbr13));
    assert!(!m1.disjoint(&mbr13)); // False
    assert!(m1.disjoint(&mbr14)); // True disjoint

    assert!(m1.contains_point([1.5, 1.5]));
    assert!(m1.contains_point([2., 2.]));

    assert!(m1.completely_contains(&mbr11));
    assert!(m1.completely_contains_point([1.5, 1.5]));
    assert!(m1.completely_contains_point([1.5, 1.5]));
    assert!(!m1.completely_contains_point([2., 2.]));
    assert!(!m1.completely_contains(&mbr12));
    assert!(!m1.completely_contains(&mbr13));

    //    SECTION("translate, expand by, area")
    let mut ma = MBR::new_from_array([0., 0., 2., 2.]);
    let mb = MBR::new_from_array([-1., -1., 1.5, 1.9]);
    let mc = MBR::new_from_array([1.7, 1.5, 5., 9.]);
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
    let polyarr = vec![[0., 0.], [0., 9.], [5., 9.], [5., 0.], [0., 0.], ];
    assert_eq!(ma.as_array(), arr); //ma modified by expand
    for (i, &o) in ma.as_poly_array().iter().enumerate() {
        assert_eq!(o, polyarr[i]);
    }

    arr = [1.7, 1.5, 5., 9.];
    assert_eq!(mc.as_array(), arr); //should not be touched
    arr = [-1., -1., 2., 2.];
    assert_eq!(md.as_array(), arr); //ma modified by expand

    //mc area
    assert_eq!(mc.area(), 24.75);

    let mt = m1.translate(1., 1.);
    let mut mby = m1;
    mby.expand_by_delta(-3., -3.);

    let m1c = m1.centre();
    let mtc = mt.centre();

    let pt = [1., 1.];
    assert_eq!(m1c, pt);
    let pt = [2., 2.];
    assert_eq!(mtc, pt);
    arr = [1., 1., 3., 3.];
    assert_eq!(mt.as_array(), arr);
    arr = [-1., -1., 3., 3.];
    assert_eq!(mby.as_array(), arr);

    //    SECTION("wkt string")
    assert_eq!(m1.wkt(), "POLYGON ((0 0,0 2,2 2,2 0,0 0))".to_string());
    assert_eq!(
        format!("{}", m1),
        "POLYGON ((0 0,0 2,2 2,2 0,0 0))".to_string()
    );
}
