#!/usr/bin/python3


def interpolation(array, target):
    low = 0
    high = len(array) - 1

    while (low <= high and (array[low] <= target <= array[high])):
        if low == high:
            if array[low] == target:
                return low
            else:
                return -1
        else:
            # interpolation search algorithm formula
            numerator = (target - array[low]) * (high - low)
            denominator = array[high] - array[low]
            pos = low + (numerator) // (denominator)

            if pos < low or pos > high:
                return -1
            if array[pos] == target:
                return pos
            elif array[pos] < target:
                low = pos + 1
            else:
                high = pos - 1

    return -1


array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
target = 7
result = interpolation(array, target)


if result == -1:
    print(f"{target} not found in {array}")
else:
    print(f"The value: {target} is on {result} in {array}")
