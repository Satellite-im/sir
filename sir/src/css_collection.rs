use std::sync::Mutex;

#[doc(hidden)]
pub struct CssCollection {
    fragments: Mutex<Vec<String>>,
}

impl CssCollection {
    pub fn new() -> Self {
        Self {
            fragments: Mutex::new(Vec::new()),
        }
    }

    pub fn register(&self, css: String) {
        let mut lock = self.fragments.lock().expect("Could not acquire lock");
        lock.push(css);
    }

    pub fn get_css(&self) -> String {
        let lock = self.fragments.lock().expect("Could not acquire lock");
        lock.join("\n")
    }
}
