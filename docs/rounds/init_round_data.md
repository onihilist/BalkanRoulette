## Rust Documentation : 'rounds.rs' - Function init_round_data

### &emsp;**Implementations:**

```rust
pub fn init_round_data() -> RoundData {}
```

This function create this object : 
```rust
    RoundData {
        round: 1,
        stage: 1,
        round_winner: ["null".to_string(), "null".to_string(), "null".to_string(), "null".to_string(), "null".to_string()],
        stage_winner: ["null".to_string(), "null".to_string(), "null".to_string()]
    }
```

Soon this object will be updated for the different game difficulties.