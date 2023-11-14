#[cfg(test)]
mod tests {
    use builder_pattern_macro::builder_pattern;

    #[test]
    fn no_fields() {
        #[builder_pattern]
        struct S;

        let _ = S::builder().build().unwrap();
    }

    #[test]
    fn single_field() -> Result<(), String> {
        #[builder_pattern]
        struct S {
            a: i32,
        }

        let s = S::builder().a(0)?.build()?;
        assert_eq!(s.a, 0);

        Ok(())
    }

    #[test]
    fn overwriting() -> Result<(), String> {
        #[builder_pattern(overwriting = true)]
        struct S {
            a: i32,
        }

        let s = S::builder().a(0).a(1).build()?;
        assert_eq!(s.a, 1);

        Ok(())
    }

    #[test]
    fn compile_failures() {
        let t = trybuild::TestCases::new();
        t.compile_fail("src/compile-fail/*.rs");
    }
}
