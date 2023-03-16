
use std::fmt::Debug;

// O(n2)
#[allow(dead_code)]
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for i in 0..v.len() {
        println!("{:?}", v);
        let mut sorted = true;
        for j in 0..(v.len() - 1) - i    {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
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
        let mut v = vec![4,6,1,8,11,13,3];

        bubble_sort(&mut v);
        assert_eq!(v, vec![1,3,4,6,8,11,13]);
    }
}