jui is a ui set based on slint  
function soon

### Supported components

| name           | import example                                                           |
|----------------|--------------------------------------------------------------------------|
| Button         | ```import { Button } from "@jui/button.slint";```                        |
| InputBase      | ```import { InputBase } from "@jui/input/input_base.slint";```           |
| Input          | ```import { Input } from "@jui/input/input.slint";```                    |
| UnderlineInput | ```import { UnderlineInput } from "@jui/input/underline_input.slint";``` |

### Button example

Cargo.toml file :

```toml
[dependencies]
slint = "1.3.2"

[build-dependencies]
jui_file = "0.1.0"
jui = "0.1.1"
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
import { Button } from "@jui/button.slint";

export component App inherits Window {
    min-width: 300px;
    min-height: 300px;
    
    Rectangle {
        Button {
            color: red;
        }
    }
}
```

### More example

coming soon
