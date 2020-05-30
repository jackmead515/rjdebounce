use std::time::{Duration, Instant};

pub struct Bouncer<T> {
    pub delay: Duration,
    last_run: Option<Instant>,
    func: Option<fn() -> T>,
    result: Option<T>,
}

impl<T> Bouncer<T> {

    /// Creates a new bouncer object
    /// 
    /// Example Usage:
    /// ```rust
    ///     use rjdebounce::Bouncer;
    /// 
    ///     let mut bouncer = Bouncer::new(Duration::from_secs(1));
    /// 
    ///     let result = bouncer.debounce(|| {
    ///         return 5 + 5;
    ///     })
    ///  
    ///     assert_eq!(result.is_some(), true);
    ///     assert_eq!(result.unwrap(), 10);
    /// ```
    pub fn new(delay: Duration) -> Self {
        return Bouncer {
            delay,
            last_run: None,
            func: None,
            result: None,
        };
    }

    /// Binds a function internally to call using 
    /// the execute() function.
    /// 
    /// Example Usage:
    /// ```rust
    ///     use rjdebounce::Bouncer;
    /// 
    ///     let mut bouncer = Bouncer::new(Duration::from_secs(1)).with_func(|| 5 + 5);
    ///     
    ///     let result1 = bouncer.get_result();
    ///     bouncer.execute();
    ///     let result2 = bouncer.get_result();
    /// 
    ///     assert_equal!(result.is_some(), false);
    ///     assert_equal!(result.is_some(), true);
    /// ```
    pub fn with_func(mut self, func: fn() -> T) -> Self {
        self.func = Some(func);
        return self;
    }

    /// If Bouncer is created with_func(), call this
    /// function to execute the provided function.
    pub fn execute(&mut self) {
        if self.func.is_some() {
            let result = self.debounce(self.func.unwrap());
            self.result = result;
        }
    }

    /// If Bouncer is created with_func(), call this
    /// function in order to obtain a reference
    /// to the result.
    pub fn get_result(&mut self) -> Option<&mut T> {
        return self.result.as_mut();
    }
    
    /// debounces provided function only running
    /// if it has never been run, or, if the elasped 
    /// time has past since the function was last run.
    pub fn debounce(&mut self, func: fn() -> T) -> Option<T> {
        if self.last_run.is_some() {
            let then = self.last_run.unwrap();
            let now = Instant::now();

            if now.duration_since(then) > self.delay {
                self.last_run = Some(Instant::now());

                return Some(func());
            } else {
                return None;
            }
        } else {
            self.last_run = Some(Instant::now());
            return Some(func());
        }
    }

    /// Resets the last_run property
    /// to None. Makes the bouncer
    /// act as if it never ran a function.
    pub fn reset(&mut self) {
        self.last_run = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let delay = Duration::from_secs(1);
        let mut bouncer = Bouncer::new(delay);

        let result = bouncer.debounce(|| {
            return 5 + 6;
        });

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), 11);
    }

    #[test]
    fn it_binds_internal_func() {
        let func = || 5 + 6;
        let delay = Duration::from_secs(1);

        let mut bouncer = Bouncer::new(delay).with_func(func);

        assert_eq!(bouncer.get_result().is_some(), false);
        bouncer.execute();
        assert_eq!(bouncer.get_result().is_some(), true);
    }

    #[test]
    fn it_debounces() {
        let delay = Duration::from_millis(100);
        let mut bouncer = Bouncer::new(delay);

        let func = || {
            return 5 + 6;
        };

        let result1 = bouncer.debounce(func);
        let result2 = bouncer.debounce(func);

        std::thread::sleep(Duration::from_millis(101));

        let result3 = bouncer.debounce(func);

        assert_eq!(result1.is_some(), true);
        assert_eq!(result2.is_none(), true);
        assert_eq!(result3.is_some(), true);
    }
}
