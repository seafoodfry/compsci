def kadane(numbers: list[float | int]) -> float:
    """Find the largest sum of any contiguous subarray."""
    current_sum = best_sum = numbers[0]
    for x in numbers:
        current_sum = max(x, current_sum + x)
        best_sum = max(best_sum, current_sum)
    return best_sum
