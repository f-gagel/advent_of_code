use std::{error::Error, fmt::Debug};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Ident, LitStr, Token, parse::Parse};

struct DeclPattern {
    pub_token: Option<Token!(pub)>,
    name: Ident,
    _comma: Token!(,),
    pattern: LitStr,
}

impl Parse for DeclPattern {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pub_token: input.parse()?,
            name: input.parse()?,
            _comma: input.parse()?,
            pattern: input.parse()?,
        })
    }
}

enum PatternElement<'a> {
    Literal(&'a str),
    Parse(syn::Type),
}

impl Debug for PatternElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternElement::Literal(lit) => f.debug_tuple("Literal").field(lit).finish(),
            PatternElement::Parse(ty) => f
                .debug_tuple("Parse")
                .field(&ty.to_token_stream().to_string())
                .finish(),
        }
    }
}

impl PartialEq for PatternElement<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PatternElement::Literal(a), PatternElement::Literal(b)) => a == b,
            (PatternElement::Parse(a), PatternElement::Parse(b)) => {
                a.to_token_stream().to_string() == b.to_token_stream().to_string()
            }
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Pattern<'a>(Vec<PatternElement<'a>>);

impl<'a> Pattern<'a> {
    fn from_str(mut s: &'a str) -> Result<Self, Box<dyn Error>> {
        let mut elements = Vec::new();
        while s.is_empty() == false {
            let mut chars = s.chars();
            if chars.next() == Some('{') {
                let ty_len = chars.take_while(|c| *c != '}').count();
                let ty = syn::parse_str(&s[1..ty_len + 1])?;
                elements.push(PatternElement::Parse(ty));
                s = &s[ty_len + 2..];
            } else {
                let lit_len = s.chars().take_while(|c| *c != '{').count();
                if lit_len > 0 {
                    elements.push(PatternElement::Literal(&s[..lit_len]));
                    s = &s[lit_len..];
                }
            }
        }
        Ok(Self(elements))
    }
}

fn parse_body(pattern: Pattern<'_>) -> (impl quote::ToTokens, impl quote::ToTokens) {
    let step_offset = quote!(
        pos += offset;
        s = &s[offset..];
    );

    let steps =
        pattern
            .0
            .iter()
            .enumerate()
            .fold(TokenStream::new(), |mut stream, (i, element)| {
                let var_name = format_ident!("_{i}");
                let parse = match element {
                    PatternElement::Literal(lit) => {
                        quote!(
                            let (#var_name, offset ) = if s.starts_with( #lit ) {
                                ( (), #lit .len() )
                            } else {
                                return Err( ParseError {
                                    error: Box::new( LiteralMismatch{expected: #lit.into(), got: s.into()} ),
                                    position: pos
                                })
                            };
                        )
                    } 
                    PatternElement::Parse(ty) => {
                        quote!(
                            let ( #var_name , offset) = <#ty as PatternParse>::parse(s)
                                .map_err(|err| ParseError{error: Box::new(err), position: pos})?;
                        )
                    }
                };

                let step_offset = step_offset.clone();
                quote!(#parse #step_offset).to_tokens(&mut stream);

                stream
            });

    let collect_tuple = pattern
        .0
        .iter()
        .enumerate()
        .filter_map(|(i, u)| {
            if let PatternElement::Parse(_) = u {
                Some(format_ident!("_{i}"))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let body = quote!(
        use pattern_parse::*;
        let mut pos = 0_usize;
        #steps
        Ok( ( #( #collect_tuple ),* ) )
    );

    let tuple_types = pattern
        .0
        .iter()
        .filter_map(|u| {
            if let PatternElement::Parse(ty) = u {
                Some(ty)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let ty = quote!(
        ( #( #tuple_types ),* )
    );

    (body, ty)
}

fn parse_fn_core(decl: DeclPattern) -> Result<impl quote::ToTokens, Box<dyn Error>> {
    let pat = &decl.pattern.value();
    let pattern = Pattern::from_str(pat)?;
    let (body, res) = parse_body(pattern);
    let pub_token = decl.pub_token;
    let name = decl.name;
    let func = quote!(
        #pub_token fn #name (mut s: &str) -> Result< #res , pattern_parse::ParseError> { #body }
    );

    Ok(func)
}

/// Macro for generating a parsing function
///
/// The first item is the identifier the generated function should have.
/// The second item must be a string literal representing the parsing.
/// - Sections to be parsed into a value are represented by the type name enclosed in `{}` braces.
/// - All non parsed sections must be an exact match
/// - The function returns a tuple containing all parsed values in order
///
/// Examples:
///
/// ```
/// // becomes fn parse(s: &str) -> Result<(i32, i32, i32, i32), pattern_parse::ParseError>
/// pattern_parse::parse_fn!(parse, "target area: x={i32}..{i32}, y={i32}..{i32}");
/// ```
#[proc_macro]
pub fn parse_fn(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let decl = syn::parse_macro_input!(stream as DeclPattern);
    parse_fn_core(decl).unwrap().to_token_stream().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_from_str_blank() {
        use super::*;

        let pattern = Pattern::from_str("").unwrap();
        let expected = Pattern(vec![]);
        assert_eq!(pattern, expected);
    }
    #[test]
    fn pattern_from_str_literal() {
        use super::*;

        let pattern = Pattern::from_str("pure literal").unwrap();
        let expected = Pattern(vec![lit("pure literal")]);
        assert_eq!(pattern, expected);
    }
    #[test]
    fn pattern_from_str_parse() {
        use super::*;

        let pattern = Pattern::from_str("{u32}").unwrap();
        let expected = Pattern(vec![parse("u32")]);
        assert_eq!(pattern, expected);
    }
    #[test]
    fn pattern_from_str_literal_parse() {
        use super::*;

        let pattern = Pattern::from_str("literal{u32}").unwrap();
        let expected = Pattern(vec![lit("literal"), parse("u32")]);
        assert_eq!(pattern, expected);
    }
    #[test]
    fn pattern_from_str_parse_literal() {
        use super::*;

        let pattern = Pattern::from_str("{u32}literal").unwrap();
        let expected = Pattern(vec![parse("u32"), lit("literal")]);
        assert_eq!(pattern, expected);
    }

    fn lit(s: &'static str) -> PatternElement {
        PatternElement::Literal(s)
    }
    fn parse(ty: &str) -> PatternElement {
        PatternElement::Parse(syn::parse_str(ty).unwrap())
    }
}
