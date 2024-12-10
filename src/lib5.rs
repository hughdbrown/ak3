use web_sys::Performance;
use std::cell::RefCell;

thread_local! {
    static PERFORMANCE: RefCell<Option<Performance>> = RefCell::new(None);
}

#[wasm_bindgen]
pub struct PerformanceMetrics {
    start_time: f64,
    measurements: Vec<(String, f64)>,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        let performance = web_sys::window()
            .unwrap()
            .performance()
            .unwrap();
            
        PERFORMANCE.with(|p| {
            *p.borrow_mut() = Some(performance.clone());
        });

        PerformanceMetrics {
            start_time: performance.now(),
            measurements: Vec::new(),
        }
    }

    pub fn measure<F>(&mut self, name: &str, f: F)
    where
        F: FnOnce(),
    {
        PERFORMANCE.with(|p| {
            let performance = p.borrow().as_ref().unwrap();
            let start = performance.now();
            f();
            let end = performance.now();
            self.measurements.push((name.to_string(), end - start));
        });
    }

    pub fn report(&self) -> String {
        let mut report = String::new();
        let total_time = PERFORMANCE.with(|p| {
            p.borrow().as_ref().unwrap().now() - self.start_time
        });

        report.push_str(&format!("Total time: {:.2}ms\n", total_time));
        
        for (name, time) in &self.measurements {
            let percentage = (time / total_time) * 100.0;
            report.push_str(&format!(
                "{}: {:.2}ms ({:.1}%)\n",
                name, time, percentage
            ));
        }
        
        report
    }
}
