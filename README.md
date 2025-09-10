# 🚀 Apex Library

   Apex Library

Fast, modular, and fun Rust library



A Rust library to simplify tasks and experiments with modular design. Supports features like **core utilities**, **window handling**, and **structured data**.

```
                         _ _ _     
  __ _ _ __   _____  __ | (_) |__  
 / _` | '_ \ / _ \ \/ / | | | '_ \ 
| (_| | |_) |  __/>  <  | | | |_) |
 \__,_| .__/ \___/_/\_\ |_|_|_.__/ 
      |_|                                              
        
```

---

## ⚙️ Features

- `core`: Core utilities (labels, string operations, etc.)
- `features`: Extra functionalities (window creation, etc.)
- `structs`: Structured data modules
- `tests`: Tests of some units and functions
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


// open window 
Open a 400x400 window
open_window(400, 400);
`oapex_window();`

// **And many ready-made functions and commands**

# ⚡ Notes

    Fully compatible with Linux, macOS, and Windows

    Automatic library validation before build

    Modular design for easy expansion

    This is a preliminary version and its files may change at any time.
