use std::fmt::Debug;

pub fn heap_sort<T: Ord + Debug>(array: &mut [T]) {
    if array.len() <= 1 {
        return; // already soretd
    }
    heapify(array);
    for end in (1..array.len()).rev() {
        array.swap(0, end);
        max_heapify(&mut array[..end], 0);
    }
}

fn heapify<T: Ord + Debug>(array: &mut [T]) {
    let len = array.len();
    for i in (0..len / 2).rev() {
        println!("   - {:?} - {}", array, i);
        max_heapify(array, i);
    }
}

fn max_heapify<T: Ord + Debug>(array: &mut [T], i: usize) {
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let pos = if left < array.len() && array[left] > array[i] {
        left
    } else {
        i
    };
    let pos = if right < array.len() && array[pos] < array[right] {
        right
    } else {
        pos
    };
    if i != pos {
        array.swap(i, pos);
        max_heapify(array, pos);
    }
}

pub fn run_heap_sort_algorithm() {
    let mut arr: Vec<u8> = vec![4, 6, 7, 8, 9, 2, 5, 1, 3];
    println!("before heapify:\t {:?}", arr);
    heap_sort(&mut arr);
    println!("after heapify:\t {:?}", arr);
}
