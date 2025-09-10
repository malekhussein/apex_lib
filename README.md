# 🚀 Apex Library

   Apex Library

Fast, modular, and fun Rust library



A Rust library to simplify tasks and experiments with modular design. Supports features like **core utilities**, **window handling**, and **structured data**.

---

## ⚙️ Features

- `core`: Core utilities (labels, string operations, etc.)
- `features`: Extra functionalities (window creation, etc.)
- `structs`: Structured data modules
- `tests`: Unit tests for your modules
- `build.rs` & `check_lib.py`: Automatic library check for any project using it

---

## 🛠 Installation

**Add to your `Cargo.toml`:**

```toml
[dependencies]
apex = { path = "../apex_lib" }
```
# 📝 Usage
```rust
use apex_lib::*;
```
// Labels example
`labels!("one", "two", "three");`

Open a 400x400 window
open_window(400, 400);
`oapex_window();`
# ⚡ Notes

    Fully compatible with Linux, macOS, and Windows

    Automatic library validation before build

    Modular design for easy expansion

    This is a preliminary version and its files may change at any time.
