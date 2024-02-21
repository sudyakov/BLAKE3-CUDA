use cudarc::{
    driver::{CudaDevice, DriverError, LaunchAsync, LaunchConfig},
    nvrtc::Ptx,
};
use std::thread;

// Option 1: use the same device on each thread.
fn test_option_1(cfg: LaunchConfig) -> Result<(), DriverError> {
    // Option 1: use the same device on each thread.
    let dev = CudaDevice::new(0)?;
    dev.load_ptx(
        Ptx::from_file(
            "/home/sudya/Repos/BLAKE3-CUDA/cuda/crates/cudarc-main/examples/hello_world.ptx",
        ),
        "kernel",
        &["hello_world"],
    )?;

    // explicit borrow so we don't have to re-clone the device for each thread
    let dev = &dev;

    thread::scope(move |s| {
        for i in 0..10i32 {
            s.spawn(move || {
                dev.bind_to_thread()?;
                let f = dev.get_func("kernel", "hello_world").unwrap();
                unsafe { f.launch(cfg, (i,)) }
            });
        }
    });
    Ok(())
}
// Option 2: create a new device in each thread
fn test_option_2(cfg: LaunchConfig) -> Result<(), DriverError> {
    // Option 2: create a new device in each thread
    let ptx = Ptx::from_file(
        "/home/sudya/Repos/BLAKE3-CUDA/cuda/crates/cudarc-main/examples/hello_world.ptx",
    );

    thread::scope(|s| {
        for i in 0..10i32 {
            let ptx = ptx.clone();
            s.spawn(move || {
                let dev = CudaDevice::new(0)?;
                dev.load_ptx(ptx, "kernel", &["hello_world"])?;
                let f = dev.get_func("kernel", "hello_world").unwrap();
                unsafe { f.launch(cfg, (i + 100,)) }
            });
        }
    });

    Ok(())
}

// Main function
fn main() -> Result<(), DriverError> {
    
    let cfg = LaunchConfig::for_num_elems(10);

    let res1 = test_option_1(cfg);
    print!("Option 1 result: {:?}", res1);

    // let res2 = test_option_2(cfg);
    // print!("Option 2 result: {:?}", res2);

    Ok(())
}




