use std::cmp::Ordering;

// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}
impl<T: Ord> BinarySearch<T> for [T] {
    // log(n)
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    // log(n)
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

pub trait BinarySearchBy<T> {
    fn lower_bound_by<F>(&self, x: &T, compare: F) -> usize
    where
        F: FnMut(&T, &T) -> Ordering;
    fn upper_bound_by<F>(&self, x: &T, compare: F) -> usize
    where
        F: FnMut(&T, &T) -> Ordering;
}
impl<T: std::any::Any>  BinarySearchBy<T> for [T] {
    fn lower_bound_by<F>(&self, x: &T, mut compare: F) -> usize
    where
        F: FnMut(&T, &T) -> Ordering
    {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match compare(&self[mid], x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound_by<F>(&self, x: &T, mut compare: F) -> usize
    where
        F: FnMut(&T, &T) -> Ordering
    {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match compare(&self[mid], x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

pub trait BinarySearchByKey<T> {
    fn lower_bound_by_key<K, F>(&self, x: &T, key: F) -> usize
    where
        F: FnMut(&T) -> K,
        K: Ord;
    fn upper_bound_by_key<K, F>(&self, x: &T, key: F) -> usize
    where
        F: FnMut(&T) -> K,
        K: Ord;
}
impl<T: std::any::Any>  BinarySearchByKey<T> for [T] {
    fn lower_bound_by_key<K, F>(&self, x: &T, mut key: F) -> usize
    where
        F: FnMut(&T) -> K,
        K: Ord
    {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match key(&self[mid]).cmp(&key(x)) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
    fn upper_bound_by_key<K, F>(&self, x: &T, mut key: F) -> usize
    where
        F: FnMut(&T) -> K,
        K: Ord
    {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match key(&self[mid]).cmp(&key(x)) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

fn main() {
    let v = vec![3, 4, 23, 422, 1343, 0];
    println!("{:?}", v.upper_bound(&23));
}