extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
use quote::{quote, TokenStreamExt};
use syn;

#[proc_macro_derive(EnumStringer, attributes(stringer))]
pub fn enum_stringer_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_enum_stringer_derive(&ast)
}

struct MatchArmExpression((String, String, String));

impl quote::ToTokens for MatchArmExpression {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let MatchArmExpression((name, variant, literal_value)) = self;
        tokens.append(proc_macro2::Ident::new(
            name,
            proc_macro2::Span::call_site(),
        ));
        tokens.append(proc_macro2::Punct::new(':', proc_macro2::Spacing::Joint));
        tokens.append(proc_macro2::Punct::new(':', proc_macro2::Spacing::Joint));
        tokens.append(proc_macro2::Ident::new(
            variant,
            proc_macro2::Span::call_site(),
        ));
        tokens.append(proc_macro2::Punct::new('=', proc_macro2::Spacing::Joint));
        tokens.append(proc_macro2::Punct::new('>', proc_macro2::Spacing::Joint));
        literal_value.to_tokens(tokens);
    }
}

fn impl_enum_stringer_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let a = match &ast.data {
        syn::Data::Enum(de) => de
            .variants
            .iter()
            .map(|variant| {
                let string_for_variant: syn::LitStr = variant.attrs[0].parse_args().unwrap();
                MatchArmExpression((
                    name.to_string(),
                    variant.ident.to_string(),
                    string_for_variant.value(),
                ))
            })
            .collect::<Vec<MatchArmExpression>>(),
        _ => panic!("stringer implementations are only valid for enums"),
    };

    let gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let out = match self {
                      #(#a,)*
                };

                write!(f, "{}", out)
            }
        }
    };

    gen.into()
}
