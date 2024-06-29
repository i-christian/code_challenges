# Jump Search Algorithm

Jump Search is a searching algorithm used on sorted arrays. It works by jumping ahead by a fixed number of steps in the array to find an interval where the target element may lie, and then performs a linear search in that interval.

## Usage

Jump Search is suitable for searching elements in a sorted array when:
- Random access to elements is feasible (like in arrays).
- The elements are uniformly distributed.

### Steps Involved:
1. **Initialization**:
   - Calculate the optimal block size (`jump_step`) as the square root of the array length.
   - Set `current_index` to 0, the start of the array.
   - Set `next_index` to `jump_step`, the index for the next block to check.

2. **Jumping Through Blocks**:
   - While `current_index` is within the bounds of the array:
     - Compare the element at `current_index` with the target.
     - If they match, return `current_index` as the position of the target.
     - If the element is less than the target, move `current_index` to `next_index` and update `next_index` by adding `jump_step`. Ensure `next_index` does not exceed the array length.
     - If the element is greater than the target, perform a linear search within the previous block.

3. **Linear Search in Block**:
   - Once a block containing the potential target is found:
     - Perform a linear search from `(current_index - jump_step)` to `current_index` within this block.
     - If the target is found during this search, return its index.

4. **Completion**:
   - If the target is not found after all iterations, return `None`, indicating the target is not present in the array.

## Time Complexity

- **Best Case**: O(1), when the target element is found at the first comparison.
- **Average Case**: O(√n), where n is the number of elements in the array. This is due to the jump step being √n, resulting in approximately √n comparisons in the worst case.
- **Worst Case**: O(√n), when the target is at the end of the array or absent, requiring a full traversal of the array in √n steps.

## Space Complexity

- O(1): Jump Search requires constant extra space for variables regardless of the input size.

## Example

Consider an array `arr = [1, 4, 7, 9, 11, 14, 17, 19, 22]`:
- Searching for `14` using Jump Search:
  - Calculate `jump_step = sqrt(9) ≈ 3`.
  - Start at `current_index = 0`, `next_index = 3`.
  - Jump through blocks until `current_index = 6`, `next_index = 9`.
  - Perform linear search within the block `[6, 7, 8]`, finding `14` at index `5`.

This algorithm combines the efficiency of binary search with the simplicity of linear search, making it effective for medium-sized arrays where binary search might be overkill.

## Run the code using:
```
  cargo test
```
