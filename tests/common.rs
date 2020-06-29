use std::collections::HashMap;
use std::collections::HashSet;

use grouping_by::GroupingBy;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

const FOO: [Point; 4] = [
    Point { x: 1, y: 2 },
    Point { x: 1, y: 3 },
    Point { x: 2, y: 2 },
    Point { x: 2, y: 2 },
];

#[test]
fn test_grouping_by() {
    assert_eq!(
        [
            (1, vec![&Point { x: 1, y: 2 }, &Point { x: 1, y: 3 }]),
            (2, vec![&Point { x: 2, y: 2 }, &Point { x: 2, y: 2 }])
        ]
        .iter()
        .cloned()
        .collect::<HashMap<i32, Vec<&Point>>>(),
        FOO.iter().grouping_by(|point| point.x)
    );
}
#[test]
fn test_grouping_by_set() {
    assert_eq!(
        [
            (
                1,
                [Point { x: 1, y: 2 }, Point { x: 1, y: 3 }]
                    .iter()
                    .collect()
            ),
            (
                2,
                [Point { x: 2, y: 2 }, Point { x: 2, y: 2 }]
                    .iter()
                    .collect()
            )
        ]
        .iter()
        .cloned()
        .collect::<HashMap<i32, HashSet<&Point>>>(),
        FOO.iter().grouping_by_as_set(|point| point.x)
    );
}
