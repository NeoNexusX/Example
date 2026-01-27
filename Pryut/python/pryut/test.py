import pryut._pryut as pryut

def test_sum_as_string():
    # 这是一个 Python 测试函数，用于调用 Rust 实现的 sum_as_string
    result = pryut.sum_as_string(5, 7)
    print(f"Result from Rust: {result}")
    assert result == "12"

if __name__ == "__main__":
    test_sum_as_string()
    print("Test passed!")
