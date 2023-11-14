#![no_main]

use builder_pattern_macro::builder_pattern;

#[builder_pattern]
struct S(i32);
