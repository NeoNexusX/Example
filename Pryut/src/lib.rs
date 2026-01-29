use pyo3::prelude::*;
use pyo3::types::PyBytes;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::sync::Mutex;

mod test_read;
/// A Python module implemented in Rust.
#[pymodule]
fn _pryut(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(receive_imzml_info, m)?)?;
    m.add_function(wrap_pyfunction!(read_batch_data, m)?)?;
    Ok(())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

struct ImzmlMetadata {
    mz_precision: u64,
    intensity_precision: u64,
    mz_offsets: Vec<u64>,
    intensity_offsets: Vec<u64>,
    mz_lengths: Vec<u64>,
    intensity_lengths: Vec<u64>,
}

static IMZML_CACHE: Mutex<Option<ImzmlMetadata>> = Mutex::new(None);

#[pyfunction]
fn receive_imzml_info(
    mz_precision: u64,
    intensity_precision: u64,
    mz_offsets: Vec<u64>,
    intensity_offsets: Vec<u64>,
    mz_lengths: Vec<u64>,
    intensity_lengths: Vec<u64>,
) -> PyResult<()> {
    let mut cache = IMZML_CACHE.lock().unwrap();
    *cache = Some(ImzmlMetadata {
        mz_precision,
        intensity_precision,
        mz_offsets,
        intensity_offsets,
        mz_lengths,
        intensity_lengths,
    });

    if let Some(data) = cache.as_ref() {
        println!("Received metadata and cached it:");
        println!("mz_precision: {}", data.mz_precision);
        println!("intensity_precision: {}", data.intensity_precision);
        println!("mz_offsets count: {}", data.mz_offsets.len());
        println!("intensity_offsets count: {}", data.intensity_offsets.len());
        println!("mz_lengths count: {}", data.mz_lengths.len());
        println!("intensity_lengths count: {}", data.intensity_lengths.len());
    }
    Ok(())
}

#[pyfunction]
fn read_batch_data(
    py: Python<'_>,
    ibd_path: String,
    start_index: usize,
    end_index: usize,
) -> PyResult<(Py<PyAny>)> {
    // 2. 获取元数据与参数校验
    let cache = IMZML_CACHE.lock().unwrap();
    let metadata = cache.as_ref().ok_or_else(|| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Metadata not initialized. Call receive_imzml_info first.")
    })?;

    if start_index >= end_index || end_index > metadata.mz_offsets.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Invalid indices: start={}, end={}, total={}", start_index, end_index, metadata.mz_offsets.len())
        ));
    }

    // 3. 预计算空间 (Pre-allocation)
    let mz_size = metadata.mz_precision as usize;
    let int_size = metadata.intensity_precision as usize;

    let total_length = metadata.mz_lengths[start_index..end_index]
        .iter()
        .sum::<u64>() as usize;

    let total_mz_bytes = total_length * mz_size;

    let total_int_bytes =total_length * int_size;

    // 4. 内存分配
    let mut mz_buffer = vec![0u8; total_mz_bytes];
    let mut int_buffer = vec![0u8; total_int_bytes];

    // 5. 批量读取 (Batch Read Loop)
    let mut file = File::open(ibd_path)?;
    let mut mz_ptr = 0;
    let mut int_ptr = 0;

    for i in start_index..end_index {
        // Read M/Z
        // let len = (metadata.mz_lengths[i] as usize) * mz_size;
        // file.seek(SeekFrom::Start(metadata.mz_offsets[i]))?;
        // file.read_exact(&mut mz_buffer[mz_ptr..mz_ptr + len])?;
        // mz_ptr += len;

        // Read Intensity
        let len = (metadata.intensity_lengths[i] as usize) * int_size;
        file.seek(SeekFrom::Start(metadata.intensity_offsets[i]))?;
        file.read_exact(&mut int_buffer[int_ptr..int_ptr + len])?;
        int_ptr += len;
    }

    // 6. 返回 Python 对象
    Ok(
        PyBytes::new(py, &int_buffer).into_any().unbind(),
    )
}
