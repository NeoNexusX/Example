from pyimzml.ImzMLParser import ImzMLParser
from pryut import _pryut
import numpy as np
import os
import time

def prepare_data(file_path):

    parser = ImzMLParser(file_path)

    # Get metadata
    mz_precision = 64
    intensity_precision = 32

    # offsets
    mz_offsets = parser.mzOffsets
    intensity_offsets = parser.intensityOffsets

    # lengths
    mz_lengths = parser.mzLengths
    intensity_lengths = parser.intensityLengths

    return (
        mz_precision,
        intensity_precision,
        mz_offsets,
        intensity_offsets,
        mz_lengths,
        intensity_lengths,
    )


def test_data2rust():
    file_path = "/Users/neo/Desktop/data/example.imzML"  # Updated path based on workspace info from test_read.rs
    (   mz_precision,
        intensity_precision,
        mz_offsets,
        intensity_offsets,
        mz_lengths,
        intensity_lengths,
    ) = prepare_data(file_path)

    try:
        _pryut.receive_imzml_info(
            mz_precision,
            intensity_precision,
            mz_offsets,
            intensity_offsets,
            mz_lengths,
            intensity_lengths,
        )
        print("Rust function called successfully!")
    except ImportError as e:
        print(f"Could not import Rust module: {e}")
        print("Make sure to run `uv run maturin develop` first.")

    print("All tests passed!")


def test_read_batch_martrix():

    imzml_path = "/Users/neo/Desktop/data/example.imzML"
    ibd_path = imzml_path.replace(".imzML", ".ibd")

    # 获取元数据以便计算总数和初始化 Rust
    (   mz_precision,
        intensity_precision,
        mz_offsets,
        intensity_offsets,
        mz_lengths,
        intensity_lengths,
    ) = prepare_data(imzml_path)

    # 初始化 Rust 模块缓存
    _pryut.receive_imzml_info(
        mz_precision,
        intensity_precision,
        mz_offsets,
        intensity_offsets,
        mz_lengths,
        intensity_lengths,
    )

    total_spectra = len(mz_offsets)
    batch_size = 100  # 设定 batch 大小
    
    print(f"\n[Benchmark] Start reading {total_spectra} spectra in batches of {batch_size}...")

    start_time = time.time()

    try:
        for start_index in range(0, total_spectra, batch_size):
            end_index = min(start_index + batch_size, total_spectra)

            # 调用 Rust 函数读取一个 batch
            int_bytes = _pryut.read_batch_data(ibd_path, start_index, end_index)
            
            # 可选: 这里可以添加转换代码 np.frombuffer(...) 来验证数据完整性
            int_arr = np.frombuffer(int_bytes, dtype=np.float32)

            print(f"Processed batch {start_index} - {end_index}", end='\r')

        end_time = time.time()
        duration = end_time - start_time

        print(f"\n\nStatus: Success")
        print(f"Total Time: {duration:.4f} seconds")
        print(f"Throughput: {total_spectra / duration:.2f} spectra/sec")

    except Exception as e:
        print(f"\nError during batch read: {e}")

