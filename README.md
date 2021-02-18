# Dactyl

This crate contains helpers to format unsigned integers as "pretty" byte strings with commas marking each thousand. To that singular end, they are much faster than using the `format!` macro or an external crate like [`num_format`].

To minimize unnecessary allocation, structs are split up by type:

* [`NiceU8`]
* [`NiceU16`]
* [`NiceU32`]
* [`NiceU64`]

Note: [`NiceU64`] implements both `from<u64>` and `from<usize>`, so can be used for either type.

Working on a similar idea, there is also [`NicePercent`], which implements `from<f32>` and `from<f64>` to convert values between `0..=1` to a "pretty" percent byte string, like `75.66%`.

Last but not least, there is [`NiceElapsed`], which converts numerical time into a human-readable, oxford-joined byte string, like `1 hour, 2 minutes, and 3 seconds`.



## Stability

Release versions of this library should be in a working state, but as this project is under perpetual development, code might change from version to version.



## Installation

Add `dactyl` to your `dependencies` in `Cargo.toml`, like:

```
[dependencies.dactyl]
git = "https://github.com/Blobfolio/dactyl.git"
tag = "v0.1.*"
```



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
