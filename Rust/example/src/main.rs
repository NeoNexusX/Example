use imzml::{ImzML, ImzMLReader};
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    // 指定线程数，例如 8
    let _pool = ThreadPoolBuilder::new()
        .num_threads(10)
        .build_global()
        .expect("build rayon pool");
    let data_path: &str = "/Users/neo/Desktop/data/example.imzML";
    read_imzml(data_path);
}

fn read_imzml(data_path: &str) {
    let parser = ImzMLReader::from_path(data_path).unwrap();

    for error in parser.errors() {
        println!("{:?}", error);
    }

    let imzml: ImzML<_> = parser.into();
    println!("{:#?}", imzml.width());
    println!("{:#?}", imzml.height());
    println!("{:#?}", imzml.num_spectra());

    // 创建一个数组来存储谱数据
    let spectra_data: Vec<_> = imzml
        .spectra()
        .par_bridge()
        .filter_map(|spectrum_access| {
            spectrum_access
                .intensity_array()
                .map(|array| array.as_f64())
        })
        .collect();

    println!("spectra_data len = {}", spectra_data.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_imzml_runtime() {
        let data_path: &str = "/Users/neo/Desktop/data/example.imzML";
        let start_time = Instant::now();
        read_imzml(data_path);
        let duration = start_time.elapsed();
        println!("read_imzml took: {:?}", duration);
    }
}
