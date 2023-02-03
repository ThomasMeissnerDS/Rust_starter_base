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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 3, 4, 6, 11, 13]);
    }
}

// run cargo test to see the results