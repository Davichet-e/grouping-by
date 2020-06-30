//! This library provides users the possibility of grouping your iterators of various ways.
//! It is similar to Java `Collectors.groupingBy`
//!
//! It is implemented for any type which implements `Iterator`, but you can implement it to your custom iterator.
//!
//! ## Example:
//! ```rust
//! use std::collections::HashMap;
//! use crate::grouping_by::GroupingBy;
//!
//! #[derive(Debug, PartialEq)]
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
// TODO
// Implement two argument grouping by, just like groupingBy of Java does

use std::collections::{
    hash_map::{Entry, HashMap},
    HashSet,
};
use std::hash::Hash;

pub trait GroupingBy {
    /// The type of the Item of the iterator
    type GItem;

    /// Group by the key function given as parameter.
    /// The keys are the different values that the function can return,
    /// and the values are a `Vec` with the items of the iterator which has the key as property
    ///
    /// ## Example
    /// ```rust
    /// # use crate::grouping_by::GroupingBy;
    /// # use std::collections::{HashSet, HashMap};
    ///
    /// let numbers_grouped = [-1i8, -2, 1, 2]
    ///     .iter()
    ///     .grouping_by_as_set(|number| number.abs());
    ///
    /// assert_eq!(
    ///     numbers_grouped,
    ///     [(1, [1, -1].iter().collect()), (2, [2, -2].iter().collect())]
    ///         .iter()
    ///         .cloned()
    ///         .collect::<HashMap<i8, HashSet<&i8>>>()
    /// );
    /// ```
    fn grouping_by<K, F>(self, key: F) -> HashMap<K, Vec<Self::GItem>>
    where
        F: Fn(&Self::GItem) -> K,
        K: Eq + Hash;

    /// Group by the key function given as parameter.
    /// The keys are the different values that the function can return,
    /// and the values are a `HashSet` with the items of the iterator which has the key as property
    ///
    /// ## Example
    /// ```rust
    /// # use crate::grouping_by::GroupingBy;
    /// # use std::collections::{HashSet, HashMap};
    ///
    /// let numbers_grouped = [-1i8, -2, 1, 2]
    ///     .iter()
    ///     .grouping_by_as_set(|number| number.abs());
    ///
    /// assert_eq!(
    ///     numbers_grouped,
    ///     [(1, [1, -1].iter().collect()), (2, [2, -2].iter().collect())]
    ///         .iter()
    ///         .cloned()
    ///         .collect::<HashMap<i8, HashSet<&i8>>>()
    /// );
    /// ```
    fn grouping_by_as_set<K, F>(self, key: F) -> HashMap<K, HashSet<Self::GItem>>
    where
        Self::GItem: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        K: Eq + Hash;

    /// Count the elements of the iterator given a function
    ///
    /// ## Example
    /// ```rust
    /// # use crate::grouping_by::GroupingBy;
    /// # use std::collections::{HashSet, HashMap};
    /// let numbers_counted = [1, 2, 2, 3, 4].iter().counter(|&&x| x);
    ///
    /// assert_eq!(
    ///    numbers_counted,
    ///    [(1, 1), (2, 2), (3, 1), (4, 1)]
    ///        .iter()
    ///        .cloned()
    ///        .collect::<HashMap<i8, usize>>()
    /// )
    /// ```
    fn counter<K, F>(self, key: F) -> HashMap<K, usize>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K;

    /// Given a functions F, C and G, compute the minimum of the elements given a comparator and a finisher.
    /// Params:
    ///
    /// `key: F` -> function to create the keys of the resulting map
    ///
    /// `comparator: C` -> function to get the min value
    ///
    /// `finisher: G` -> function to perform the last transformation to the value
    fn grouping_by_min<K, F, G, O, C>(self, key: F, comparator: C, finisher: G) -> HashMap<K, O>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        G: Fn(&Self::GItem) -> O,
        C: Fn(&O, &O) -> std::cmp::Ordering;

    /// Given a functions F, C and G, compute the maximum of the elements given a comparator and a finisher.
    /// Params:
    ///
    /// `key` -> function to create the keys of the resulting map
    ///
    /// `comparator` -> function to get the max value
    ///
    /// `finisher` -> function to perform the last transformation to the value
    fn grouping_by_max<K, F, G, O, C>(self, key: F, comparator: C, finisher: G) -> HashMap<K, O>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        G: Fn(&Self::GItem) -> O,
        C: Fn(&O, &O) -> std::cmp::Ordering;
}

impl<T: Iterator> GroupingBy for T {
    type GItem = <T as Iterator>::Item;
    fn grouping_by<K, F>(self, key: F) -> HashMap<K, Vec<Self::GItem>>
    where
        F: Fn(&Self::GItem) -> K,
        K: Eq + Hash,
    {
        let mut map = HashMap::new();
        for item in self {
            map.entry(key(&item)).or_insert_with(Vec::new).push(item);
        }
        map
    }
    fn grouping_by_as_set<K, F>(self, key: F) -> HashMap<K, HashSet<Self::GItem>>
    where
        Self::GItem: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        K: Eq + Hash,
    {
        let mut map = HashMap::new();
        for item in self {
            map.entry(key(&item))
                .or_insert_with(HashSet::new)
                .insert(item);
        }
        map
    }
    fn counter<K, F>(self, key: F) -> HashMap<K, usize>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
    {
        let mut map = HashMap::new();
        for item in self {
            *map.entry(key(&item)).or_insert(0) += 1;
        }
        map
    }

    fn grouping_by_min<K, F, G, O, C>(self, key: F, comparator: C, finisher: G) -> HashMap<K, O>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        G: Fn(&Self::GItem) -> O,
        C: Fn(&O, &O) -> std::cmp::Ordering,
    {
        let mut map: HashMap<K, O> = HashMap::new();
        for item in self {
            let new_value = finisher(&item);
            match map.entry(key(&item)) {
                Entry::Occupied(mut entry) => {
                    if comparator(&new_value, entry.get()) == std::cmp::Ordering::Less {
                        entry.insert(new_value);
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(new_value);
                }
            }
        }
        map
    }

    fn grouping_by_max<K, F, G, O, C>(self, key: F, comparator: C, finisher: G) -> HashMap<K, O>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        G: Fn(&Self::GItem) -> O,
        C: Fn(&O, &O) -> std::cmp::Ordering,
    {
        let mut map: HashMap<K, O> = HashMap::new();
        for item in self {
            let new_value = finisher(&item);
            match map.entry(key(&item)) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    if comparator(&new_value, entry.get()) == std::cmp::Ordering::Greater {
                        entry.insert(new_value);
                    }
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(new_value);
                }
            }
        }
        map
    }
}
