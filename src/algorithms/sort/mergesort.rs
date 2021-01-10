use std::fmt::Debug;

pub fn merge_sort<T: Debug + Ord + Clone + Copy>(arr: &mut [T]) {
    let middle = arr.len() / 2;
    if middle == 0 {
        return;
    }
    merge_sort(&mut arr[..middle]);
    merge_sort(&mut arr[middle..]);

    let mut ret = arr.to_vec();
    merge(&arr[..middle], &arr[middle..], &mut ret);

    arr.copy_from_slice(&ret);
}

pub fn merge_sort_bottom_up<T: Debug + Ord + Clone + Copy>(arr: &mut [T]) {
    let mut width = 1;

    let mut ret = arr.to_vec();
    let len = arr.len();

    while width < len {
        let mut i = 0;
        while i < len {
            let upper = std::cmp::min(i + 2 * width, len);
            let mid = std::cmp::min(i + width, len);

            merge(&arr[i..mid], &arr[mid..upper], &mut ret[i..upper]);
            arr[i..upper].copy_from_slice(&ret[i..upper]);

            i += 2 * width;
        }
        width *= 2;
    }
}

fn merge<T: Ord + Debug + Copy>(first: &[T], second: &[T], result: &mut [T]) {
    let mut idx_first = 0;
    let mut idx_second = 0;
    let mut idx_res = 0;

    while idx_first < first.len() && idx_second < second.len() {
        if first[idx_first] <= second[idx_second] {
            result[idx_res] = first[idx_first];
            idx_res += 1;
            idx_first += 1;
        } else {
            result[idx_res] = second[idx_second];
            idx_res += 1;
            idx_second += 1;
        }
    }

    if idx_first < first.len() {
        result[idx_res..].copy_from_slice(&first[idx_first..]);
    }
    if idx_second < second.len() {
        result[idx_res..].copy_from_slice(&second[idx_second..]);
    }
}

pub fn run_merge_sort_algorithm() {
    let mut arr: Vec<u8> = vec![3, 5, 8, 2, 6, 9, 4, 1, 7];
    let mut arr2 = arr.clone();
    println!("before sort:\t {:?}", arr);

    merge_sort(&mut arr);
    println!("after sort:\t {:?}", arr);

    merge_sort_bottom_up(&mut arr2);
    println!("bottomup merge:\t {:?}", arr2);
}
