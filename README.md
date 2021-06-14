# Dactyl

[![Documentation](https://docs.rs/dactyl/badge.svg)](https://docs.rs/dactyl/)
[![crates.io](https://img.shields.io/crates/v/dactyl.svg)](https://crates.io/crates/dactyl)
[![Build Status](https://github.com/Blobfolio/dactyl/workflows/Build/badge.svg)](https://github.com/Blobfolio/dactyl/actions)

This crate provides a fast interface to "stringify" unsigned integers, formatted with commas at each thousand. It prioritizes speed and simplicity over configurability.

If your application just wants to quickly turn `1010` into `"1,010"`, `Dactyl` is a great choice. If your application requires locale awareness or other options, something like [`num-format`](https://crates.io/crates/num-format) would probably make more sense.

Similar to [`itoa`](https://crates.io/crates/itoa), Dactyl writes ASCII conversions to a temporary buffer, but does so using fixed arrays sized for each type's maximum value, minimizing the allocation overhead for, say, tiny little `u8`s.

Each type has its own struct, each of which works exactly the same way:

* [`NiceU8`]
* [`NiceU16`]
* [`NiceU32`]
* [`NiceU64`]

(Note: support for `usize` values is folded into [`NiceU64`].)

The intended use case is to simply call the appropriate `from()` for the type, then use either the `as_str()` or `as_bytes()` struct methods to retrieve the output in the desired format. Each struct also implements traits like `Deref`, `Display`, `AsRef<str>`, `AsRef<[u8]>`, etc., if you prefer those.

```rust
use dactyl::NiceU16;

assert_eq!(NiceU16::from(11234_u16).as_str(), "11,234");
assert_eq!(NiceU16::from(11234_u16).as_bytes(), b"11,234");
```



## Installation

Add `dactyl` to your `dependencies` in `Cargo.toml`, like:

```
[dependencies]
dactyl = "0.1.*"
```



## Other

This crate also contains two "in development" structs — [`NicePercent`] and [`NiceElapsed`] — that can be useful for formatting percentages and durations, however their implementations are subject to change and they might eventually be split off into their own crates.



## License

See also: [CREDITS.md](CREDITS.md)

Copyright © 2021 [Blobfolio, LLC](https://blobfolio.com) &lt;hello@blobfolio.com&gt;

This work is free. You can redistribute it and/or modify it under the terms of the Do What The Fuck You Want To Public License, Version 2.

    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    Version 2, December 2004
    
    Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
    
    Everyone is permitted to copy and distribute verbatim or modified
    copies of this license document, and changing it is allowed as long
    as the name is changed.
    
    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
    
    0. You just DO WHAT THE FUCK YOU WANT TO.
