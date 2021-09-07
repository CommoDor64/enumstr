use error::EnumStringer;
use std::fmt;

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
