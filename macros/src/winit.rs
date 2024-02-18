use super::get_key_raw;
use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
use proc_macro_error::abort;
use quote::quote;
use std::borrow::Cow;

pub fn get_key(tree: TokenTree) -> Option<TokenStream> {
    get_key_raw(tree).map(|r| match r {
        Ok(c) => {
            let l = Literal::character(c);
            quote! { ::winit::keyboard::Key::Character(#l) }
        }
        Err(cow) => {
            let i = Ident::new(&cow, Span::call_site());
            quote! { ::winit::keyboard::Key::Named(::winit::keyboard::NamedKey::#i) }
        }
    })
}

pub fn get_pkey(tree: TokenTree) -> Option<TokenStream> {
    match tree {
        TokenTree::Literal(ref literal) => {
            let x = literal.span().source_text().unwrap();
            if x.len() == 1 && x.parse::<u8>().is_ok() {
                eprintln!("got numeric literal {:?}", x);
                Some(Ident::new(&format!("Digit{x}"), Span::call_site()))
            } else {
                let name = match x.as_str() {
                    "'\\''" => Some("Apostrophe"),
                    "'`'" => Some("Grave"),
                    "'\\\\'" => Some("Backslash"),
                    _ => todo!("literal char {x} {:?}", literal),
                };
                name.map(|x| Ident::new(x, Span::call_site()))
            }
        }
        TokenTree::Punct(ref punct) => {
            let name: Option<&str> = match punct.as_char() {
                ';' => Some("Semicolon"),
                ':' => {
                    // TODO: `ctrl-:` Can't be entered on a US ANSI
                    // keyboard only `shift-;` can. Make docs clear this
                    // is the key and not the symbol?

                    // add_shift = true;
                    // Some("Semicolon")
                    Some("Colon")
                }
                ',' => Some("Comma"),
                '.' => Some("Period"),
                '^' => Some("Caret"),
                '=' => Some("Equals"),
                '/' => Some("Slash"),
                '-' => Some("Minus"),
                '*' => Some("Asterisk"),
                '+' => Some("Plus"),
                '@' => Some("At"),
                // _ => None
                _ => todo!("punct {:?}", punct),
            };
            name.map(|n| Ident::new(n, punct.span()))
        }
        TokenTree::Ident(ref ident) => {
            let label = ident.span().source_text().unwrap();
            if label.len() == 1 {
                let name: Option<Cow<'static, str>> = match label.chars().next().unwrap() {
                    // x @ 'A'..='Z' => {
                    x @ 'A'..='Z' => {
                        // I'm not sure I like adding shift.
                        // add_shift = true;
                        // Some(x.to_string().into())
                        Some(format!("Key{x}").into())
                    }
                    x @ 'a'..='z' => {
                        abort!(x, "Use uppercase key names for physical keys");
                        // let s = x.to_ascii_uppercase().to_string();
                        // Some(s.into())
                    }
                    '_' => Some("Underline".into()),
                    _ => todo!("ident {:?}", ident),
                };
                name.as_ref().map(|n| Ident::new(n, ident.span()))
            } else {
                Some(ident.clone())
            }
        }
        _ => None,
    }
    .map(key_code_path)
}

// pub fn to_modifiers(bitflags: u8) -> TokenStream {
//
//     quote! { ::winit::keyboard::ModifiersState::#id }
// }

fn key_code_path(id: Ident) -> TokenStream {
    quote! { ::winit::keyboard::KeyCode::#id }
}
