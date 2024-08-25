from max_subarray.kadane import kadane, kadane_with_indices

def test_kadana():
    arr = [13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7]
    assert kadane(arr) == 43

def test_kadana_with_indices():
    arr = [13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7]
    result = kadane_with_indices(arr)
    assert result.sum == 43
    assert result.start == 7
    assert result.end == 10
