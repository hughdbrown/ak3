use wit_bindgen_rust::*;

#[wit_section]
pub mod interface {
    #[derive(Clone)]
    pub struct ComponentConfig {
        pub name: String,
        pub version: String,
        pub features: Vec<String>,
    }

    pub trait Component {
        fn initialize(config: ComponentConfig) -> Result<(), String>;
        fn process_message(message: Vec<u8>) -> Result<Vec<u8>, String>;
        fn cleanup() -> Result<(), String>;
    }
}

#[derive(Default)]
pub struct MyComponent {
    config: Option<ComponentConfig>,
    initialized: bool,
}

impl Component for MyComponent {
    fn initialize(config: ComponentConfig) -> Result<(), String> {
        let component = MyComponent {
            config: Some(config),
            initialized: true,
        };
        
        // Store component instance
        unsafe {
            COMPONENT = Some(component);
        }
        
        Ok(())
    }

    fn process_message(message: Vec<u8>) -> Result<Vec<u8>, String> {
        let component = unsafe {
            COMPONENT.as_mut().ok_or("Component not initialized")?
        };
        
        if !component.initialized {
            return Err("Component not initialized".into());
        }

        // Process message
        let mut response = message;
        response.reverse();
        Ok(response)
    }

    fn cleanup() -> Result<(), String> {
        unsafe {
            COMPONENT = None;
        }
        Ok(())
    }
}

static mut COMPONENT: Option<MyComponent> = None;

#[export_name = "component_initialize"]
pub extern "C" fn component_initialize(
    ptr: *const u8,
    len: usize,
) -> i32 {
    let config_bytes = unsafe {
        std::slice::from_raw_parts(ptr, len)
    };
    
    match serde_json::from_slice::<ComponentConfig>(config_bytes) {
        Ok(config) => {
            match MyComponent::initialize(config) {
                Ok(()) => 0,
                Err(_) => -1,
            }
        }
        Err(_) => -1,
    }
}
