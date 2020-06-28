use prueba_stream::GroupingBy;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

static FOO: [Point; 4] = [
    Point { x: 1, y: 2 },
    Point { x: 1, y: 3 },
    Point { x: 2, y: 2 },
    Point { x: 2, y: 2 },
];

fn main() {
    println!("{:?}", FOO.iter().grouping_by(|point| point.x));

    println!("{:?}", FOO.iter().grouping_by_as_set(|point| point.y));
}
