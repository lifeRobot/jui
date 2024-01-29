jui is a ui set based on slint  
[SurrealismUI](https://github.com/Surrealism-All/SurrealismUI) is already supported
more function coming soon

### Supported components

| name           | import example                                                           |
|----------------|--------------------------------------------------------------------------|
| Button         | ```import { Button } from "@jui/button.slint";```                        |
| InputBase      | ```import { InputBase } from "@jui/input/input_base.slint";```           |
| Input          | ```import { Input } from "@jui/input/input.slint";```                    |
| UnderlineInput | ```import { UnderlineInput } from "@jui/input/underline_input.slint";``` |

### [SurrealismUI](https://github.com/Surrealism-All/SurrealismUI) example

features enable surrealism_ui, Cargo.toml file like example:

```toml
[dependencies]
slint = "1.3.2"

[build-dependencies]
jui = { version = "0.1.3", features = ["surrealism_ui"] }
```

use SurrealismUI in you slint file:

```slint
// support all component
// import { SButton } from "@surrealism_all";

// support single component
import { SButton } from "@surrealism/button/index.slint";

export component App inherits Window {
    SButton {}
}
```

more SurrealismUI see: [SurrealismUI wiki](https://github.com/Surrealism-All/SurrealismUI/wiki)  
if you want use alias import SurrealismUI, example for this build.rs:

```rust
use jui::surrealism_ui::SurrealismUI;

pub fn main() {
    let separator = jui::jui_file::separator;
    let surrealism_ui = SurrealismUI { alias: "sui".into(), ..Default::default() };
    jui::compile_with_surrealism_ui(format!("view{separator}main.slint"), surrealism_ui).unwrap();
}
```

### Jui Button example

Cargo.toml file :

```toml
[dependencies]
slint = "1.3.2"

[build-dependencies]
jui = "0.1.3"
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
    let separator = jui::jui_file::separator;
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
