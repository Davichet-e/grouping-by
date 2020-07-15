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

    /// Given a functions F, C and G, compute the maximum of the elements given a comparator and a finisher.
    ///
    /// Params:
    ///
    /// `key` -> function to create the keys of the resulting map
    ///
    /// `comparator` -> function to get the max value
    ///
    /// `finisher` -> function to perform the last transformation to the value
    ///
    /// ## Example:
    ///
    /// ```rust
    /// # use crate::grouping_by::GroupingBy;
    /// struct Vector {
    ///     x: i32,
    ///     y: i32,
    ///     z: i32
    /// }
    ///
    /// const BAR: [Vector; 4] = [
    ///     Vector { x: 1, y: 2, z: 4 },
    ///     Vector { x: 1, y: 3, z: 3 },
    ///     Vector { x: 2, y: 2, z: 2 },
    ///     Vector { x: 2, y: 2, z: 1 },
    /// ];
    ///
    /// // Return a HashMap with the `y` fields as keys
    /// // and the `z` fields of the vectors with that key with the maximum `x`
    ///
    /// let a = BAR.iter().grouping_by_max(
    ///     |vector| vector.y,
    ///     |vector1, vector2| vector1.x.cmp(&vector2.x),
    ///     |vector| vector.z,
    /// );
    /// assert_eq!(a, [(3, 3), (2, 2)].iter().cloned().collect())
    /// ```
    fn grouping_by_max<K, F, G, O, C>(self, key: F, comparator: C, finisher: G) -> HashMap<K, O>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        C: Fn(&Self::GItem, &Self::GItem) -> std::cmp::Ordering,
        G: Fn(&Self::GItem) -> O;

    /// Given a functions F, C and G, compute the maximum of the elements given a comparator and a finisher.
    ///
    /// Params:
    ///
    /// `key` -> function to create the keys of the resulting map
    ///
    /// `comparator` -> function to get the max value
    ///
    /// `finisher` -> function to perform the last transformation to the value
    ///
    /// ## Example:
    ///
    /// ```rust
    /// # use crate::grouping_by::GroupingBy;
    /// struct Vector {
    ///     x: i32,
    ///     y: i32,
    ///     z: i32
    /// }
    ///
    /// const BAR: [Vector; 4] = [
    ///     Vector { x: 1, y: 2, z: 4 },
    ///     Vector { x: 1, y: 3, z: 3 },
    ///     Vector { x: 2, y: 2, z: 2 },
    ///     Vector { x: 2, y: 2, z: 1 },
    /// ];
    ///
    /// // Return a HashMap with the `y` fields as keys
    /// // and the `z` fields of the vectors with that key with the minimum `x`
    ///
    /// let a = BAR.iter().grouping_by_min(
    ///     |vector| vector.y,
    ///     |vector1, vector2| vector1.x.cmp(&vector2.x),
    ///     |vector| vector.z,
    /// );
    /// assert_eq!(a, [(2, 4), (3, 3)].iter().cloned().collect())
    /// ```
    fn grouping_by_min<K, F, G, O, C>(self, key: F, comparator: C, finisher: G) -> HashMap<K, O>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        C: Fn(&Self::GItem, &Self::GItem) -> std::cmp::Ordering,
        G: Fn(&Self::GItem) -> O;

    /// Return a map containing the sum of the values of a given key both obtained by provided as input functions.
    ///
    /// Params:
    ///
    /// `key` -> function to create the keys of the resulting map
    ///
    /// `value` -> function to get the values to sum
    ///
    /// ## Example:
    ///
    /// ```rust
    /// # use crate::grouping_by::GroupingBy;
    /// struct Vector {
    ///     x: i32,
    ///     y: i32,
    ///     z: i32
    /// }
    ///
    /// const BAR: [Vector; 4] = [
    ///     Vector { x: 1, y: 2, z: 4 },
    ///     Vector { x: 1, y: 3, z: 3 },
    ///     Vector { x: 2, y: 2, z: 2 },
    ///     Vector { x: 2, y: 2, z: 1 },
    /// ];
    ///
    /// let a = BAR.iter().grouping_by_summing(
    ///     |vector| vector.x,
    ///     |vector| vector.y
    /// );
    /// assert_eq!(a, [(2, 4), (1, 5)].iter().cloned().collect())
    /// ```
    fn grouping_by_summing<K, V, F, G>(self, key: F, sum_func: G) -> HashMap<K, V>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        G: Fn(&Self::GItem) -> V,
        V: Default + std::ops::AddAssign;
}

mod utilities {
    use super::{Entry, Hash, HashMap};

    pub fn grouping_by_min_max_aux<T, K, F, G, O, C>(
        iterator: T,
        key: F,
        comparator: C,
        finisher: G,
        type_ord: std::cmp::Ordering,
    ) -> HashMap<K, O>
    where
        T: Iterator,
        K: Eq + Hash,
        F: Fn(&T::Item) -> K,
        C: Fn(&T::Item, &T::Item) -> std::cmp::Ordering,
        G: Fn(&T::Item) -> O,
    {
        let mut aux: HashMap<K, T::Item> = HashMap::new();
        for item in iterator {
            let key = key(&item);
            match aux.entry(key) {
                Entry::Occupied(mut entry) => {
                    if comparator(&item, entry.get()) == type_ord {
                        entry.insert(item);
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert(item);
                }
            }
        }
        aux.into_iter()
            .map(|(key, value)| (key, finisher(&value)))
            .collect()
    }
}

impl<T: Iterator> GroupingBy for T {
    type GItem = T::Item;
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

    fn grouping_by_max<K, F, G, O, C>(self, key: F, comparator: C, finisher: G) -> HashMap<K, O>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        C: Fn(&Self::GItem, &Self::GItem) -> std::cmp::Ordering,
        G: Fn(&Self::GItem) -> O,
    {
        utilities::grouping_by_min_max_aux(
            self,
            key,
            comparator,
            finisher,
            std::cmp::Ordering::Greater,
        )
    }

    fn grouping_by_min<K, F, G, O, C>(self, key: F, comparator: C, finisher: G) -> HashMap<K, O>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        C: Fn(&Self::GItem, &Self::GItem) -> std::cmp::Ordering,
        G: Fn(&Self::GItem) -> O,
    {
        utilities::grouping_by_min_max_aux(
            self,
            key,
            comparator,
            finisher,
            std::cmp::Ordering::Less,
        )
    }

    fn grouping_by_summing<K, V, F, G>(self, key: F, value: G) -> HashMap<K, V>
    where
        K: Eq + Hash,
        F: Fn(&Self::GItem) -> K,
        G: Fn(&Self::GItem) -> V,
        V: Default + std::ops::AddAssign,
    {
        let mut map = HashMap::new();
        for item in self {
            let v = map.entry(key(&item)).or_default();
            *v += value(&item);
        }
        map
    }
}
