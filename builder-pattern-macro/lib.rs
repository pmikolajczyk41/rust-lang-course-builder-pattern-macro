use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

type SynResult<T> = Result<T, syn::Error>;

#[proc_macro_attribute]
pub fn builder_pattern(attr: TokenStream, item: TokenStream) -> TokenStream {
    match builder_pattern_internal(attr, item) {
        Ok(ts) => ts.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Auxiliary function enabling two convenient things:
/// - `?` operator usage
/// - working with `TokenStream2` instead of `TokenStream` objects
fn builder_pattern_internal(attr: TokenStream, item: TokenStream) -> SynResult<TokenStream2> {
    let ir = IR::try_from((attr, item))?;
    generate_code(ir)
}

/// Intermediate representation of the input (parsing arguments and handling AST).
mod ir {
    pub struct IR;

    impl TryFrom<(TokenStream, TokenStream)> for IR {
        type Error = syn::Error;

        fn try_from((attr, item): (TokenStream, TokenStream)) -> Result<Self, Self::Error> {
            todo!()
        }
    }
}

/// Converts intermediate representation into the final code.
mod codegen {
    use proc_macro2::TokenStream as TokenStream2;

    use super::{
        ir::IR,
        SynResult,
    };

    pub fn generate_code(ir: IR) -> SynResult<TokenStream2> {
        todo!()
    }
}
