use std::collections::HashMap;
use std::collections::HashSet;

use grouping_by::GroupingBy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

const POINT_ARRAY: [Point; 4] = [
    Point { x: 4, y: 2 },
    Point { x: 4, y: 2 },
    Point { x: 5, y: 13 },
    Point { x: 18, y: 9 },
];

const VECTOR_ARRAY: [Vector; 4] = [
    Vector { x: 1, y: 2, z: 4 },
    Vector { x: 1, y: 3, z: 3 },
    Vector { x: 2, y: 2, z: 2 },
    Vector { x: 2, y: 2, z: 1 },
];

#[test]
fn test_grouping_by() {
    let points_grouped = [
        (18, vec![&Point { x: 18, y: 9 }]),
        (5, vec![&Point { x: 5, y: 13 }]),
        (4, vec![&Point { x: 4, y: 2 }, &Point { x: 4, y: 2 }]),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<i32, Vec<&Point>>>();

    assert_eq!(
        points_grouped,
        POINT_ARRAY.iter().grouping_by(|point| point.x)
    );
}
#[test]
fn test_grouping_by_set() {
    let points_grouped = [
        (18, [Point { x: 18, y: 9 }].iter().collect()),
        (5, [Point { x: 5, y: 13 }].iter().collect()),
        (
            4,
            [Point { x: 4, y: 2 }, Point { x: 4, y: 2 }]
                .iter()
                .collect(),
        ),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<i32, HashSet<&Point>>>();

    assert_eq!(
        points_grouped,
        POINT_ARRAY.iter().grouping_by_as_set(|point| point.x)
    );
}

#[test]
fn test_counter() {
    let numbers_counted = [
        (&Point { x: 5, y: 13 }, 1),
        (&Point { x: 18, y: 9 }, 1),
        (&Point { x: 4, y: 2 }, 2),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<&Point, usize>>();

    assert_eq!(numbers_counted, POINT_ARRAY.iter().counter(|&x| x))
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
    let a = VECTOR_ARRAY.iter().grouping_by_min(
        |vector| vector.y,
        |vector1, vector2| vector1.x.cmp(&vector2.x),
        |vector| vector.z,
    );
    assert_eq!(a, [(2, 4), (3, 3)].iter().cloned().collect())
}

#[test]
fn grouping_by_max() {
    let a = VECTOR_ARRAY.iter().grouping_by_max(
        |vector| vector.y,
        |vector1, vector2| vector1.x.cmp(&vector2.x),
        |vector| vector.z,
    );
    assert_eq!(a, [(2, 2), (3, 3)].iter().cloned().collect())
}

#[test]
fn test_grouping_by_summing() {
    let points_summed: HashMap<i32, i32> = [(4, 4), (5, 13), (18, 9)].iter().cloned().collect();

    assert_eq!(
        points_summed,
        POINT_ARRAY
            .iter()
            .grouping_by_summing(|point| point.x, |point| point.y)
    );
}
