fn quicksort_helper(arr: &mut Vec<i32>, low: usize, high: usize) {
    let pi = partition(arr, low, high);

    if low + 1 < pi {
        quicksort_helper(arr, low, pi - 1);
    }

    if pi + 1 < high {
        quicksort_helper(arr, pi + 1, high);
    }
}

fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = arr[high];

    let mut i: usize = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);

    return i;
}

#[test]
fn sorts_array() {
    let mut arr = vec![4, 1, 89, 23, 43, 5];

    quicksort(&mut arr);

    let expect = vec![1, 4, 5, 23, 43, 89];

    assert_eq!(arr, expect);
}

#[test]
fn sorts_empty_array() {
    let mut arr = vec![];

    quicksort(&mut arr);

    let expect = vec![];

    assert_eq!(arr, expect);
}

#[test]
fn sorts_array_one_element() {
    let mut arr = vec![1];

    quicksort(&mut arr);

    let expect = vec![1];

    assert_eq!(arr, expect);
}

pub fn quicksort(arr: &mut Vec<i32>) {
    if arr.len() < 1 {
        return;
    }

    quicksort_helper(arr, 0, arr.len() - 1);
}
