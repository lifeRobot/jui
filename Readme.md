jui is a ui set based on slint  
function soon

### hello world example

Cargo.toml file :

```toml
[dependencies]
slint = "1.3.2"

[build-dependencies]
jui_file = "0.1.0"
jui = "0.1.0"
```

main.rs file :

```rust
slint::include_modules!();

fn main() {
    App::new().unwrap().run().unwrap();
}
```

build.rs file :

```rust
pub fn main() {
    let separator = jui_file::separator;
    jui::compile(format!("view{separator}main.slint")).unwrap();
}
```

view/main.slint file :

```slint
import { HelloWorld } from "@jui/index.slint";
import { Test } from "@jui/test.slint";

export component App inherits Window {
    min-width: 300px;
    min-height: 300px;

    VerticalLayout {
        height: 30px;
        HelloWorld {}
        Test{}
    }
}
```