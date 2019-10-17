# bbox-2d
Minimum bounding rectangle (**MBR**)- axis aligned rectangle in 2d space.

## Coverage
`100%` line coverage

## Examples 
`todo:`

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

Lower left and upper right corners as an array [minx,miny, maxx,maxy]

as_tuple(&self) -> (f64, f64, f64, f64)

Lower left and upper right corners as a tuple (minx,miny, maxx,maxy)

llur(self) -> (Point, Point)

lower left and upper right as tuple ((minx,miny),(maxx,maxy))

equals(&self, other: &Self) -> bool

Compare equality of two bounding boxes

is_point(&self) -> bool

Checks if bounding box can be represented as a point, width and height as 0.

contains(&self, other: &Self) -> bool

Contains bonding box is true if mbr completely contains other, boundaries may touch

contains_xy(&self, x: f64, y: f64) -> bool

contains x, y

completely_contains(&self, other: &Self) -> bool

Completely contains bonding box is true if mbr completely contains other without touching boundaries

completely_contains_xy(&self, x: f64, y: f64) -> bool

completely_contains_xy is true if mbr completely contains location with {x, y} without touching boundaries

translate(&self, dx: f64, dy: f64) -> MBR

Translate bounding box by change in dx and dy.

intersects(&self, other: &Self) -> bool

Checks if bounding box intersects other

intersects_point(&self, pt: &Point) -> bool

intersects point

intersects_xy(&self, x: f64, y: f64) -> bool

intersects point with x, y

intersects_bounds(&self, pt1: &Point, pt2: &Point) -> bool

Intersects bounds

disjoint(&self, m: &Self) -> bool

Test for disjoint between two mbrs

intersection(&self, other: &Self) -> Option<MBR>

Computes the intersection of two bounding box

expand_to_include(&mut self, other: &Self) -> &mut MBR

Expand include other bounding box

expand_to_include_xy(&mut self, x: f64, y: f64) -> &mut Self

Expand to include x,y

expand_by_delta(&mut self, dx: f64, dy: f64) -> &mut MBR

Expand by delta in x and y

distance(&self, other: &Self) -> f64

distance computes the distance between two mbrs

distance_square(&self, other: &Self) -> f64

distance square computes the squared distance between bounding boxes

wkt(&self) -> String

