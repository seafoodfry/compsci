from dataclasses import dataclass
from typing import Optional


@dataclass
class SubArrayResult:
    sum: float
    start: int
    end: int


def kadane(numbers: list[float | int]) -> Optional[float]:
    """Find the largest sum of any contiguous subarray."""
    if len(numbers) == 0:
        return None

    current_sum = best_sum = numbers[0]
    for x in numbers:
        current_sum = max(x, current_sum + x)
        best_sum = max(best_sum, current_sum)
    return best_sum


def kadane_with_indices(numbers: list[float | int]) -> Optional[SubArrayResult]:
    """Find the largest sum of any contiguous subarray."""
    if len(numbers) == 0:
        return None

    current_sum = best_sum = numbers[0]
    start = end = best_start = 0

    for i, x in enumerate(numbers):
        if x > current_sum + x:
            current_sum = x
            start = i
        else:
            current_sum += x

        if current_sum > best_sum:
            best_sum = current_sum
            best_start = start
            end = i

    return SubArrayResult(sum=best_sum, start=best_start, end=end)
