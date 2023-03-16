use std::{fmt::Debug    };

#[allow(dead_code)]
pub fn insertion_sort<T: PartialOrd + Debug>( v: &mut [T]) {
    for i in 0..v.len() {
    let mut j = i;
    while j > 0 && v[j-1] > v[j] {
        v.swap(j, j-1);
        j -= 1;
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort(){
        let mut v = vec![4,6,1,8,11,13,3];
        insertion_sort(&mut v);

        assert_eq!(v, vec![1,3,4,6,8,11,13])
    }
}