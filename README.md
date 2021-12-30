# rgb2hex

Convert RGB values to their hexadecimal equivalent.

## Usage

```rust
use rgb2hex::{rgb2hex, rgb2hex_from_str};

rgb2hex(251, 169, 12);              // 0xfba90c
rgb2hex_from_str("251, 169, 12");   // 0xfba90c
```