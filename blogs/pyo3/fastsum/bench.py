import time
import subprocess
import fastsum


def py_sum(n: int) -> int:
    return sum(i for i in range(n) if i % 3 == 0)


def rust_bin_sum(n: int) -> int:
    out = subprocess.check_output(["./target/release/fastsum_bin", str(n)])
    return int(out.strip())


def bench(fn, n: int, repeat: int) -> float:
    start = time.perf_counter()
    for _ in range(repeat):
        fn(n)
    return time.perf_counter() - start


if __name__ == "__main__":
    n = 10_000_000
    repeat = 10

    # warmup to show first call overhead
    t0 = time.perf_counter()
    fastsum.rs_sum(1)
    init_cost = time.perf_counter() - t0
    print(f"PyO3 first call: {init_cost*1000:.2f} ms")

    t_py = bench(py_sum, n, repeat)
    t_rs = bench(fastsum.rs_sum, n, repeat)
    t_bin = bench(rust_bin_sum, n, repeat)

    print(f"Python total: {t_py:.3f} s")
    print(f"PyO3 total: {t_rs:.3f} s")
    print(f"Rust binary total: {t_bin:.3f} s")
