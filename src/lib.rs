pub use crate::bisect_right as bisect;
pub use crate::insort_right as insort;

use std::cmp::Ordering;
use std::ops::{Bound::*, RangeBounds};

// TODO: Doctest examples

/// Insert `x` in `a[within]`, keeping it sorted assuming `a` is sorted.
///
/// If `a` contains `x`, insert it just *after* the *rightmost* occurence of `x`.
///
/// # Panics
///
/// Panics if `within` is out of bounds of `a`.
pub fn insort_right_slice<T, I>(a: &mut Vec<T>, x: T, within: I)
where
    I: RangeBounds<usize>,
    T: Ord,
{
    insort_right_slice_by(a, x, within, T::cmp);
}

/// Insert `x` in `a`, keeping it sorted assuming `a` is sorted.
/// If `a` contains `x`, insert it just *after* the *rightmost* occurence of `x`.
pub fn insort_right<T>(a: &mut Vec<T>, x: T)
where
    T: PartialOrd,
{
    let lo = bisect_right_slice(a, &x,..);
    a.insert(lo, x);
}

/// Insert `x` in `a`, keeping it sorted, assuming `a` is sorted, according to a comparator
/// function.
///
/// The comparator function should implement an order consistent with the sort order of the
/// underlying slice.
///
/// If `a` contains `x`, insert it just *after* the *rightmost* occurence of `x`.
pub fn insort_right_by<T, F>(a: &mut Vec<T>, x: T, f: F)
where
    T: Ord,
    F: FnMut(&T, &T) -> Ordering,
{
    insort_right_slice_by(a, x, .., f);
}

/// Insert x in `a[within]`, keeping it sorted, assuming `a` is sorted, according to a comparator
/// function.
///
/// The comparator function should implement an order consistent with the sort order of the
/// underlying slice.
///
/// If `a` contains `x`, insert it just *after* the *rightmost* occurence of `x`.
///
/// # Panics
///
/// Panics if `within` is out of bounds of `a`.
pub fn insort_right_slice_by<T, I, F>(a: &mut Vec<T>, x: T, within: I, mut f: F)
where
    I: RangeBounds<usize>,
    F: FnMut(&T, &T) -> Ordering,
{
    let lo = bisect_right_slice_by(a, within, |p| f(&x, p));
    a.insert(lo, x);
}

