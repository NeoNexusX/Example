use mzdata::io::imzml::reader::ImzMLReader;
use mzdata::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let data_path: &str = "/Users/neo/Desktop/data/example.imzML";
    // 设置想要使用的线程数量
    let num_threads = 4;
    read_imzml(data_path, num_threads);
}

fn read_imzml(data_path: &str, num_threads: usize) {
    let func_start_time = Instant::now();
    // 1. 初始化读取器以获取谱图总数
    // 在主线程打开一次以获取元数据和总数
    let init_reader = ImzMLReader::open_path(data_path).expect("Failed to open imzML file");
    let total_spectra = init_reader.len();
    println!("Total spectra to process: {}", total_spectra);

    // 2. 创建局部线程池
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("Failed to build thread pool");

    // 3. 在指定线程池中执行并发读取与聚合
    pool.install(|| {
        // 获取当前局部线程池的实际线程数
        let effective_threads = rayon::current_num_threads();
        // 计算每个块的大小，向上取整
        let chunk_size = (total_spectra + effective_threads - 1) / effective_threads;

        // 生成每个线程需处理的索引范围 (Start..End)
        let ranges: Vec<_> = (0..total_spectra)
            .step_by(chunk_size)
            .map(|start| start..std::cmp::min(start + chunk_size, total_spectra))
            .collect();

        println!("Splitting work into {} chunks for {} threads", ranges.len(), effective_threads);

        let start_time = Instant::now();

        // 使用 par_iter 并行处理每个范围
        let spectra: Vec<_> = ranges.par_iter()
            .map(|range| {
                // 线程安全性：在每个任务中创建一个新的读取器实例。
                // 这样每个线程拥有独立的文件句柄，无需 Mutex 锁住同一个 Reader，避免了 IO 瓶颈。
                let mut reader = ImzMLReader::open_path(data_path).expect("Failed to open reader in thread");
                
                let mut chunk_data = Vec::with_capacity(range.len());
                
                for i in range.clone() {
                    // 读取指定索引的谱图
                    if let Some(spec) = reader.get_spectrum_by_index(i) {
                        // 确保加载强度数据，这里我们将读取到的完整 Spectrum 对象收集起来
                        chunk_data.push(spec);
                    }
                }
                chunk_data
            })
            // 将各个线程返回的 Vec<Spectrum> 扁平化为一个大的 Vec<Spectrum>
            .flatten()
            // Rayon 的 collect 会自动处理并发写入的聚合
            .collect();

        let duration = start_time.elapsed();
        println!("Successfully read {} spectra in {:.2?}", spectra.len(), duration);
    });
    println!("read_imzml function executed in {:.2?}", func_start_time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_imzml_runtime() {
        let data_path: &str = "/Users/neo/Desktop/data/example.imzML";
        // 测试使用 2 个线程
        read_imzml(data_path, 8);
    }
}