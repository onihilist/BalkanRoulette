## Rust Documentation : 'events.rs' - Type EventData

### &emsp;**Struct(s):**

&emsp;&emsp;&emsp;**Struct events::EventData :**
<br/>

```rust
        pub struct EventData {
            pub event_type: String,
            pub shooter_health: i16,
            pub victim_health: i16,
            pub is_double_dmg: bool,
            pub used_items: Option<[String;3]>
        }
```