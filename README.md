# grouping-by

[![Crates.io](https://img.shields.io/crates/v/grouping-by.svg)](https://crates.io/crates/grouping-by)
[![Documentation](https://docs.rs/grouping_by/badge.svg)](https://docs.rs/grouping-by)
This library provides users the possibility of grouping their iterators by any function.
It is similar to C# `IEnumerable.GroupBy` or Java `Collectors.groupingBy`

# Example:

```rust
#[derive(Debug, PartialEq)]
struct Point {
   x: i32,
   y: i32,
}
let array: [Point; 4] = [
       Point { x: 1, y: 2 },
       Point { x: 1, y: 3 },
       Point { x: 2, y: 2 },
       Point { x: 2, y: 2 },
];

assert_eq!(
    [
        (1, vec![&Point { x: 1, y: 2 }, &Point { x: 1, y: 3 }]),
        (2, vec![&Point { x: 2, y: 2 }, &Point { x: 2, y: 2 }])
    ]
    .iter()
    .cloned()
    .collect::<HashMap<i32, Vec<&Point>>>(),
    array.iter().grouping_by(|point| point.x)
);
```

## Usage

Just import the trait (`use grouping_by::GroupingBy;`) into your crate and it would just work.
