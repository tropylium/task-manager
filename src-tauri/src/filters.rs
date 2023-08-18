use std::collections::HashSet;
use std::hash::Hash;
use serde::{Serialize, Deserialize};

/// Generic filter on a type.
pub trait ApplyFilter<T> {
    fn passes(&self, value: &T) -> bool;
}

#[derive(Serialize, Deserialize)]
pub struct ExactlyFilter<T: PartialEq> {
    pub value: T,
}
impl<T: PartialEq> ApplyFilter<T> for ExactlyFilter<T> {
    fn passes(&self, value: &T) -> bool {
        self.value.eq(value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ContainsStringFilter {
    pub pattern: String
}
impl ApplyFilter<String> for ContainsStringFilter {
    fn passes(&self, string: &String) -> bool {
        string.contains(&self.pattern)
    }
}

#[derive(Serialize, Deserialize)]
pub enum OptionalFilter<T> {
    OnlySome(T),
    OnlyNone,
}
impl<T, F: ApplyFilter<T>> ApplyFilter<Option<T>> for OptionalFilter<F> {
    fn passes(&self, value: &Option<T>) -> bool {
        match self {
            OptionalFilter::OnlySome(filter) => {
                value.as_ref().is_some_and(|value| filter.passes(&value))
            }
            OptionalFilter::OnlyNone => value.is_none()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SetFilter<T> {
    #[serde(bound(deserialize = "T: 'de + Eq + Hash + Deserialize<'de>"))]
    pub set: HashSet<T>,
}
impl<T: Eq + Hash> ApplyFilter<T> for SetFilter<T> {
    fn passes(&self, value: &T) -> bool {
        self.set.contains(value)
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderedRangeFilter<T> {
    pub lower_bound: Option<T>,
    pub upper_bound: Option<T>,
}
impl<T: PartialOrd> ApplyFilter<T> for OrderedRangeFilter<T> {
    fn passes(&self, value: &T) -> bool {
        match (self.lower_bound.as_ref(), self.upper_bound.as_ref()) {
            (Some(lower), Some(upper)) => value >= lower && value <= upper,
            (Some(lower), None) => value >= lower,
            (None, Some(upper)) => value <= upper,
            (None, None) => true
        }
    }
}
