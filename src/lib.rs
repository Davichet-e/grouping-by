//! This library provides users the possibility of grouping their iterators by any function.
//! It is similar to C# `IEnumerable.GroupBy` or Java `Collectors.groupingBy`
//!
//! # Example:
//! ```rust
//! #[derive(Debug, Clone)]
//! struct Point {
//!    x: i32,
//!    y: i32,
//! }
//! let array: [Point; 4] = [
//!        Point { x: 1, y: 2 },
//!        Point { x: 1, y: 3 },
//!        Point { x: 2, y: 2 },
//!        Point { x: 2, y: 2 },
//! ];
//!
//! assert_eq!(
//!     [
//!         (1, vec![&Point { x: 1, y: 2 }, &Point { x: 1, y: 3 }]),
//!         (2, vec![&Point { x: 2, y: 2 }, &Point { x: 2, y: 2 }])
//!     ]
//!     .iter()
//!     .cloned()
//!     .collect::<HashMap<i32, Vec<&Point>>>(),
//!     array.iter().grouping_by(|point| point.x)
//! );
//! ```
//!
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait GroupingBy: Iterator {
    /// Group by the key function given as parameter.
    /// The keys are the different values that the function can return,
    /// and the values are a `Vec` with the items of the iterator which has the key as property
    fn grouping_by<K, F>(self, key: F) -> HashMap<K, Vec<Self::Item>>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> K,
        K: Eq + Hash;

    /// Group by the key function given as parameter.
    /// The keys are the different values that the function can return,
    /// and the values are a `HashSet` with the items of the iterator which has the key as property
    fn grouping_by_as_set<K, F>(self, key: F) -> HashMap<K, HashSet<Self::Item>>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
        F: Fn(&Self::Item) -> K,
        K: Eq + Hash;
}

impl<T: Iterator> GroupingBy for T {
    fn grouping_by<K, F>(self, key: F) -> HashMap<K, Vec<Self::Item>>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> K,
        K: Eq + Hash,
    {
        let mut map: HashMap<K, Vec<Self::Item>> = HashMap::new();
        for item in self {
            map.entry(key(&item)).or_insert_with(Vec::new).push(item);
        }
        map
    }
    fn grouping_by_as_set<K, F>(self, key: F) -> HashMap<K, HashSet<Self::Item>>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
        F: Fn(&Self::Item) -> K,
        K: Eq + Hash,
    {
        let mut map: HashMap<K, HashSet<Self::Item>> = HashMap::new();
        for item in self {
            map.entry(key(&item))
                .or_insert_with(HashSet::new)
                .insert(item);
        }
        map
    }
}
