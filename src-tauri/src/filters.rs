use std::collections::HashSet;
use std::hash::Hash;

/// Generic filter on a type.
pub trait ApplyFilter<T> {
    fn passes(&self, value: &T) -> bool;
}

pub struct ExactlyFilter<T: PartialEq> {
    pub value: T,
}
impl<T: PartialEq> ApplyFilter<T> for ExactlyFilter<T> {
    fn passes(&self, value: &T) -> bool {
        self.value.eq(value)
    }
}

pub struct ContainsStringFilter {
    pub pattern: String
}
impl ApplyFilter<String> for ContainsStringFilter {
    fn passes(&self, string: &String) -> bool {
        string.contains(&self.pattern)
    }
}

pub enum OptionalFilter<T> {
    AcceptsSome(T),
    AcceptsNone,
}
impl<T, F: ApplyFilter<T>> ApplyFilter<Option<T>> for OptionalFilter<F> {
    fn passes(&self, value: &Option<T>) -> bool {
        match self {
            OptionalFilter::AcceptsSome(filter) => {
                value.as_ref().is_some_and(|value| filter.passes(&value))
            }
            OptionalFilter::AcceptsNone => value.is_none()
        }
    }
}

pub struct SetFilter<T> {
    pub set: HashSet<T>,
}
impl<T: Eq + Hash> ApplyFilter<T> for SetFilter<T> {
    fn passes(&self, value: &T) -> bool {
        self.set.contains(value)
    }
}

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
