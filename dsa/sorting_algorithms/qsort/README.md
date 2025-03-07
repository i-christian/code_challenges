# Quicksort Algorithm in Rust

This repository contains a Rust implementation of the quicksort algorithm. Quicksort is a popular sorting algorithm that uses a divide-and-conquer approach to sort elements in an array.

## Algorithm Explanation

The quicksort algorithm works as follows:

1. **Base Case**: If the array has fewer than two elements, return the array as it is already sorted.
2. **Choosing a Pivot**: Select the first element of the array as the pivot.
3. **Partitioning**: 
   - Create two new arrays: `less` for elements less than or equal to the pivot, and `greater` for elements greater than the pivot.
   - Iterate through the remaining elements of the array and populate the `less` and `greater` arrays accordingly.
4. **Recursion**: Recursively apply the quicksort algorithm to the `less` and `greater` arrays.
5. **Concatenation**: Concatenate the sorted `less` array, the pivot, and the sorted `greater` array to form the final sorted array.

## Complexity
- **Time Complexity**:
  - Average case: O(n log n)
  - Worst case: O(n^2)

- **Space Complexity**: O(log n)

Note: Quicksort has a smaller constant than merge sort. So if they’re both O(n log n) time, quicksort is faster. And quicksort is faster in practice because it hits the average case way more often than the worst case.
