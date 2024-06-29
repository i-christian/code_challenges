
# Exponential Search Algorithm

Exponential Search is a searching algorithm used on sorted arrays. It combines an initial exponential step to quickly narrow down the range where the target may reside, followed by a binary search within that range.

## Usage

Exponential Search is suitable for searching elements in a sorted array when:
- The array is sorted and allows for random access to elements.
- Fast narrowing down of potential search space is desired.

### Steps Involved:
1. **Initialization**:
   - Start by checking if the first element matches the target.
   - If a match is found, return the 0th index.

2. **Exponential Phase**:
   - Initialize `idx = 1` and compute `jump = idx.pow(2)`.
   - Double `idx` and recalculate `jump` until `jump` exceeds the array bounds or the element at `list[jump]` is greater than `target`.

3. **Binary Search Phase**:
   - Set `low = 2usize.pow(idx as u32 - 1)` and `high = usize::min(jump, list.len() - 1)` to define the range `[low, high]` for binary search.
   - Perform binary search within `[low, high]` to find the target element.

4. **Completion**:
   - If the target is found during binary search, return its index.
   - If not found after all iterations, return `None`, indicating the target is not present in the array.

## Time Complexity

- **Best Case**: O(1), when the target element is at the first position.
- **Average Case**: O(log n), where n is the number of elements in the array. The exponential step quickly narrows down the range, followed by a binary search.
- **Worst Case**: O(log n), when the target is at the end of the array or absent, requiring traversal and binary search.

## Space Complexity

- O(1): Exponential Search requires constant extra space for variables regardless of the input size.

## Example

Consider an array `arr = [1, 4, 7, 9, 11, 14, 17, 19, 22]`:
- Searching for `14` using Exponential Search:
  - Start with `idx = 1`, doubling until `jump = 8`.
  - Narrow the search to `[4, 8]` and perform binary search, finding `14` at index `5`.

This algorithm efficiently narrows down the search space, combining the speed of the exponential step with the precision of binary search.

## Run the code using:
```
   cargo test
```
