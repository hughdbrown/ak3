use std::mem;

use wasm_bindgen::prelude::*;
use js_sys::{Array, ArrayBuffer, Uint8Array};

#[wasm_bindgen]
pub struct SharedMemoryBuffer {
    buffer: Vec<u8>,
    capacity: usize,
}

#[wasm_bindgen]
impl SharedMemoryBuffer {
    #[wasm_bindgen(constructor)]
    pub fn new(initial_capacity: usize) -> Self {
        SharedMemoryBuffer {
            buffer: Vec::with_capacity(initial_capacity),
            capacity: initial_capacity,
        }
    }

    #[wasm_bindgen]
    pub fn get_buffer(&self) -> Result<ArrayBuffer, JsValue> {
        let array = unsafe {
            // Create a view into our Rust memory
            Uint8Array::view(&self.buffer)
        };
        
        // Create a copy for JavaScript
        Ok(array.buffer())
    }

    #[wasm_bindgen]
    pub fn process_data(&mut self, data: &[u8]) -> Result<(), JsValue> {
        if data.len() > self.capacity {
            return Err(JsValue::from_str("Data exceeds buffer capacity"));
        }

        self.buffer.clear();
        self.buffer.extend_from_slice(data);
        
        // Perform processing in place
        for byte in self.buffer.iter_mut() {
            *byte = byte.wrapping_add(1);
        }

        Ok(())
    }
}

// Helper function to create typed arrays
#[wasm_bindgen]
pub fn create_typed_array(size: usize) -> Uint8Array {
    let mut array = vec![0u8; size];
    let ptr = array.as_mut_ptr();
    
    // Prevent deallocation when function returns
    mem::forget(array);
    
    unsafe {
        Uint8Array::view_mut_raw(ptr, size)
    }
}
