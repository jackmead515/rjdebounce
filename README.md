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

let mut bouncer = Bouncer::new(Duration::from_secs(1)).with_func(|| 5 + 5);

let result1 = bouncer.get_result();
bouncer.execute();
let result2 = bouncer.get_result();

assert_equal!(result.is_some(), false);
assert_equal!(result.is_some(), true);
```

## Do I plan to expand on this library?

Yeah absolutely. I'd love to make a bigger lib.

## Should I use this in production?

I mean... sure, if you want. But this is just practice for me. I do
not have a licence for this and I purely wrote it
to get a feel for cargo and package management. I will
use this in my personal projects though. So feel free!