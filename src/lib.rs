use std::time::{Duration, Instant};

pub struct Bouncer {
    pub delay: Duration,
    last_run: Option<Instant>,
}

impl Bouncer {

    pub fn new(delay: Duration) -> Self {
        return Bouncer {
            delay,
            last_run: None
        };
    }

    pub fn debounce<T>(&mut self, func: fn() -> T) -> Option<T> {
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
