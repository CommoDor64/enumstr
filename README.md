# enumstr
A rust derive macro for Display trait implementation for enums

This create contains a single macro, namely, a derive creating a Display implementation for enums

## Usage
```rust
#[derive(EnumStringer)]
enum SomeStringer {
    #[stringer("unit_variant")]
    UnitVariant,
}

#[test]
fn should_succeed_on_stringer_enum_variants() {
    assert_eq!("unit_variant", format!("{}", SomeStringer::UnitVariant));
    assert_eq!("unit_variant", SomeStringer::UnitVariant.to_string());
}

```
