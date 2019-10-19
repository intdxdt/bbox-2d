use bbox_2d::MBR;
use point::Point;

fn main() {
    let a = MBR::new_from_bounds(Point { x: 350., y: 400. }, Point { x: 200., y: 250. });
    let b = MBR::new( 300.,  200. ,  400.,  350. );
    let inter = a.intersection(&b).unwrap();
    println!("{}", inter);//POLYGON ((300 250,300 350,350 350,350 250,300 250))
    //intersection (same as inter above)
    let inter_a_b = (&a & &b).unwrap();
    println!("area A={}, area B={}; A&B {}", a.area(), b.area(), inter_a_b.area());
    //area A=22500, area B=15000; A&B 5000
    //union
    let union_a_b = &a | &b;
    println!("area A={}, area B={}; A|B {}", a.area(), b.area(), union_a_b.area());
    //area A=22500, area B=15000; A+B 40000
}