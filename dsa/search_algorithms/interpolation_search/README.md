# Interpolation Search Algorithm

Interpolation Search is a searching algorithm used on sorted arrays. It estimates the position of the target by interpolating based on the values at the boundaries.

## Usage

Interpolation Search is suitable for searching elements in a sorted array when:
- The array is sorted and allows for random access to elements.
- The distribution of elements is uniform, allowing for effective estimation of the target's position.

### Steps Involved:
1. **Initialization**:
   - Set `low` to `0` and `high` to the last index of the array.
   - If the array is empty, return `None`.

2. **Search Phase**:
   - While `low` is less than or equal to `high` and the target is within the bounds defined by `list[low]` and `list[high]`:
     - Calculate the estimated position (`pos`) using the formula:

       ```
       pos = low + ((target - list[low]) * (high - low)) / (list[high] - list[low])
       ```

     - If `list[pos]` equals the target, return `Some(pos)`.
     - If `list[pos]` is less than the target, update `low` to `pos + 1`.
     - If `list[pos]` is greater than the target, update `high` to `pos - 1`.

3. **Completion**:
   - If the target is found, return its index.
   - If not found after all iterations, return `None`, indicating the target is not present in the array.

## Time Complexity

- **Best Case**: O(1), when the target element is at the estimated position.
- **Average Case**: O(log log n) for uniformly distributed data, where n is the number of elements in the array.
- **Worst Case**: O(n), when the distribution of elements is non-uniform or when searching in a highly skewed dataset.

## Space Complexity

- O(1): Interpolation Search requires constant extra space for variables regardless of the input size.

## Example

Consider an array `arr = [10, 20, 30, 40, 50, 60, 70, 80, 90]`:
- Searching for `50` using Interpolation Search:
  - Estimate position based on the values at the boundaries.
  - Find `50` at index `4` after a few iterations.

This algorithm efficiently estimates the position of the target, combining the speed of interpolation with precision.

## Run the code using:
```
  cargo test
```
