# Exponential Search Algorithm

In the exponential search algorithm, the jump starts from the 1st index of the array. So we manually compare the first element as the first step in the algorithm.

## Steps Involved:

1. **Compare the first element**:
   - Compare the first element in the array with the key.
   - If a match is found, return the 0th index.

2. **Initialize index**:
   - Initialize `i = 1` and compare the ith element of the array with the key to be searched.
   - If it matches, return the index.

3. **Exponential jumps**:
   - If the element does not match, jump through the array exponentially in the powers of 2.
   - Compare the element present in the incremental position (i.e., at index `2^k` where k is an incrementing integer).

4. **Repeat comparisons**:
   - If a match is found, return the index.
   - Otherwise, repeat Step 3 iteratively until the element at the incremental position becomes greater than the key to be searched.

5. **Binary search in block**:
   - Since the next increment has a higher element than the key and the input is sorted, apply the binary search algorithm on the current block.

6. **Return result**:
   - Return the index at which the key is present if the match is found.
   - Otherwise, determine it as an unsuccessful search.

## Time Complexity

- **Best Case**: O(1), when the target element is at the first position.
- **Average Case**: O(log n), where n is the number of elements in the array. The exponential step narrows down the range quickly, followed by a binary search.
- **Worst Case**: O(log n), when the target is at the end of the array or absent, requiring traversal and binary search.

## Space Complexity

- O(1): Exponential Search requires constant extra space for variables regardless of the input size.

## Example

Consider an array `arr = [1, 4, 7, 9, 11, 14, 17, 19, 22]`:
- Searching for `14` using Exponential Search:
  - Start at index `1`, doubling until the element is greater than `14`.
  - Perform a binary search within the narrowed range, finding `14` at index `5`.

This algorithm efficiently narrows down the search space, combining the speed of the exponential step with the precision of binary search.

## Run the code using:
```
  cargo test
```
