
#[allow(dead_code)]
pub fn selection_sort<T: PartialOrd>(v: &mut [T]) {
    for i in 0..v.len() {
        let mut min_j = i;
        
        for j in (i+1)..v.len() {
            if v[j] < v[min_j] {
                min_j = j
            }
        }

        if min_j != i {
            v.swap(i, min_j)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort(){
        let mut v = vec![4,6,1,8,11,13,3];
        selection_sort(&mut v);

        assert_eq!(v, vec![1,3,4,6,8,11,13])
    }
}


