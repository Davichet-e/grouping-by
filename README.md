# grouping-by

[![Crates.io](https://img.shields.io/crates/v/grouping-by.svg)](https://crates.io/crates/grouping-by)
[![Documentation](https://docs.rs/grouping_by/badge.svg)](https://docs.rs/grouping-by)

This small library provides users the possibility of grouping their iterators of various ways. It is still in development and therefore is not recommended for production code. There will be breaking changes constantly.

It is similar to Java `Collectors.groupingBy`

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

### More advanced usage

```rust
// This returns for each year, the codes of the contracts with the most days.
contracts.iter().grouping_by_max(
    |contract| contract.date.year(), // Key of HashMap
    |contract1, contract2| contract1.days.cmp(&contract2.days), // Comparator to get the max
    |contract| contract.code.clone() // Finisher to get the desired value
) // Returns `HashMap<i32, String>`
```

## Usage

Just import the trait (`use grouping_by::GroupingBy;`) into your crate and use it on your iterators.
