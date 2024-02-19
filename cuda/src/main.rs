use cudarc::driver::{CudaDevice, DriverError, LaunchAsync, LaunchConfig};
use cudarc::nvrtc::compile_ptx;

const PTX_SRC: &str = "
extern \"C\" __global__ void matmul(float* A, float* B, float* C, int N) {
    int ROW = blockIdx.y*blockDim.y+threadIdx.y;
    int COL = blockIdx.x*blockDim.x+threadIdx.x;

    float tmpSum = 0;

    if (ROW < N && COL < N) {
        // each thread computes one element of the block sub-matrix
        for (int i = 0; i < N; i++) {
            tmpSum += A[ROW * N + i] * B[i * N + COL];
        }
    }
    // printf(\"pos, (%d, %d) - N %d - value %d\\n\", ROW, COL, N, tmpSum);
    C[ROW * N + COL] = tmpSum;
}
";

fn main() -> Result<(), DriverError> {
    let start = std::time::Instant::now();

    // Compiles the PTX source code and checks the compilation time.
    // Prints a message with the compilation time.
    let ptx = compile_ptx(PTX_SRC).unwrap();
    println!("Compilation succeeded in {:?}", start.elapsed());

    // Creates a new CudaDevice instance for the GPU with the given index.
    // Prints a message with the elapsed time since `start` was instantiated.
    // This measures the time taken to initialize the CudaDevice.
    let dev = CudaDevice::new(0)?;
    println!("Built in {:?}", start.elapsed());

    // Loads the compiled PTX code into the CudaDevice instance `dev` using the
    // given function name `matmul` and kernel name `matmul`. Prints a message with
    // the elapsed time since `start` was instantiated, measuring the time taken to
    // load the PTX code.
    dev.load_ptx(ptx, "matmul", &["matmul"])?;
    let cuda_fun = dev.get_func("matmul", "matmul").unwrap();
    println!("Loaded in {:?}", start.elapsed());

    // Creates host arrays to hold the input and output matrices.
    // `a_host` - Holds the input matrix A.
    // `b_host` - Holds the input matrix B.
    // `c_host` - Will hold the output matrix C after the computation.
    let a_host = [1.0f32, 2.0, 3.0, 4.0];
    let b_host = [1.0f32, 2.0, 3.0, 4.0];
    let mut c_host = [0.0f32; 4];

    // Copies the host arrays `a_host`, `b_host` and `c_host` to device
    // memory, returning device pointers `a_dev`, `b_dev` and `c_dev`.
    // This allows us to upload the input data to the GPU device memory
    // so it is accessible to the CUDA kernel.
    let a_dev = dev.htod_sync_copy(&a_host)?;
    let b_dev = dev.htod_sync_copy(&b_host)?;
    let mut c_dev = dev.htod_sync_copy(&c_host)?;

    println!("Copied in {:?}", start.elapsed());

    // Configures the launch parameters for invoking the CUDA kernel.
    // Sets the block dimensions (`block_dim`) and grid dimensions (`grid_dim`)
    // to configure how many thread blocks and threads to launch. Also sets the
    // amount of shared memory to allocate for each thread block.
    let cfg = LaunchConfig {
        block_dim: (2, 2, 1),
        grid_dim: (1, 1, 1),
        shared_mem_bytes: 0,
    };

    // Launches the CUDA kernel function previously loaded into the CudaDevice
    // instance `dev`. The `cfg` parameter configures the launch dimensions and
    // shared memory size. The tuple contains pointers to the input and output
    // device arrays to compute the matrix multiplication. The final integer
    // specifies the stream to launch the kernel on.
    unsafe { cuda_fun.launch(cfg, (&a_dev, &b_dev, &mut c_dev, 2i32)) }?;

    dev.dtoh_sync_copy_into(&c_dev, &mut c_host)?;
    println!("Found {:?} in {:?}", c_host, start.elapsed());
    Ok(())
}







