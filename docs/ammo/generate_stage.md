## Rust Documentation : 'ammo.rs' - Function generate_stage_{...}

### &emsp;**Implementations:**

&emsp;&emsp;&emsp;**Func ammo::generate_stage_one() -> [i16;3] :**
<br/>

```rust 
    pub fn generate_stage_one() -> [i16;3] {

    let total_ammo: i16 = rand::thread_rng().gen_range(2..4);
    let blank_ammo: i16 = rand::thread_rng().gen_range(1..total_ammo);
    let letal_ammo: i16 = total_ammo - blank_ammo;

    return [total_ammo, letal_ammo, blank_ammo];
}
```
<br/>

You can use this function for generate random ammo during stage one. Exemple :
```rust
        // main.rs
        mod ammo;
    
        ammo::generate_stage_one();
```

&emsp;&emsp;&emsp;**Func ammo::generate_stage_two() -> [i16;3] :**
<br/>

```rust 
    pub fn generate_stage_two() -> [i16;3] {
    
        let total_ammo: i16 = rand::thread_rng().gen_range(3..7);
        let substract_ammo: i16 = total_ammo - 1;
        let blank_ammo: i16 = rand::thread_rng().gen_range(2..substract_ammo);
        let letal_ammo: i16 = total_ammo - blank_ammo;
    
        return [total_ammo, letal_ammo, blank_ammo];
    }
```
<br/>

You can use this function for generate random ammo during stage two. Exemple :
```rust
        // main.rs
        mod ammo;
    
        ammo::generate_stage_two();
```