# bbox-2d
Minimum bounding rectangle (**MBR**)- axis aligned rectangle in 2d space.

## Coverage
`100%` line coverage

## Examples 
```rust
use bbox_2d::MBR;
use point::Point;

fn main() {
    let pt = Point{x:  367.74747560229144, y: 363.2231833134207};
    let a = MBR::new_from_bounds(Point { x: 350., y: 400. }, Point { x: 200., y: 250. });
    let b = MBR::new(300., 200., 400., 350.);
    println!("a intersects b = {} ", a.intersects(&b));
    println!("a disjoint b = {} ", a.disjoint(&b));
    println!("a equals b = {} ", a == b );

    let inter = a.intersection(&b).unwrap();
    println!("{}", inter);//POLYGON ((300 250,300 350,350 350,350 250,300 250))
    //intersection (same as inter above)
    let mut  inter_a_b = (&a & &b).unwrap();
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
    println!("{}", inter_a_b);//POLYGON ((270 225,270 375,380 375,380 225,270 225))

    //contains
    println!("inter_a_b intersects pt = {}", inter_a_b.intersects_point(&pt));
    println!("inter_a_b intersects pt = {}", inter_a_b.intersects_xy(pt.x , pt.y));
    println!("inter_a_b intersects pt = {}", inter_a_b.contains(&MBR::new_from_pt(pt)));
    println!("inter_a_b intersects pt = {}", inter_a_b.contains_xy(pt.x, pt.y));

    //distance
    println!("a distance to mbr(pt) = {}", a.distance(&MBR::new_from_pt(pt)));

}
```

*Intersection = `A & B`*

<img src="assets/01-intersection.jpg" alt="01-intersection" width="50%"/>

*Union =`A | B`*

<img src="assets/01-union.jpg" alt="01-union" width="50%"/>

*Intersection exapand by `dx=30,dy=25`*

<img src="assets/01-intersection-expand-deltas.jpg" alt="expand-deltas" width="50%"/>


## API
### Fields
```rust
struct MBR{
    minx: f64,
    miny: f64,
    maxx: f64,
    maxy: f64,
}
```

### Constructors 
```rust 
new(x1: f64, y1: f64, x2: f64, y2: f64) -> MBR
```
```rust
//constructs MBR as provided params  
new_raw(minx: f64, miny: f64, maxx: f64, maxy: f64) -> MBR
```
```rust
//origin (0, 0, 0, 0)
new_default() -> MBR
```

```rust
//MBR from point a(x, y)
new_from_pt(a: Point) -> MBR
```

```rust
//MBR bounded by two points a(x, y) and b(x, y)
new_from_bounds(a: Point, b: Point) -> MBR
```

### Methods
**bbox** is reference to `self`
```rust
bbox(&self) -> &Self
```
**clone** is copy of `self`
```rust
clone(&self) -> Self
```
**width** of bounding box.
```rust
width(&self) -> f64

```
**height** of bounding box.
```rust
height(&self) -> f64
```

**area** of bounding box.
```rust
area(&self) -> f64
```

as a **closed** polygon coordinates
```rust
as_poly_array(&self) -> Vec<Point>
```
as an array 
```rust
as_array(&self) -> [f64; 4]
```
bounds as **tuple** [minx,miny, maxx,maxy]
```rust
as_tuple(&self) -> (f64, f64, f64, f64)
```

lower left and upper right: Point(minx,miny)-Point(maxx,maxy)


```rust
llur(self) -> (Point, Point)
```

**equality** of two bounding boxes
```rust
equals(&self, other: &Self) -> bool
```

is it a **point**, width and height as 0.
```rust
is_point(&self) -> bool
```

check for **containment** of `other`
```rust
contains(&self, other: &Self) -> bool
```

**contains** `x`, `y`
```rust
contains_xy(&self, x: f64, y: f64) -> bool
```

**completely contains** other 
```rust
completely_contains(&self, other: &Self) -> bool
```

**completely contains** `x`, `y` 
```rust
completely_contains_xy(&self, x: f64, y: f64) -> bool
```

**translate** by `dx`, `dy`
```rust
translate(&self, dx: f64, dy: f64) -> MBR
```

**intersects** other box
```rust
intersects(&self, other: &Self) -> bool
```

**intersects** point
```rust
intersects_point(&self, pt: &Point) -> bool
```

**intersects** `x, y`
```rust
intersects_xy(&self, x: f64, y: f64) -> bool
```

**intersects** bounds
```rust
intersects_bounds(&self, pt1: &Point, pt2: &Point) -> bool
```

checks **disjoint** between boxes
```rust
disjoint(&self, m: &Self) -> bool

```
**intersection** between boxes
```rust
intersection(&self, other: &Self) -> Option<MBR>
```

**expand** to include other box
```rust
expand_to_include(&mut self, other: &Self) -> &mut MBR
```

**expand** to include `x, y`
```rust
expand_to_include_xy(&mut self, x: f64, y: f64) -> &mut Self
```

**expand** by deltas : dx, dy (pad x and y dimension by dx and dy)
```rust
expand_by_delta(&mut self, dx: f64, dy: f64) -> &mut MBR
```

**distance** between boxes
```rust
distance(&self, other: &Self) -> f64
```

**square** distance between boxes
```rust
distance_square(&self, other: &Self) -> f64
```
**wkt** string 
```rust
wkt(&self) -> String
```

## LIC 
 MIT