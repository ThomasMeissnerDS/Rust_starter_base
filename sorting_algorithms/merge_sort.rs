/*
Merge sort explained:
- we split our array into two arrays
 */

use std::fmt::Debug;

// this still needs almost O(n^2) operations
pub fn bubble_sort <T:PartialOrd> (v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in (0..v.len() - 1) - p { // in each iteration we can ignore the last value as it is the biggest one for sure
            if v[i] > v[i+1] {
                v.swap(i, i+1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

pub fn merge_sort <T:PartialOrd + Debug> (mut v: Vec<T>) -> Vec<T>{
    // sort the left half
    // sort the right half O(n * ln(n))
    // bring the sorted halfs together: O(n)
    let mut res = Vec::with_capacity(v.len());
    if v.len() <= 1 {
        return v
    }
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    b = merge_sort(b);

    // bring them together again
    let mut a_it = a.into_iter(); // this moves the pointer if value was bigger than in other array
    let mut b = b.into_iter(); // this moves the pointer if value was bigger than in other array
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peak {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap()); // take() takes an option and unwrap() removes the option layer
                        b_peek = b_it.next();
                    }
                    else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extent(a_it);
                    return res;
                }
            }
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extent(b_it);
                return res;
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_sort() {
        let v = vec![4, 6, 1, 8, 11, 13, 3];
        let v = merge_sort(v);
        assert_eq!(v, [1, 3, 4, 6, 11, 13]);
    }
}

// run cargo test to see the results