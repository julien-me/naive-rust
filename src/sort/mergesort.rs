fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut sorted_sub_array = vec![];
    let mut i = 0;
    let mut j = 0;

    let left_len = left.len();
    let right_len = right.len();

    while i < left_len && j < right_len {
        if left[i] < right[j] {
            sorted_sub_array.push(left[i]);
            i += 1;
        } else {
            sorted_sub_array.push(right[j]);
            j += 1;
        }
    }

    if i == left_len {
        for g in j..right_len {
            sorted_sub_array.push(right[g]);
        }
    } else {
        for g in i..left_len {
            sorted_sub_array.push(left[g]);
        }
    }

    sorted_sub_array
}

#[test]
fn merge_arrays() {
    let left = vec![1, 3];
    let right = vec![2, 4];

    let merged = merge(left, right);

    let expected = vec![1, 2, 3, 4];

    assert_eq!(merged, expected);
}

fn merge_sort_helper(arr: &Vec<i32>, low: usize, high: usize) -> Vec<i32> {
    if low == high {
        let single_elem_arr = vec![arr[low]];
        return single_elem_arr;
    }

    let mid = (high + low) / 2;
    let left = merge_sort_helper(arr, low, mid);
    let right = merge_sort_helper(arr, mid + 1, high);

    let merged_arr = merge(left, right);

    merged_arr
}

#[test]
fn sort_array() {
    let unsorted: Vec<i32> = vec![5, 3, 1, 2];

    let sorted = merge_sort(&unsorted);

    let expected = vec![1, 2, 3, 5];

    assert_eq!(sorted, expected);
}

#[test]
fn sort_empty_array() {
    let unsorted: Vec<i32> = vec![];

    let sorted = merge_sort(&unsorted);

    let expected = vec![];

    assert_eq!(sorted, expected);
}

#[test]
fn sort_single_elem_array() {
    let unsorted: Vec<i32> = vec![1];

    let sorted = merge_sort(&unsorted);

    let expected = vec![1];

    assert_eq!(sorted, expected);
}

pub fn merge_sort(arr: &Vec<i32>) -> Vec<i32> {
    if arr.len() < 1 {
        return vec![];
    }

    merge_sort_helper(arr, 0, arr.len() - 1)
}
