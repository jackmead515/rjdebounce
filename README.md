# rjdebounce

#### Super simple library to do a super simple thing. Debounce a function. Zero dependencies.

## Example

```rust
use rjdebounce::Bouncer;

let mut bouncer = Bouncer::new(Duration::from_secs(1));

let result = bouncer.debounce(|| {
    return 5 + 5;
})

assert_eq!(result.is_some(), true);
assert_eq!(result.unwrap(), 10);
```
## OR...
```rust
use rjdebounce::Bouncer;

let func = || 5 + 6;
let delay = Duration::from_secs(1);

let mut bouncer = Bouncer::new(delay).with_func(func);

assert_eq!(bouncer.get_result().is_some(), false);
bouncer.execute();
assert_eq!(bouncer.get_result().is_some(), true);
```

## Do I plan to expand on this library?

Yeah absolutely. I'd love to make a bigger lib.

## Should I use this in production?

I mean... sure, if you want. But this is just practice for me. I do
not have a licence for this and I purely wrote it
to get a feel for cargo and package management. I will
use this in my personal projects though. So feel free!