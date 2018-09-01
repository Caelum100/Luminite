//! Utility functions for the creation, allocation, and
//! filling of GPU buffers.

use super::*;
/// Creates an empty buffer on the GPU.
pub fn empty_buffer<B: Backend, I>(
    device: &B::Device,
    memory_types: &[MemoryType],
    properties: Properties,
    usage: Usage,
    item_count: usize,
) -> (B::Buffer, B::Memory) {
    // Length of each item
    let stride = ::std::mem::size_of::<I>() as u64;
    // Length of entire buffer
    let buffer_len = item_count as u64 * stride;
    let unbound_buffer = device.create_buffer(buffer_len, usage).unwrap();
    let requirements = device.get_buffer_requirements(&unbound_buffer);
    let upload_type = memory_types
        .iter()
        .enumerate()
        .position(|(id, ty)| {
            requirements.type_mask & (1 << id) != 0 && ty.properties.contains(properties)
        })
        .unwrap()
        .into();

    let buffer_memory = device
        .allocate_memory(upload_type, requirements.size)
        .unwrap();
    let buffer = device
        .bind_buffer_memory(&buffer_memory, 0, unbound_buffer)
        .unwrap();

    (buffer, buffer_memory)
}

/// Transfers data into a GPU buffer.
/// `buffer_memory` can be created by calling
/// `empty_buffer()`.
pub fn fill_buffer<B: Backend, I: Copy>(
    device: &B::Device,
    buffer_memory: &mut B::Memory,
    items: &[I],
) {
    let stride = std::mem::size_of::<I>() as u64;
    let buffer_len = items.len() as u64 * stride;

    let mut dest = device
        .acquire_mapping_writer::<I>(&buffer_memory, 0..buffer_len)
        .unwrap();
    dest.copy_from_slice(items);
    device.release_mapping_writer(dest);
}

/// Creates a buffer and fills
/// it with items. This is equivalent
/// to using `empty_buffer()` and then `fill_buffer()`.
pub fn create_buffer<B: Backend, I: Copy>(
    device: &B::Device,
    memory_types: &[MemoryType],
    properties: Properties,
    usage: Usage,
    items: &[I],
) -> (B::Buffer, B::Memory) {
    let (empty_buffer, mut empty_buffer_mem) =
        empty_buffer::<B, I>(device, memory_types, properties, usage, items.len());

    fill_buffer::<B, I>(device, &mut empty_buffer_mem, items);

    (empty_buffer, empty_buffer_mem)
}
