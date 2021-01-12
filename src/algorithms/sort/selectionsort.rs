use std::fmt::Debug;

pub fn selection_sort<T: Ord + Debug>(arr: &mut [T]) {
    let len = arr.len();
    if len > 0 {
        for i in 0..(len - 1) {
            let mut min = i;
            for j in (i + 1)..len {
                if arr[j] < arr[min] {
                    min = j;
                }
            }
            if i != min {
                arr.swap(i, min);
            }
        }
    }
}

pub fn run_selection_sort() {
    let mut numbers: Vec<u8> = vec![1, 3, 9, 4, 7, 5, 8, 6, 2];
    println!("before sort: {:?}", numbers);
    selection_sort(&mut numbers);
    println!("after sort : {:?}", numbers);
}
