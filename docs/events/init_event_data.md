## Rust Documentation : 'events.rs' - Function init_event_data

### &emsp;**Implementations:**

```rust 
    pub fn init_event_data(
        current_event_type: String,
        shooter_h: i16, 
        victim_h: i16, 
        idd: bool, 
        current_used_items: Option<[String; 3]>
    ) -> EventData {}
```
<br/>

You can use this function for initialize an event object. Exemple :
```rust
        let mut event_data: EventData = events::init_event_data(
            "shot".to_string(),
            player_data.health,
            dealer_data.health,
            false,
            None
        );
```