use std::fmt::Debug;

pub fn insertion_sort<T: Ord + Debug>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn run_insertion_sort_algorithm() {
    let mut numbers: Vec<u8> = vec![1, 3, 9, 4, 7, 5, 8, 6, 2];
    println!("before sort: {:?}", numbers);
    insertion_sort(&mut numbers);
    println!("after sort : {:?}", numbers);
}
