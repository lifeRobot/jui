jui is a ui set based on slint  
[SurrealismUI](https://github.com/Surrealism-All/SurrealismUI) is already supported  
more function coming soon

### Supported components

| name            | import example                                                              |
|-----------------|-----------------------------------------------------------------------------|
| Button          | ```import { Button } from "@jui/button/button.slint";```                    |
| ButtonBase      | ```import { ButtonBase } from "@jui/button/button_base.slint";```           |
| ButtonOpt       | ```import { ButtonBase } from "@jui/button/button_opt.slint";```            |
| ButtonOptBase   | ```import { ButtonOptBase } from "@jui/button/button_opt_base.slint";```    |
| ButtonOptList   | ```import { ButtonOptList } from "@jui/button/button_opt_list.slint";```    |
| InputBase       | ```import { InputBase } from "@jui/input/input_base.slint";```              |
| Input           | ```import { Input } from "@jui/input/input.slint";```                       |
| UnderlineInput  | ```import { UnderlineInput } from "@jui/input/underline_input.slint";```    |
| CheckBox        | ```import { CheckBox } from "@jui/checkbox.slint";```                       |
| SpaceEnter      | ```import { SpaceEnter } from "@jui/event/space_enter.slint";```            |
| TouchSpaceEnter | ```import { TouchSpaceEnter } from "@jui/event/touch_space_enter.slint";``` |
| Collapse        | ```import { Collapse } from "@jui/collapse/collapse.slint";```              |
| Confirm         | ```import { Confirm } from "@jui/message/confirm.slint";```                 |
| Tag             | ```import { Tag } from "@jui/message/tag.slint";```                         |
| Mask            | ```import { Mask } from "@jui/message/mask.slint";```                       |
| DrawerCommon    | ```import { DrawerCommon } from "@jui/drawer/drawer_common.slint";```       |
| Drawer          | ```import { Drawer } from "@jui/drawer/drawer.slint";```                    |
| DrawerTop       | ```import { DrawerTop } from "@jui/drawer/drawer_top.slint";```             |
| DrawerLeft      | ```import { DrawerLeft } from "@jui/drawer/drawer_left.slint";```           |
| DrawerRight     | ```import { DrawerRight } from "@jui/drawer/drawer_right.slint";```         |
| DrawerBottom    | ```import { DrawerBottom } from "@jui/drawer/drawer_bottom.slint";```       |
| Tabs            | ```import { Tabs } from "@jui/nav/tabs.slint";```                           |
| ComboBox        | ```import { ComboBox } from "@jui/combobox/combobox.slint";```              |

### Supported data

| nane              | import example                                                           |
|-------------------|--------------------------------------------------------------------------|
| Position          | ```import { Position } from "@jui/data/position.slint";```               |
| ButtonOptData     | ```import { ButtonOptData } from "@jui/button/button_opt_list.slint";``` |
| TabsData          | ```import { TabsData } from "@jui/nav/tabs.slint";```                    |
| ConfirmData       | ```import { ConfirmData } from "@jui/message/confirm.slint";```          |
| ConfirmTitleData  | ```import { ConfirmTitleData } from "@jui/message/confirm.slint";```     |
| ConfirmButtonData | ```import { ConfirmButtonData } from "@jui/message/confirm.slint";```    |
| ComboBoxData      | ```import { ComboBoxData } from "@jui/combobox/combobox.slint";```       |

### Supported icons

| nane      | import example                                                 |
|-----------|----------------------------------------------------------------|
| IconClose | ```import { IconClose } from "@jui/icons/icon_close.slint";``` |

### [SurrealismUI](https://github.com/Surrealism-All/SurrealismUI) example

features enable surrealism_ui, Cargo.toml file like example:

```toml
[dependencies]
slint = "1.5.1"

[build-dependencies]
jui = { version = "0.1.21", features = ["surrealism_ui"] }
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
    let surrealism_ui = SurrealismUI::new_with_alias("sui");
    jui::compile_with_surrealism_ui(format!("view{separator}main.slint"), surrealism_ui).unwrap();
}
```

### Jui Button example

Cargo.toml file :

```toml
[dependencies]
slint = "1.5.1"

[build-dependencies]
jui = "0.1.21"
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
