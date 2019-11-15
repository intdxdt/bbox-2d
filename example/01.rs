use bbox_2d::MBR;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    let pt =  [367.74747560229144, 363.2231833134207];
    let a = MBR::new( 350.,  400. ,  200.,  250. );
    let b = [300., 200., 400., 350.].into();
    println!("a intersects b = {} ", a.intersects(&b));
    println!("a disjoint b = {} ", a.disjoint(&b));
    println!("a equals b = {} ", a == b);

    let inter = a.intersection(&b).unwrap();
    println!("{}", inter); //POLYGON ((300 250,300 350,350 350,350 250,300 250))
                           //intersection (same as inter above)
    let mut inter_a_b = (&a & &b).unwrap();
    println!("area A={}, area B={}; A&B {}", a.area(), b.area(), inter_a_b.area());
    //area A=22500, area B=15000; A&B 5000
    println!("inter_a_b intersects pt = {}", inter_a_b.intersects_point(&pt));

    //union
    let union_a_b = &a | &b;
    println!("area A={}, area B={}; A|B {}", a.area(), b.area(), union_a_b.area());
    //area A=22500, area B=15000; A+B 40000
    println!("a | b = {}", union_a_b);

    //some methods :
    println!("is a&b decompose as point = {}", inter_a_b.is_point());
    println!("width  of a&b = {}", inter_a_b.width());
    println!("height of a&b = {}", inter_a_b.height());
    inter_a_b.expand_by_delta(30.0, 25.0);
    println!("{}", inter_a_b); //POLYGON ((270 225,270 375,380 375,380 225,270 225))

    //contains
    println!("inter_a_b intersects pt = {}", inter_a_b.intersects_point(&pt));
    println!("inter_a_b intersects pt = {}", inter_a_b.intersects_xy(pt[0], pt[1]));
    println!("inter_a_b intersects pt = {}", inter_a_b.contains(&MBR::new_from_pt(pt)));
    println!("inter_a_b intersects pt = {}", inter_a_b.contains_xy(pt[0], pt[1]));

    //distance
    println!("a distance to mbr(pt) = {}", a.distance(&MBR::new_from_pt(pt)));
}
