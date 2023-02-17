use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Token,
};

pub struct LibReturnMacroInput(pub Punctuated<Expr, Token![,]>);
impl Parse for LibReturnMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
        Ok(Self(parser(input)?))
    }
}
