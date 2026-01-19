---
name: create-buff
description: Create a new Buff and implement its functionality
---

# Create Buff

Create a Buff component to implement game functionality

## Example

Using Fiora's E ability as an example

### 1. Create Buff File

Create a new .rs file in the `src/buffs` directory. Name the file using champion name + ability name. In this example, the file name is `fiora_e.rs`.
For generic buffs, name them by function with noun first then verb, e.g. `attack_speed_up.rs`.

### 2. Write Buff File

Add the plugin and Buff component definition in `src/buffs/fiora_e.rs`:

```rs
use bevy::prelude::*;

use crate::{Buff, Buffs, Lifetime};

#[derive(Default)]
pub struct PluginFioraE;

impl Plugin for PluginFioraE {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Clone, Debug)]
#[require(Buff = Buff { name: "FioraE" }, Lifetime = Lifetime::new_timer(3.0))]
pub struct BuffFioraE {
    pub left: i32,
}

impl Default for BuffFioraE {
    fn default() -> Self {
        Self { left: 2 }
    }
}
```

If the Buff lasts forever, you don't need to require the Lifetime component

### 3. Register Buff Module and Plugin

Add the Buff module in `src/buffs.rs`:

```rs
// ...other mods
mod fiora_e;

// ...other uses
pub use fiora_e::*;
```

Register the Buff plugin in `src/lib.rs`:

```rs
plugin_group! {
    pub struct PluginCore {
        // ...other Buff plugins
        :PluginFioraE,
    }
}
```

### 4. Implement Buff Logic

Implement the Buff logic. For example, Fiora's E ability decreases by one on each attack end, and removes the Buff when the count reaches 0.

```rs
fn on_event_attack_end(
    trigger: On<EventAttackEnd>,
    mut commands: Commands,
    q_buffs: Query<&Buffs>,
    mut q_buff_fiora_e: Query<&mut BuffFioraE>,
) {
    let entity = trigger.event_target();
    let Ok(buffs) = q_buffs.get(entity) else {
        return;
    };

    for buff in buffs.iter() {
        let Ok(mut buff_fiora_e) = q_buff_fiora_e.get_mut(buff) else {
            continue;
        };

        buff_fiora_e.left -= 1;

        if buff_fiora_e.left <= 0 {
            commands.entity(buff).despawn();
        }
    }
}
```

Add the observer in the plugin:

```rs
impl Plugin for PluginFioraE {
    fn build(&self, app: &mut App) {
        app.add_observer(on_event_attack_end);
    }
}
```

### 5. Add Buff

Add the Buff in the ability effect:

```rs
                Behave::trigger(ActionBuffSpawn::new((
                    BuffAttack {
                        bonus_attack_speed: 0.5,
                    },
                    BuffFioraE::default()
                ))),
```

Since Fiora's E ability comes with attack speed increase, the two Buffs are combined and attached to the same Buff entity. BuffFioraE manages the count, BuffAttack manages the attack speed, thus achieving Buff effect composition.