/// Return the index where `x` should be inserted in `a[within]`, assuming `a`
/// is sorted.
///
/// The return value `i` is such that all `e` in `a[..i]` have `e <= x`, and
/// all `e` in `a[i..]` have `e > x`.
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *after* the
///   *rightmost* `x`.
///
/// # Panics
///
/// Panics if `within` is out of bounds of `a`.
pub fn bisect_right_slice<T, I>(a: &[T], x: &T, within: I) -> usize
where
    I: RangeBounds<usize>,
    T: PartialOrd,
{
    let (mut lo, mut hi) = bounds_to_indices(a, within);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if *x < a[mid] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

/// Return the index where `x` should be inserted in `a`, assuming `a` is sorted.
///
/// The return value `i` is such that all `e` in `a[..i]` have `e <= x`, and
/// all `e` in `a[i..]` have `e > x`.
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *after* the
///   *rightmost* occurence of `x`.
pub fn bisect_right<T>(a: &[T], x: &T) -> usize
where
    T: PartialOrd,
{
    bisect_right_slice(a, x, ..)
}

/// Return the index where `x` should be inserted in `a`, assuming `a` is sorted, according to
/// a comparator function.
///
/// The comparator function should implement an order consistent with the sort order of the
/// underlying slice, returning an order code that indicates whethers its argument is `Less`,
/// `Equal` or `Greater` that the **desired target**.
///
/// The return value `i` is such that all `e` in `a[..i]` have `f(e) == Less | f(e) == Equal`, and
/// all `e` in `a[i..]` have `f(e) == Greater`.
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *after* the
///   *rightmost* occurence of `x`.
pub fn bisect_right_by<T, F>(a: &[T], f: F) -> usize
where
    F: FnMut(&T) -> Ordering,
{
    bisect_right_slice_by(a, .., f)
}

/// Return the index where a value should be inserted in `a[within]`, assuming it sorted,
/// according to a comparator function.
///
/// The comparator function should implement an order consistent with the sort order of the
/// underlying slice, returning an order code that indicates whethers its argument is `Less`,
/// `Equal` or `Greater` that the **desired target**.
///
/// The return value `i` is such that all `e` in `a[..i]` have `f(e) == Less | f(e) == Equal`, and
/// all `e` in `a[i..]` have `f(e) == Greater`.
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *after* the
///   *rightmost* occurence of `x`.
///
/// # Panics
///
/// Panics if `within` is out of bounds of `a`.
pub fn bisect_right_slice_by<T, I, F>(a: &[T], within: I, mut f: F) -> usize
where
    I: RangeBounds<usize>,
    F: FnMut(&T) -> Ordering,
{
    let (mut lo, mut hi) = bounds_to_indices(a, within);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if f(&a[mid]) == Ordering::Less {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

/// Insert `x` in `a[within]`, keeping it sorted assuming `a` is sorted.
///
/// If `a` contains `x`, insert it just *before* the *leftmost* occurence of `x`.
///
/// # Panics
///
/// Panics if `within` is out of bounds of `a`.
pub fn insort_left_slice<T, I>(a: &mut Vec<T>, x: T, within: I)
where
    I: RangeBounds<usize>,
    T: Ord,
{
    insort_left_slice_by(a, x, within, T::cmp);
}

/// Insert `x` in `a`, keeping it sorted assuming `a` is sorted.
///
/// If `a` contains `x`, insert it just *before* the *leftmost* occurence of `x`.
pub fn insort_left<T>(a: &mut Vec<T>, x: T)
where
    T: PartialOrd,
{
    let lo = bisect_left_slice(a, &x,..);
    a.insert(lo, x);
}

/// Insert `x` in `a`, keeping it sorted, assuming `a` is sorted, according to a comparator
/// function.
///
/// The comparator function should implement an order consistent with the sort order of the
/// underlying slice.
///
/// If `a` contains `x`, insert it just *before* the *leftmost* occurence of `x`.
pub fn insort_left_by<T, F>(a: &mut Vec<T>, x: T, f: F)
where
    T: Ord,
    F: FnMut(&T, &T) -> Ordering,
{
    insort_left_slice_by(a, x, .., f);
}

/// Insert x in `a[within]`, keeping it sorted, assuming `a` is sorted, according to a comparator
/// function.
///
/// The comparator function should implement an order consistent with the sort order of the
/// underlying slice.
///
/// If `a` contains `x`, insert it just *before* the *leftmost* occurence of `x`.
///
/// # Panics
///
/// Panics if `within` is out of bounds of `a`.
pub fn insort_left_slice_by<T, I, F>(a: &mut Vec<T>, x: T, within: I, mut f: F)
where
    I: RangeBounds<usize>,
    F: FnMut(&T, &T) -> Ordering,
{
    let lo = bisect_right_slice_by(a, within, |p| f(&x, p));
    a.insert(lo, x);
}

/// Return the index where `x` should be inserted in `a[within]`, assuming `a`
/// is sorted.
///
/// The return value `i` is such that all `e` in `a[..i]` have `e < x`, and
/// all `e` in `a[i..]` have `e >= x`.
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *before* the
///   *leftmost* `x`.
///
/// # Panics
///
/// Panics if `within` is out of bounds of `a`.
pub fn bisect_left_slice<T, I>(a: &[T], x: &T, within: I) -> usize
where
    I: RangeBounds<usize>,
    T: PartialOrd,
{
    let (mut lo, mut hi) = bounds_to_indices(a, within);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] < *x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// Return the index where `x` should be inserted in `a`, assuming `a` is sorted.
///
/// The return value `i` is such that all `e` in `a[..i]` have `e < x`, and
/// all `e` in `a[i..]` have `e >= x`.
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *before* the
///   *leftmost* `x`.
pub fn bisect_left<T>(a: &[T], x: &T) -> usize
where
    T: PartialOrd,
{
    bisect_left_slice(a, x, ..)
}

/// Return the index where a value should be inserted in `a`, assuming `a` is sorted, according to
/// a comparator function.
///
/// The comparator function should implement an order consistent with the sort order of the
/// underlying slice, returning an order code that indicates whethers its argument is `Less`,
/// `Equal` or `Greater` that the **desired target**.
///
/// The return value `i` is such that all `e` in `a[..i]` have `f(e) == Less`, and
/// all `e` in `a[i..]` have `f(e) == Greater | f(e) == Equal`
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *before* the
///   *leftmost* `x`.
pub fn bisect_left_by<T, F>(a: &[T], f: F) -> usize
where
    F: FnMut(&T) -> Ordering,
{
    bisect_left_slice_by(a, .., f)
}

/// Return the index where a value should be inserted in `a[within]`, assuming it sorted,
/// according to a comparator function.
///
/// The comparator function should implement an order consistent with the sort order of the
/// underlying slice, returning an order code that indicates whethers its argument is `Less`,
/// `Equal` or `Greater` that the **desired target**.
///
/// The return value `i` is such that all `e` in `a[..i]` have `f(e) == Less`, and
/// all `e` in `a[i..]` have `f(e) == Greater | f(e) == Equal`
/// - If `a` contains `x`, `a.insert(i, x)` will insert just *before* the
///   *leftmost* `x`.
/// # Panics
///
/// Panics if `within` is out of bounds of `a`.
pub fn bisect_left_slice_by<T, I, F>(a: &[T], within: I, mut f: F) -> usize
where
    I: RangeBounds<usize>,
    F: FnMut(&T) -> Ordering,
{
    let (mut lo, mut hi) = bounds_to_indices(a, within);
    while lo < hi {
        let mid = (lo + hi) / 2;
        let cmp = f(&a[mid]);
        if cmp == Ordering::Less {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// Convert bounds to a `(lo, hi)`  pair for indexing into a slice of `a`.
///
/// # Panics
///
/// Panics if `within` is out of bounds of `a`.
fn bounds_to_indices<T, I>(a: &[T], within: I) -> (usize, usize)
where
    I: RangeBounds<usize>,
{
    // TODO: Panics
    let lo = match within.start_bound() {
        Unbounded => 0,
        Included(i) => *i,
        Excluded(i) => i + 1,
    };

    let hi = match within.end_bound() {
        Unbounded => a.len(),
        Included(i) => i + 1,
        Excluded(i) => *i,
    };

    if hi > a.len() {
        panic!("index out of bounds")
    }

    (lo, hi)
}

#[cfg(test)]
mod tests {

    use super::*;
    use proptest::prelude::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[derive(Clone, Debug)]
    struct BisectTest<T: 'static>
    where
        T: PartialOrd,
    {
        name: &'static str,
        a: &'static [T],
        x: T,
        expected_index: usize,
    }

    #[derive(Debug, Clone)]
    enum TestDirection {
        Left,
        Right,
    }

    type TestCollection<T> = &'static [BisectTest<T>];

    macro_rules! t {
        ($name:ident, $a:expr, $x:expr, $expected_index:expr) => {
            BisectTest {
                name: stringify!($name),
                a: $a,
                x: $x,
                expected_index: $expected_index,
            }
        };
    }

    const RIGHT_INT_CASES: TestCollection<i32> = &[
        t!(ints_right_0, &[], 1, 0),
        t!(ints_right_1, &[1], 0, 0),
        t!(ints_right_2, &[1], 1, 1),
        t!(ints_right_3, &[1], 2, 1),
        t!(ints_right_4, &[1, 1], 0, 0),
        t!(ints_right_5, &[1, 1], 1, 2),
        t!(ints_right_6, &[1, 1], 2, 2),
        t!(ints_right_7, &[1, 1, 1], 0, 0),
        t!(ints_right_8, &[1, 1, 1], 1, 3),
        t!(ints_right_9, &[1, 1, 1], 2, 3),
        t!(ints_right_10, &[1, 1, 1, 1], 0, 0),
        t!(ints_right_11, &[1, 1, 1, 1], 1, 4),
        t!(ints_right_12, &[1, 1, 1, 1], 2, 4),
        t!(ints_right_13, &[1, 2], 0, 0),
        t!(ints_right_14, &[1, 2], 1, 1),
        t!(ints_right_15, &[1, 2], 2, 2),
        t!(ints_right_16, &[1, 2], 3, 2),
        t!(ints_right_17, &[1, 1, 2, 2], 0, 0),
        t!(ints_right_18, &[1, 1, 2, 2], 1, 2),
        t!(ints_right_19, &[1, 1, 2, 2], 2, 4),
        t!(ints_right_20, &[1, 1, 2, 2], 3, 4),
        t!(ints_right_21, &[1, 2, 3], 0, 0),
        t!(ints_right_22, &[1, 2, 3], 1, 1),
        t!(ints_right_23, &[1, 2, 3], 2, 2),
        t!(ints_right_24, &[1, 2, 3], 3, 3),
        t!(ints_right_25, &[1, 2, 3], 4, 3),
        t!(ints_right_26, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 0, 0),
        t!(ints_right_27, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 1, 1),
        t!(ints_right_28, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 2, 3),
        t!(ints_right_29, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 3, 6),
        t!(ints_right_30, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 4, 10),
        t!(ints_right_31, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 5, 10),
    ];

    const LEFT_INT_CASES: TestCollection<i32> = &[
        t!(ints_left_0, &[], 1, 0),
        t!(ints_left_1, &[1], 0, 0),
        t!(ints_left_2, &[1], 1, 0),
        t!(ints_left_3, &[1], 2, 1),
        t!(ints_left_4, &[1, 1], 0, 0),
        t!(ints_left_5, &[1, 1], 1, 0),
        t!(ints_left_6, &[1, 1], 2, 2),
        t!(ints_left_7, &[1, 1, 1], 0, 0),
        t!(ints_left_8, &[1, 1, 1], 1, 0),
        t!(ints_left_9, &[1, 1, 1], 2, 3),
        t!(ints_left_10, &[1, 1, 1, 1], 0, 0),
        t!(ints_left_11, &[1, 1, 1, 1], 1, 0),
        t!(ints_left_12, &[1, 1, 1, 1], 2, 4),
        t!(ints_left_13, &[1, 2], 0, 0),
        t!(ints_left_14, &[1, 2], 1, 0),
        t!(ints_left_15, &[1, 2], 2, 1),
        t!(ints_left_16, &[1, 2], 3, 2),
        t!(ints_left_17, &[1, 1, 2, 2], 0, 0),
        t!(ints_left_18, &[1, 1, 2, 2], 1, 0),
        t!(ints_left_19, &[1, 1, 2, 2], 2, 2),
        t!(ints_left_20, &[1, 1, 2, 2], 3, 4),
        t!(ints_left_21, &[1, 2, 3], 0, 0),
        t!(ints_left_22, &[1, 2, 3], 1, 0),
        t!(ints_left_23, &[1, 2, 3], 2, 1),
        t!(ints_left_24, &[1, 2, 3], 3, 2),
        t!(ints_left_25, &[1, 2, 3], 4, 3),
        t!(ints_left_26, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 0, 0),
        t!(ints_left_27, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 1, 0),
        t!(ints_left_28, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 2, 1),
        t!(ints_left_29, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 3, 3),
        t!(ints_left_30, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 4, 6),
        t!(ints_left_31, &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4], 5, 10),
    ];

    const RIGHT_FLOAT_CASES: TestCollection<f32> = &[
        t!(floats_right_0, &[], 1.0, 0),
        t!(floats_right_1, &[1.0], 0.0, 0),
        t!(floats_right_2, &[1.0], 1.0, 1),
        t!(floats_right_3, &[1.0], 2.0, 1),
        t!(floats_right_4, &[1.0, 1.0], 0.0, 0),
        t!(floats_right_5, &[1.0, 1.0], 1.0, 2),
        t!(floats_right_6, &[1.0, 1.0], 2.0, 2),
        t!(floats_right_7, &[1.0, 1.0, 1.0], 0.0, 0),
        t!(floats_right_8, &[1.0, 1.0, 1.0], 1.0, 3),
        t!(floats_right_9, &[1.0, 1.0, 1.0], 2.0, 3),
        t!(floats_right_10, &[1.0, 1.0, 1.0, 1.0], 0.0, 0),
        t!(floats_right_11, &[1.0, 1.0, 1.0, 1.0], 1.0, 4),
        t!(floats_right_12, &[1.0, 1.0, 1.0, 1.0], 2.0, 4),
        t!(floats_right_13, &[1.0, 2.0], 0.0, 0),
        t!(floats_right_14, &[1.0, 2.0], 1.0, 1),
        t!(floats_right_15, &[1.0, 2.0], 2.0, 2),
        t!(floats_right_16, &[1.0, 2.0], 3.0, 2),
        t!(floats_right_17, &[1.0, 1.0, 2.0, 2.0], 0.0, 0),
        t!(floats_right_18, &[1.0, 1.0, 2.0, 2.0], 1.0, 2),
        t!(floats_right_19, &[1.0, 1.0, 2.0, 2.0], 2.0, 4),
        t!(floats_right_20, &[1.0, 1.0, 2.0, 2.0], 3.0, 4),
        t!(floats_right_21, &[1.0, 2.0, 3.0], 0.0, 0),
        t!(floats_right_22, &[1.0, 2.0, 3.0], 1.0, 1),
        t!(floats_right_23, &[1.0, 2.0, 3.0], 2.0, 2),
        t!(floats_right_24, &[1.0, 2.0, 3.0], 3.0, 3),
        t!(floats_right_25, &[1.0, 2.0, 3.0], 4.0, 3),
        t!(floats_right_26, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 0.0, 0),
        t!(floats_right_27, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 1.0, 1),
        t!(floats_right_28, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 2.0, 3),
        t!(floats_right_29, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 3.0, 6),
        t!(floats_right_30, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 4.0, 10),
        t!(floats_right_31, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 5.0, 10),
    ];

    const LEFT_FLOAT_CASES: TestCollection<f32> = &[
        t!(floats_left_0, &[], 1.0, 0),
        t!(floats_left_1, &[1.0], 0.0, 0),
        t!(floats_left_2, &[1.0], 1.0, 0),
        t!(floats_left_3, &[1.0], 2.0, 1),
        t!(floats_left_4, &[1.0, 1.0], 0.0, 0),
        t!(floats_left_5, &[1.0, 1.0], 1.0, 0),
        t!(floats_left_6, &[1.0, 1.0], 2.0, 2),
        t!(floats_left_7, &[1.0, 1.0, 1.0], 0.0, 0),
        t!(floats_left_8, &[1.0, 1.0, 1.0], 1.0, 0),
        t!(floats_left_9, &[1.0, 1.0, 1.0], 2.0, 3),
        t!(floats_left_10, &[1.0, 1.0, 1.0, 1.0], 0.0, 0),
        t!(floats_left_11, &[1.0, 1.0, 1.0, 1.0], 1.0, 0),
        t!(floats_left_12, &[1.0, 1.0, 1.0, 1.0], 2.0, 4),
        t!(floats_left_13, &[1.0, 2.0], 0.0, 0),
        t!(floats_left_14, &[1.0, 2.0], 1.0, 0),
        t!(floats_left_15, &[1.0, 2.0], 2.0, 1),
        t!(floats_left_16, &[1.0, 2.0], 3.0, 2),
        t!(floats_left_17, &[1.0, 1.0, 2.0, 2.0], 0.0, 0),
        t!(floats_left_18, &[1.0, 1.0, 2.0, 2.0], 1.0, 0),
        t!(floats_left_19, &[1.0, 1.0, 2.0, 2.0], 2.0, 2),
        t!(floats_left_20, &[1.0, 1.0, 2.0, 2.0], 3.0, 4),
        t!(floats_left_21, &[1.0, 2.0, 3.0], 0.0, 0),
        t!(floats_left_22, &[1.0, 2.0, 3.0], 1.0, 0),
        t!(floats_left_23, &[1.0, 2.0, 3.0], 2.0, 1),
        t!(floats_left_24, &[1.0, 2.0, 3.0], 3.0, 2),
        t!(floats_left_25, &[1.0, 2.0, 3.0], 4.0, 3),
        t!(floats_left_26, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 0.0, 0),
        t!(floats_left_27, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 1.0, 0),
        t!(floats_left_28, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 2.0, 1),
        t!(floats_left_29, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 3.0, 3),
        t!(floats_left_30, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 4.0, 6),
        t!(floats_left_31, &[1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0], 5.0, 10),
    ];

    #[test]
    fn bisect_right_precomputed() {
        run_bisect_tests(TestDirection::Right, RIGHT_INT_CASES);
        run_bisect_tests(TestDirection::Right, RIGHT_FLOAT_CASES);
    }

    #[test]
    fn bisect_left_precomputed() {
        run_bisect_tests(TestDirection::Left, LEFT_INT_CASES);
        run_bisect_tests(TestDirection::Left, LEFT_FLOAT_CASES);
    }

    #[test]
    fn bisect_right_slice_precomputed() {
        run_bisect_slice_tests(TestDirection::Right, RIGHT_INT_CASES);
        run_bisect_slice_tests(TestDirection::Right, RIGHT_FLOAT_CASES);
    }

    #[test]
    fn bisect_left_slice_precomputed() {
        run_bisect_slice_tests(TestDirection::Left, LEFT_INT_CASES);
        run_bisect_slice_tests(TestDirection::Left, LEFT_FLOAT_CASES);
    }

    #[test]
    #[should_panic]
    fn right_slice_index_out_of_bounds() {
        let a: Vec<u32> = (0..10).collect();
        // 10 does not fit within 5..10 so the search goes into the out of bounds range
        bisect_right_slice(&a, &10, 5..15);
    }

    #[test]
    #[should_panic]
    fn left_slice_index_out_of_bounds() {
        let a: Vec<u32> = (0..10).collect();
        // 10 does not fit within 5..10 so the search goes into the out of bounds range
        bisect_left_slice(&a, &10, 5..15);
    }

    #[test]
    #[should_panic]
    fn right_slice_index_out_of_bounds_when_not_searched() {
        let a: Vec<u32> = (0..10).collect();
        // 5 fits within 0..10 bounds so the search *doesnt* actually index into the out of bounds
        bisect_right_slice(&a, &5, ..15);
    }

    #[test]
    #[should_panic]
    fn left_slice_index_out_of_bounds_when_not_searched() {
        let a: Vec<u32> = (0..10).collect();
        // 5 fits within 0..10 bounds so the search *doesnt* actually index into the out of bounds
        bisect_left_slice(&a, &5, ..15);
    }

    fn run_bisect_tests<T: Clone + PartialOrd>(direction: TestDirection, test_cases: TestCollection<T>) {
        let bisect_func = match direction {
            TestDirection::Left => bisect_left,
            TestDirection::Right => bisect_right,
        };

        for test_case in test_cases {
            let data = test_case.a.to_vec();
            assert_eq!(test_case.expected_index, bisect_func(&data, &test_case.x));
        }
    }

    fn run_bisect_slice_tests<T: Clone + PartialOrd>(
        direction: TestDirection,
        test_cases: TestCollection<T>,
    ) {
        let bisect_func = match direction {
            TestDirection::Left => bisect_left_slice,
            TestDirection::Right => bisect_right_slice,
        };

        for test_case in test_cases {
            let data = test_case.a.to_vec();
            for lo in 0..4 {
                for hi in 3..8 {
                    let hi = std::cmp::min(data.len(), hi);
                    let ip = bisect_func(&data, &test_case.x, lo..hi);

                    match direction {
                        TestDirection::Left => {
                            if ip < hi {
                                assert!(test_case.x <= data[ip]);
                            }

                            if ip > lo {
                                assert!(data[ip - 1] < test_case.x)
                            }
                        }
                        TestDirection::Right => {
                            if ip < hi {
                                assert!(test_case.x < data[ip]);
                            }

                            if ip > lo {
                                assert!(data[ip - 1] <= test_case.x)
                            }
                        }
                    }

                    assert_eq!(
                        ip,
                        std::cmp::max(lo, std::cmp::min(hi, test_case.expected_index))
                    );
                }
            }
        }
    }

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct Person {
        name: String,
        age: u32,
    }

    fn arb_person() -> impl Strategy<Value = Person> {
        ("[a-z]*", 1..100_u32).prop_map(|(name, age)| Person { name, age })
    }

    fn check_index_right_invariant<T, F>(a: &[T], target: &T, index: usize, mut f: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        // See `bisect_right_by` docs
        assert!(a[..index].iter().all(|x| match f(x, &target) {
            Ordering::Less | Ordering::Equal => true,
            _ => false,
        }));
        assert!(a[index..]
            .iter()
            .all(|x| f(x, &target) == Ordering::Greater));
    }

    fn check_index_left_invariant<T, F>(a: &[T], target: &T, index: usize, mut f: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        // See `bisect_left_by` docs
        assert!(a[..index].iter().all(|x| f(x, &target) == Ordering::Less));
        assert!(a[index..].iter().all(|x| match f(x, &target) {
            Ordering::Greater | Ordering::Equal => true,
            _ => false,
        }));
    }

    proptest! {

        #[test]
        fn test_bisect_left_index_invariant(
            mut nums in prop::collection::vec(any::<u32>(), 0..500),
            num in any::<u32>()
        ) {
            nums.sort();

            let i = bisect_left(&nums, &num);

            check_index_left_invariant(&nums, &num, i, u32::cmp);
        }

        #[test]
        fn test_bisect_left_by_index_invariant(
            mut people in prop::collection::vec(arb_person(), 0..500),
            new_person in arb_person()
        ) {
            // Sort by age only
            let f = |a: &Person, b: &Person| a.age.cmp(&b.age);

            people.sort_by(f);

            let i = bisect_left_by(&people, |p| f(p, &new_person));

            check_index_left_invariant(&people, &new_person, i, f);

            // Sort by name only
            let f = |a: &Person, b: &Person| a.name.cmp(&b.name);

            people.sort_by(f);

            let i = bisect_left_by(&people, |p| f(p, &new_person));

            check_index_left_invariant(&people, &new_person, i, f);
        }

        #[test]
        fn test_bisect_right_index_invariant(
            mut nums in prop::collection::vec(any::<u32>(), 0..500),
            num in any::<u32>()
        ) {
            nums.sort();

            let i = bisect_right(&nums, &num);

            check_index_right_invariant(&nums, &num, i, u32::cmp)
        }

        #[test]
        fn test_bisect_right_by_index_invariant(
            mut people in prop::collection::vec(arb_person(), 0..500),
            new_person in arb_person()
        ) {
            // Sort by age only
            let f = |a: &Person, b: &Person| a.age.cmp(&b.age);

            people.sort_by(f);

            let i = bisect_right_by(&people, |p| f(&new_person, p));

            check_index_right_invariant(&people, &new_person, i, f);


            // Sort by name only
            let f = |a: &Person, b: &Person| a.name.cmp(&b.name);

            people.sort_by(f);

            let i = bisect_right_by(&people, |p| f(&new_person, p));

            check_index_right_invariant(&people, &new_person, i, f);
        }

        #[test]
        fn test_insort_vs_vec_sort(
            digits in prop::collection::vec(0..10, 0..500)
        ) {
            let left_digits = HashSet::<i32>::from_iter(vec![0, 2, 4, 6, 8]);
            let mut insorted = vec![];

            for digit in digits {
                let f = if  left_digits.contains(&digit) {
                    insort_left
                } else {
                    insort_right
                };

                f(&mut insorted, digit);
            }

            let vec_sorted = {
                let mut v = insorted.clone();
                v.sort();
                v
            };

            assert_eq!(vec_sorted, insorted);
        }
    }
}
