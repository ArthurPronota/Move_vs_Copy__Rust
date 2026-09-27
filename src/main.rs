#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

fn main() {
    let p = Point{x: 10., y: 20.} ;
    let q = p ;
    let r = p ;

    println!("{}, {}, {}", p.x, q.x, r.x) ; // Out: 10, 10, 10
}
