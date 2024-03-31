fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low;

    for j in low..high {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, high);
    i
}

fn quick_select(arr: &mut [i32], low: usize, high: usize, k: usize) -> i32 {
    if low == high {
        return arr[low];
    }

    let pivot_index = partition(arr, low, high);

    if k == pivot_index {
        return arr[k];
    } else if k < pivot_index {
        return quick_select(arr, low, pivot_index - 1, k);
    } else {
        return quick_select(arr, pivot_index + 1, high, k);
    }
}

fn find_kth_smallest(arr: &mut [i32], k: usize) -> i32 {
    if k <= 0 || k > arr.len() {
        panic!("Invalid value of k");
    }

    let n = arr.len();
    quick_select(arr, 0, n - 1, k - 1)
}

fn main() {
    let mut arr = [7, 10, 4, 3, 20, 15];
    let k = 3;

    let kth_smallest = find_kth_smallest(&mut arr, k);
    println!("The {}th smallest element is: {}", k, kth_smallest);
}
