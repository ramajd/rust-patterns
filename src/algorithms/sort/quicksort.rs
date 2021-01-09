use std::fmt::Debug;

pub fn quick_sort<T: Debug, F>(v: &mut [T], f: &F)
where
    F: Fn(&T, &T) -> bool,
{
    //1. select pivot
    //2. sort before pivot
    //3. sort after pivot
    //done
    println!("   quick sort:\t {:?}", &v);

    let len = v.len();
    if len >= 2 {
        let p = partition(v, f);
        quick_sort(&mut v[0..p], f);
        quick_sort(&mut v[p + 1..len], f);
    }
}

fn partition<T: Debug, F>(v: &mut [T], f: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    let mut pivot_index = 0;

    for i in 1..len {
        if f(&v[i], &v[0]) {
            pivot_index += 1;
            v.swap(pivot_index, i);
        }
    }
    v.swap(0, pivot_index);
    println!("\t\t {:?}", &v);
    pivot_index
}

pub fn functional_quick_sort<T, E>(mut v: T) -> Vec<E>
where
    T: Iterator<Item = E>,
    E: PartialOrd + Debug,
{
    match v.next() {
        None => Vec::new(),
        Some(pivot) => {
            let (lower, higher): (Vec<_>, Vec<_>) = v.partition(|it| it < &pivot);
            println!("\t{:?} ({:?}) {:?}", &lower, pivot, &higher);
            let lower = functional_quick_sort(lower.into_iter());
            let higher = functional_quick_sort(higher.into_iter());
            lower
                .into_iter()
                .chain(core::iter::once(pivot))
                .chain(higher.into_iter())
                .collect()
        }
    }
}

pub fn run_quick_sort_algorithm() {
    let mut numbers: Vec<u8> = vec![1, 3, 9, 4, 7, 5, 8, 6, 2];
    let numbers2 = numbers.clone();
    println!("Before sort: {:?}", numbers);
    quick_sort(&mut numbers, &|a, b| a < b);
    println!("After sort: {:?}", numbers);

    println!("Functional quick sort");
    println!("Before: {:?}", numbers2);
    let result = functional_quick_sort(numbers2.into_iter());
    println!("After: {:?}", result);
}
