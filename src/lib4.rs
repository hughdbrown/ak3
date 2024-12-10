#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Strip unnecessary features
#[cfg(target_arch = "wasm32")]
mod wasm_size_optimization {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen(start)]
    pub fn start() {
        // Initialize only what's needed
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
}
