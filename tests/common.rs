use std::collections::HashMap;
use std::collections::HashSet;

use grouping_by::GroupingBy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
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
        FOO.iter().grouping_by(|point| point.x),
        [
            (1, vec![&Point { x: 1, y: 2 }, &Point { x: 1, y: 3 }]),
            (2, vec![&Point { x: 2, y: 2 }, &Point { x: 2, y: 2 }])
        ]
        .iter()
        .cloned()
        .collect::<HashMap<i32, Vec<&Point>>>(),
    );
}
#[test]
fn test_grouping_by_set() {
    assert_eq!(
        FOO.iter().grouping_by_as_set(|point| point.x),
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
    );
}

#[test]
fn test_counter() {
    assert_eq!(
        FOO.iter().counter(|&x| x),
        [
            (&Point { x: 1, y: 2 }, 1),
            (&Point { x: 1, y: 3 }, 1),
            (&Point { x: 2, y: 2 }, 2),
        ]
        .iter()
        .cloned()
        .collect::<HashMap<&Point, usize>>()
    )
}

#[test]
fn test_counter_num() {
    let numbers_counted = [1i8, 2, 2, 3, 4].iter().counter(|&&x| x);

    assert_eq!(
        numbers_counted,
        [(1, 1), (2, 2), (3, 1), (4, 1)]
            .iter()
            .cloned()
            .collect::<HashMap<i8, usize>>()
    )
}

#[test]
fn test_grouping_by_num() {
    let numbers_grouped = [-1i8, -2, 1, 2].iter().grouping_by(|number| number.abs());

    assert_eq!(
        numbers_grouped,
        [(1, vec![&-1, &1]), (2, vec![&-2, &2])]
            .iter()
            .cloned()
            .collect::<HashMap<i8, Vec<&i8>>>()
    );
}

#[test]
fn test_grouping_by_num_set() {
    let numbers_grouped = [-1i8, -2, 1, 2]
        .iter()
        .grouping_by_as_set(|number| number.abs());

    assert_eq!(
        numbers_grouped,
        [(1, [1, -1].iter().collect()), (2, [2, -2].iter().collect())]
            .iter()
            .cloned()
            .collect()
    );
}

#[test]
fn grouping_by_min() {
    let a = FOO.iter().grouping_by_min(
        |point| point.y,
        |point1: &Point, point2: &Point| point1.x.cmp(&point2.x),
        |&&point| point,
    );
    assert_eq!(
        a,
        [(2, Point { x: 1, y: 2 }), (3, Point { x: 1, y: 3 })]
            .iter()
            .cloned()
            .collect()
    )
}

#[test]
fn grouping_by_max() {
    let a = FOO.iter().grouping_by_max(
        |point| point.y,
        |point1: &Point, point2: &Point| point1.x.cmp(&point2.x),
        |&&point| point,
    );
    assert_eq!(
        a,
        [(2, Point { x: 2, y: 2 }), (3, Point { x: 1, y: 3 })]
            .iter()
            .cloned()
            .collect()
    )
}
