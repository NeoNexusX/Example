# uv add cupy-cuda13x or cupy-cuda12x according to your cuda version
# pip install cupy-cuda13x
import cupy as cp

def main():

    # device part
    device = cp.cuda.Device()
    device_props = cp.cuda.runtime.getDeviceProperties(device.id)
    print(f"Using device: {device.id} - {device_props['name'].decode()}")
    print("device.compute_capability :", device.compute_capability)
    # 查看 MiB 数值，便于和 nvidia-smi 这一行对比
    print(f"Total Memory (MiB): {device_props['totalGlobalMem'] / (1024**2):.2f} MiB")
    print("Hello from cupy-test!")

    # 1. 获取当前空闲显存 (free, total)
    mem_free, _ = device.mem_info
    print(f"Free Memory: {mem_free / (1024**2):.2f} MiB")

    # 2. 计算目标大小 (空闲显存的1/3)
    target_bytes = mem_free // 3

    # 3. 计算元素数量 (假设使用 float64，每个元素 8 字节)
    element_size = 8 # bytes for float64
    num_elements = target_bytes // element_size

    print(f"Allocating array with {num_elements} elements (approx {target_bytes / (1024**2):.2f} MiB)...")

    # 4. 申请全是 1 的矩阵
    x_huge = cp.ones(num_elements, dtype=cp.float64)

    # 验证一下
    print(f"assert x_huge.nbytes == target_bytes : {x_huge.nbytes == target_bytes}")

    l2_gpu = cp.linalg.norm(x_huge)
    print(f"Norm calculated: {l2_gpu}")

    input("Press Enter to free memory and exit...")


if __name__ == "__main__":
    main()
