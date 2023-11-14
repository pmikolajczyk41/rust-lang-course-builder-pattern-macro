#![no_main]

use builder_pattern_macro::builder_pattern;

#[builder_pattern(overwriting = 12)]
struct S {
    a: u32,
}
