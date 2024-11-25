use std::time::Instant;

fn nsquare_simulate(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len {
            arr.swap(i, j);
        }
    }
}

fn nlogn_simulate(arr: &mut [i32]) {
    let len = arr.len();

    for i in 0..len {
        let mut step = 1;

        while step < len {
            // Perform a dummy operation in each log step
            let _ = arr[i] + step as i32; // Dummy calculation
            step *= 2; // Mimic logarithmic growth
        }
    }
}

fn logn_simulate(arr: &mut [i32]) {
    let len = arr.len();
    let mut step = 1;

    while step < len {
        for i in 0..step {
            // This is a dummy operation on the array
            arr.swap(i, i); // No actual change, just to simulate work
        }
        step *= 2; // Doubling step size, simulating log(n) behavior
    }
}

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn my_selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in i + 1..len {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

fn selection_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len - 1 {
        let mut min_index = i;
        for j in i + 1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}

// QuickSort function
fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quicksort(&mut arr[0..pivot_index]);
    quicksort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1); // Move pivot to end
    let mut i = 0;

    for j in 0..arr.len() - 1 {
        if arr[j] < arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1); // Move pivot to correct place
    i
}

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);

    // Recursively sort the two halves
    merge_sort(left);
    merge_sort(right);

    // Use a temporary buffer to merge the two halves into the original array
    let mut temp = Vec::with_capacity(len);
    let mut left_idx = 0;
    let mut right_idx = 0;

    // Merge the two sorted halves into temp
    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            temp.push(left[left_idx]);
            left_idx += 1;
        } else {
            temp.push(right[right_idx]);
            right_idx += 1;
        }
    }

    // Push remaining elements from left or right
    temp.extend_from_slice(&left[left_idx..]);
    temp.extend_from_slice(&right[right_idx..]);

    // Copy the merged result back to the original array
    arr.copy_from_slice(&temp);
}

macro_rules! time_sort {
    ($sort_fn:expr, $arr:expr, $name:expr) => {{
        let start = Instant::now();
        $sort_fn(&mut $arr.clone());
        let duration = start.elapsed();
        println!("{} took {:?}", $name, duration);
    }};
}
//rewrite merge and quicksort

//test if copying the array before effects time
// macro_rules! test_time_sort {
//     ($sort_fn:expr, $arr:expr, $name:expr) => {{
//         let mut arr_copy = $arr.clone(); // Clone here, once per call to time_sort
//         let start = Instant::now();
//         $sort_fn(&mut arr_copy);
//         let duration = start.elapsed();
//         println!("{} took {:?}", $name, duration);
//     }};
// }

fn main() {
    let range: i32 = 10000;
    // Create a random array
    let unsorted_arr: Vec<i32> = (0..range)
        .map(|_| rand::random::<i32>().abs() % 100)
        .collect();
    println!("range of array: {}", range);
    //println!("init array {:?}", unsorted_arr);
    println!("{}", "-".repeat(50));

    time_sort!(nsquare_simulate, unsorted_arr, "nsquare_simulate");
    time_sort!(nlogn_simulate, unsorted_arr, "nlogn_simulate");
    time_sort!(logn_simulate, unsorted_arr, "logn_simulate");
    time_sort!(bubble_sort, unsorted_arr, "bubble_sort");
    time_sort!(my_selection_sort, unsorted_arr, "my_selection_sort");
    time_sort!(selection_sort, unsorted_arr, "selection_sort");
    time_sort!(quicksort, unsorted_arr, "quicksort");
    time_sort!(merge_sort, unsorted_arr, "mergesort");

    println!("{}", "-".repeat(50));
}
