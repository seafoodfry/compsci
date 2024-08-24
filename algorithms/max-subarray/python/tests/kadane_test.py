import pytest
from max_subarray.kadane import kadane

def test_kadana():
    arr = [13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7]
    assert kadane(arr) == 43
