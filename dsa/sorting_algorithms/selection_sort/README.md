# Selection Sort Algorithm in Rust

## Description

This project implements the selection sort algorithm in Rust. Selection sort is a simple comparison-based sorting algorithm. The implementation consists of two main functions: `find_smallest` and `selection_sort`.

- `find_smallest`: This function takes a slice of integers and returns the index of the smallest element in the slice.
- `selection_sort`: This function takes a vector of integers and returns a sorted vector using the selection sort algorithm. If the input vector is empty, it returns `None`.

## Theory

### Selection Sort

Selection sort works by repeatedly finding the minimum element (considering ascending order) from the unsorted part of the array and putting it at the beginning. The algorithm maintains two subarrays:
- The already sorted subarray.
- The remaining subarray which is unsorted.

In each iteration of the selection sort, the minimum element from the unsorted subarray is picked and moved to the sorted subarray.

### Detailed Steps

1. Start with an empty sorted subarray and the entire input array as the unsorted subarray.
2. Find the smallest element in the unsorted subarray using the `find_smallest` function.
3. Move this smallest element to the end of the sorted subarray by removing it from the unsorted subarray and appending it to the sorted subarray.
4. Repeat the process until all elements are moved to the sorted subarray.

### Complexity

- **Time Complexity**: O(n²), where n is the number of elements in the array. This is because finding the smallest element takes O(n) time and this process is repeated n times.
- **Space Complexity**: O(n), since we are using an additional vector to store the sorted elements.
