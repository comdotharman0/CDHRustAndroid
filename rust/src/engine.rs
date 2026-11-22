use std::sync::Mutex;

pub struct AppEngine {
    pub counter: i64,
    pub history: Vec<i64>,
}

impl AppEngine {
    pub const fn new() -> Self {
        Self {
            counter: 0,
            history: Vec::new(),
        }
    }

    pub fn process_input(&mut self, delta: i64) -> String {
        self.counter += delta;
        self.history.push(self.counter);
        format!("State: {} (Steps: {})", self.counter, self.history.len())
    }

    pub fn reset(&mut self) -> String {
        self.counter = 0;
        self.history.clear();
        String::from("State: 0 (Reset)")
    }
}

// Global thread-safe engine instance shared across JNI calls
pub static ENGINE: Mutex<AppEngine> = Mutex::new(AppEngine::new());

// --- TERMUX RAPID DEBUG LOOP ---
// Run `cargo test -- --nocapture` in Termux (~0.4s per edit)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_logic() {
        let mut engine = AppEngine::new();
        assert_eq!(engine.process_input(5), "State: 5 (Steps: 1)");
        assert_eq!(engine.process_input(-2), "State: 3 (Steps: 2)");
        println!("Debug output -> counter: {}, history: {:?}", engine.counter, engine.history);

        assert_eq!(engine.reset(), "State: 0 (Reset)");
        assert!(engine.history.is_empty());
    }
}
