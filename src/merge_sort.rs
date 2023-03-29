
#[allow(dead_code)]
pub fn merge_sort<T: PartialOrd + Copy>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let mid = slice.len() / 2;

    merge_sort(&mut slice[..mid]);
    merge_sort(&mut slice[mid..]);

    let mut aux = slice.to_vec();
    let (mut i, mut j, mut k) = (0, mid, 0);

    while i < mid && j < slice.len() {
        if slice[i] <= slice[j] {
            aux[k] = slice[i];
            i += 1;
        } else {
            aux[k] = slice[j];
            j += 1;
        }
        k += 1;
    }

    while i < mid {
        aux[k] = slice[i];
        i += 1;
        k += 1;
    }

    while j < slice.len() {
        aux[k] = slice[j];
        j += 1;
        k += 1;
    }

    slice.copy_from_slice(&aux);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        let mut v1 = vec!["beach", "hotel", "airplane", "car", "house", "art"];

        merge_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);

        merge_sort(&mut v1);
        assert_eq!(v1, vec!["airplane", "art", "beach", "car", "hotel", "house"]);
    }
}
