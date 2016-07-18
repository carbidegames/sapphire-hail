use mopa;

pub struct Modules {
    modules: Vec<Box<Module>>,
}

impl Modules {
    pub fn new() -> Self {
        Modules {
            modules: Vec::new(),
        }
    }

    pub fn register<T: Module>(&mut self, module: T) {
        self.modules.push(Box::new(module));
    }

    /// Get a module by type. TODO: Change to a Result to allow try!(modules.get()) in handlers
    pub fn get<T: Module>(&self) -> Option<&T> {
        for module in &self.modules {
            // Check if this module is the one we're looking for
            if let Some(module) = module.downcast_ref::<T>() {
                return Some(module);
            }
        }

        None
    }
}

pub trait Module: mopa::Any + Send + Sync {
}

mopafy!(Module);

#[cfg(test)]
mod tests {
    use {Modules, Module};

    struct TestModule {}
    impl Module for TestModule {}

    #[test]
    fn get_finds_registered() {
        let mut modules = Modules::new();

        modules.register(TestModule{});
        let result: &TestModule = modules.get();

        assert!(result.is_some());
    }
}
