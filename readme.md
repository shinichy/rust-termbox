# A Rust wrapper for the termbox library.

[Crate documentation](http://apribadi.github.com/rust-termbox/).

The [original termbox library](https://github.com/nsf/termbox) was
created by nsf.

## Install

1. Install [original termbox library](https://github.com/nsf/termbox)
2. Install rust-termbox
        
        $ rustpkg install github.com/shinichy/rust-termbox

## Hello World example

```rust
extern mod std;
extern mod termbox;

use std::io::Timer;
use tb = termbox;

fn main() {
    tb::init();
    tb::print(1, 1, tb::bold, tb::white, tb::black, ~"Hello, world!");
    tb::present();
    let mut timer = Timer::new().unwrap();
    timer.sleep(1000);
    tb::shutdown();
}
```

## License
MIT
