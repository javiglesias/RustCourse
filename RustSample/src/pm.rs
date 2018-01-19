//methods
struct Point {
    x:f32,
    y:f64
}
struct Line {
    s:Point,
    e:Point
}
impl Line {
    fn len(&self) ->f32 {
       self.s.x
    }
}
//pattern matching
fn how_many(x:i32) ->&'static str {
    match x { 
//the order of the matches are important, the "_" match has to be the last
        1 | 2 => "1|2",
        0 => "NO",
        9...11 => "Enough",
        _ if x == 3 => "3",
        _ => "few"
    }
}
pub fn patterns_matching() {
    for x in 0..13 {
        println!("o: {}",how_many(x));
    }
    let point = (1,1);
    match point {
        (0,0) => println!("origin"),
        (0,y) => println!("y:{}",y),
        (x,0) => println!("x:{}", x),
        (x,y) => println!("x,y ({}|{})",x,y),
        _ => print!("-")
    }
}