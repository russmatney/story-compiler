// Story_Compiler

// The outline for this program is very simple.
// It is command line based. It is centered around entering one sentence at a time.
// On entering, the sentence is sorted. Into an Array. The options are only ahead, or behind (i.e. push or upshift).
// At any point, the whole array can be printed to the command line.
// Future features includes flags to add arbitrary metadata, and ways to rearrange parts of the story, etc.


use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {

    let minmax = MinMax(0, 14);
    println!("{}", minmax);

    let point2 = Point2 { x: 1.2, y: 4.2 };
    println!("{}", point2);
}









