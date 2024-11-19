use std::{thread, time::Duration};

pub fn run() {
    struct ComputeCache<T>
    where
        T: Fn() -> String,
    {
        computation: T,
        cached_value: Option<String>,
    }

    impl<T> ComputeCache<T>
    where
        T: Fn() -> String,
    {
        fn new(computation: T) -> Self {
            ComputeCache {
                computation,
                cached_value: None,
            }
        }

        fn get_result(&mut self) -> String {
            match &self.cached_value {
                Some(value) => {
                    println!("Retrieved from cache instantly!");
                    value.clone()
                }
                None => {
                    println!("Computing (this will take 2 seconds)...");
                    let result = (self.computation)();
                    self.cached_value = Some(result.clone());
                    result
                }
            }
        }
    }

    let mut cache = ComputeCache::new(|| {
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("Task 5 - First call:");
    println!("Result: {}", cache.get_result());

    println!("\nTask 5 - Second call:");
    println!("Result (cached): {}", cache.get_result());
}
