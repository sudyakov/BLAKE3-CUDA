use std::time::Instant;
use std::vec::*;

use async_cuda::ffi::device::Device;
use async_cuda::ffi::npp::context::Context;
use async_cuda::memory::{DeviceBuffer, HostBuffer};
use async_cuda::stream::Stream;
use async_cuda::*;

//use async_cuda::DeviceBuffer;

// функиция принимает теcтовый вектор и возвращаетпо отсортированый по убыванию на ЦПУ
fn sort_cpu(vec: Vec<i32>) -> Vec<i32> {
    let mut sorted_vec = vec;
    sorted_vec.sort_by(|a, b| b.cmp(a));
    sorted_vec
}

// функиция принимает теcтовый вектор и возвращаетпо отсортированый по убыванию на ГПУ
async fn sort_gpu(vec: &Vec<i32>, stream: &Stream) -> Result<Vec<i32>, async_cuda::Error> {
    // создаем контекст и стрим
    let context = Context::from_null_stream();
    //let stream = Stream::new().await.unwrap();
    // Создаем буферы на ЦПУ и ГПУ
    let host_buffer = HostBuffer::from_slice(vec).await;
    let mut device_buffer = DeviceBuffer::from_slice(&vec[..], &stream).await.unwrap();

    // Создаем контекст для вызова функци
    let context = Context::from_null_stream();
    // Запускаем сортировку на ГПУ
    let stream = Stream::new().await.unwrap();
    unsafe {}
    // Копируем отсортированный массив с ГПУ на ЦПУ
    stream.synchronize().await;
    let host_sorted_result = host_buffer.copy_to(&mut device_buffer, &stream).await;

    let host_vector: Result<Vec<i32>, async_cuda::Error> = match host_sorted_result {
        Ok(vec) => Ok(Vec::new()),
        Err(e) => Err(e),
    };
    host_vector
}

// Основная функция
#[tokio::main]
pub async fn main() {
    // создаем масив и заполняем его тестовыми данными
    let mut test_vec = Vec::new();
    for i in 0..10000 {
        test_vec.push(i);
    }
    // Выводим первые и последние элементы массива
    println!("First element: {}", test_vec[0]);
    println!("Last element: {}", test_vec[test_vec.len() - 1]);

    // функиция принимает теcтовый указатель на вектор и возвращает новый отсортированый по убыванию на ЦПУ.
    let cpu_result = sort_cpu(test_vec.clone());
    // Выводим первые и последние элементы массива
    println!("First CPU sorted element: {}", cpu_result[0]);
    println!(
        "Last CPU sorted element: {}",
        cpu_result[cpu_result.len() - 1]
    );

    // создаем стрим
    let stream = Stream::new().await.unwrap();

    // функиция принимает теcтовый вектор и возвращаетпо отсортированый по убыванию на ГПУ
    let gpu_result = sort_gpu(&test_vec, &stream).await;

    // Выводим первые и последние элементы массива
    // println!("First GPU sorted element: {}", gpu_result[0]);
    // println!(
    // "Last GPU sorted element: {}",
    // gpu_result[gpu_result.len() - 1]
    // );


    println!("Done!");
}

#[tokio::test]
async fn test_side_effects() {
    // First block contains stuff we are not interested in measuring...
    let stream = Stream::new().await.unwrap();

    // A sequence of CUDA calls that is easy to find in the trace.
    Device::synchronize().unwrap();
    let _mem_info_1 = Device::memory_info().unwrap();
    let _mem_info_2 = Device::memory_info().unwrap();
    let _mem_info_3 = Device::memory_info().unwrap();
    let _mem_info_4 = Device::memory_info().unwrap();
    Device::synchronize().unwrap();

    let _context_null = Context::from_null_stream();
    Device::synchronize().unwrap();

    let _context_new = Context::from_stream(stream);
    Device::synchronize().unwrap();
}
