
#[allow(dead_code)]
pub fn quick_sort<T: PartialOrd>(array: &mut [T]) {
    if array.is_empty() {
        return;
    }

    let pivot = partition(array);
    let len = array.len();

    quick_sort(&mut array[0..pivot]);
    quick_sort(&mut array[pivot + 1..len]);
}

fn partition<T: PartialOrd>(array: &mut [T]) -> usize {
    let len = array.len();

    let mut i = 0;
    let mut j = 0;

    while j < len - 1 {
        if array[j] <= array[len - 1] {
            array.swap(i, j);
            i += 1;
        }
        j += 1
    }

    array.swap(i, len - 1);
    
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4,6,1,8,11,13,3];
        
        quick_sort(&mut v);
        assert_eq!(v, vec![1,3,4,6,8,11,13]);
    }
}