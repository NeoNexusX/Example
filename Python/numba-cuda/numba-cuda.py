from numba import cuda
import numpy as np
import math

# 1. 定义填充内核 (替代 cp.ones)
@cuda.jit
def fill_ones_kernel(arr):
    # 获取当前线程在网格中的绝对位置
    pos = cuda.grid(1)
    # 边界检查
    if pos < arr.size:
        arr[pos] = 1.0

# 2. 定义归约求和 (用于计算 Norm)
@cuda.reduce
def sum_reduce(a, b):
    return a + b

def main():
    # 确保选中设备 0
    cuda.select_device(0)
    device = cuda.get_current_device()

    # 获取显存信息 (free, total)
    # Numba context 返回的是字节
    mem_free, mem_total = cuda.current_context().get_memory_info()

    # Numba 的 device.name 通常是 bytes 类型，需要 decode
    print(f"Using device: {device.id} - {device.name}")

    # 获取计算能力 (Compute Capability)
    cc =  device.compute_capability
    print(f"device.compute_capability : {cc[0]}.{cc[1]}")

    # 打印总显存
    print(f"Total Memory (MiB): {mem_total / (1024**2):.2f} MiB")

    print("Hello from numba-cuda-test!")

    print(f"Free Memory: {mem_free / (1024**2):.2f} MiB")

    # ----------------------------------------------------
    # 计算分配策略
    # ----------------------------------------------------

    # 计算目标大小 (空闲显存的 1/3)
    target_bytes = mem_free // 3

    # 计算元素数量 (float64 = 8 bytes)
    element_size = 8 
    num_elements = target_bytes // element_size

    print(f"Allocating array with {num_elements} elements (approx {target_bytes / (1024**2):.2f} MiB)...")

    # ----------------------------------------------------
    # Numba 显存分配与计算
    # ----------------------------------------------------

    # 1. 在 GPU 上申请未初始化的显存 (相当于 C 的 malloc)
    # 注意：这步操作非常快，因为它只分分配地址，不涉及主机到设备的复制
    d_x_huge = cuda.device_array(num_elements, dtype=np.float64)

    # 2. 验证大小
    # numba 的 device_array 有 .nbytes 属性吗？有的，但在某些版本可能是 .alloc_size
    # 我们可以手动计算验证
    actual_bytes = d_x_huge.size * d_x_huge.dtype.itemsize
    print(f"assert d_x_huge.nbytes == target_bytes : {actual_bytes == target_bytes}")

    # 3. 启动内核填充数据 (替代 cp.ones)
    threads_per_block = 256
    blocks_per_grid = (num_elements + (threads_per_block - 1)) // threads_per_block

    print(f"Launching kernel with {blocks_per_grid} blocks and {threads_per_block} threads...")
    fill_ones_kernel[blocks_per_grid, threads_per_block](d_x_huge)

    # 确保内核执行完成
    cuda.synchronize()

    # 4. 计算 L2 Norm
    # L2 Norm = sqrt(sum(x_i^2))
    # 因为我们填充的是 1.0，所以 x_i^2 也是 1.0，直接对数组求和即可
    print("Calculating norm...")

    # 使用 Numba 内置的归约操作
    total_sum = sum_reduce(d_x_huge)
    l2_gpu = math.sqrt(total_sum)

    print(f"Norm calculated: {l2_gpu}")

    input("Press Enter to free memory and exit...")

    # Numba 会在对象引用计数归零时自动释放显存，
    # 但为了演示，我们可以显式删除引用
    del d_x_huge
    # 重置上下文以彻底清理（可选）
    # cuda.close() 

if __name__ == "__main__":
    main()
