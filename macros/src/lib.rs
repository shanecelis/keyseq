#![doc(html_root_url = "https://docs.rs/keyseq_macros/0.2.2")]
#![doc = include_str!("../README.md")]
extern crate proc_macro;
#[allow(unused_imports)]
use proc_macro2::{Ident, Literal, Span, TokenStream, TokenTree};
#[allow(unused_imports)]
use proc_macro_error::{abort, abort_call_site, emit_call_site_warning, proc_macro_error};
use quote::quote;
#[allow(unused_imports)]
use std::borrow::Cow;

#[cfg(feature = "winit")]
mod winit;

#[cfg(feature = "bevy")]
mod bevy;

/// Short hand notation describes a physical key chord as `(modifiers: u8,
/// key_code: &str)`.
///
/// Specify a key and any modifiers.
///
/// ```
/// # use keyseq_macros::poor_pkey as pkey;
/// assert_eq!(pkey!(A), (0, "A"));
/// assert_eq!(pkey!(ctrl-A), (1, "A"));
/// assert_eq!(pkey!(alt-A), (2, "A"));
/// assert_eq!(pkey!(shift-A), (4, "A"));
/// assert_eq!(pkey!(super-A), (8, "A"));
/// assert_eq!(pkey!(ctrl-alt-;), (3, "Semicolon"));
/// assert_eq!(pkey!(1), (0, "Key1"));
/// assert_eq!(pkey!(alt-1), (2, "Key1"));
/// ```
///
/// More than one key will cause a panic at compile-time. Use keyseq! for that.
///
/// ```compile_fail
/// # use keyseq_macros::poor_pkey as pkey;
/// fn too_many_keys() {
///     let _ = pkey!(A B);
/// }
/// ```
///
/// This macro does not ensure the key names exist.
///
/// ```
/// # use keyseq_macros::poor_pkey as pkey;
/// assert_eq!(pkey!(alt-NoSuchKey), (2, "NoSuchKey"));
/// ```
#[cfg(feature = "poor")]
#[proc_macro_error]
#[proc_macro]
pub fn poor_pkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (result, leftover) = read_key_chord(input.into(), to_modifiers_u8, get_pkey);
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

#[cfg(feature = "bevy")]
#[proc_macro_error]
#[proc_macro]
pub fn bevy_pkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (result, leftover) = read_key_chord(input.into(), to_keyseq_modifiers, bevy::get_pkey);
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

/// Short hand notation describes a physical key chord as `(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key_code: `[winit::keyboard::KeyCode][keycode]`)`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [keycode]: https://docs.rs/winit/latest/winit/keyboard/enum.KeyCode.html
#[cfg(feature = "winit")]
#[proc_macro_error]
#[proc_macro]
pub fn winit_pkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let (result, leftover) = read_key_chord(input.into(), winit::to_modifiers, winit::get_pkey);
    let (result, leftover) = read_key_chord(input.into(), to_keyseq_modifiers, winit::get_pkey);
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

/// Short hand notation describes a logical key chord as `(modifiers: u8,
/// key_code: &str)`.
///
/// Specify a key and any modifiers.
///
/// ```
/// # use keyseq_macros::poor_lkey as key;
/// assert_eq!(key!(a), (0, "a"));
/// assert_eq!(key!(A), (0, "A"));
/// assert_eq!(key!(ctrl-A), (1, "A"));
/// assert_eq!(key!(alt-A), (2, "A"));
/// assert_eq!(key!(shift-A), (4, "A"));
/// assert_eq!(key!(super-A), (8, "A"));
/// assert_eq!(key!(ctrl-alt-;), (3, ";"));
/// assert_eq!(key!(1), (0, "1"));
/// assert_eq!(key!(alt-1), (2, "1"));
/// ```
///
/// More than one key will cause a panic at compile-time. Use keyseq! for that.
///
/// ```compile_fail
/// # use keyseq_macros::poor_key as key;
/// fn too_many_keys() {
///     let _ = key!(A B);
/// }
/// ```
#[cfg(feature = "poor")]
#[proc_macro_error]
#[proc_macro]
pub fn poor_lkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (result, leftover) = read_key_chord(input.into(), to_modifiers_u8, get_key);
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

/// Short hand notation describes a logical key chord as `(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key: `[winit::keyboard::Key][key]`)`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [key]: https://docs.rs/winit/latest/winit/keyboard/enum.Key.html
#[cfg(feature = "winit")]
#[proc_macro_error]
#[proc_macro]
pub fn winit_lkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let (result, leftover) = read_key_chord(input.into(), winit::to_modifiers, winit::get_key);
    let (result, leftover) = read_key_chord(input.into(), to_keyseq_modifiers, winit::get_key);
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

// #[cfg(feature = "bevy")]
// #[proc_macro_error]
// #[proc_macro]
// pub fn bevy_lkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let (result, leftover) = read_key_chord(input.into(), bevy::to_modifiers, bevy::get_key);
//     if !leftover.is_empty() {
//         abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
//     }
//     result.into()
// }

/// Short hand notation describes a sequence of logical key chords as
/// `[(modifiers: u8, key_code: &str)]`.
///
/// ```
/// use keyseq_macros::poor_pkeyseq as keyseq;
/// assert_eq!(keyseq!(A B), [(0, "A"), (0, "B")]);
/// assert_eq!(keyseq!(shift-A ctrl-B), [(4, "A"), (1, "B")]);
/// ```
///
/// When no features are enabled, there are no smarts to check whether a key is real
/// or not.
///
/// ```
/// use keyseq_macros::poor_pkeyseq as keyseq;
/// assert_eq!(keyseq!(A NoSuchKey), [(0, "A"), (0, "NoSuchKey")]);
/// ```
///
#[cfg(feature = "poor")]
#[proc_macro_error]
#[proc_macro]
pub fn poor_lkeyseq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let keys = read_key_chords(input.into(), to_modifiers_u8, get_key);
    quote! {
        [#(#keys),*]
    }
    .into()
}

/// Short hand notation describes a sequence of physical key chord as `[(modifiers:
/// u8, key_code: &str)]`.
///
/// [keycode]: https://docs.rs/bevy/latest/bevy/prelude/enum.KeyCode.html
#[cfg(feature = "poor")]
#[proc_macro_error]
#[proc_macro]
pub fn poor_pkeyseq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let keys = read_key_chords(input.into(), to_modifiers_u8, get_pkey);
    quote! {
        [#(#keys),*]
    }
    .into()
}

#[cfg(feature = "bevy")]
#[proc_macro_error]
#[proc_macro]
pub fn bevy_pkeyseq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let keys = read_key_chords(input.into(), to_keyseq_modifiers, bevy::get_pkey);
    quote! {
        [#(#keys),*]
    }
    .into()
}

/// Short hand notation describes a sequence of physical key chord as `[(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key_code: `[winit::keyboard::KeyCode][keycode]`)]`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [keycode]: https://docs.rs/winit/latest/winit/keyboard/enum.KeyCode.html
#[cfg(feature = "winit")]
#[proc_macro_error]
#[proc_macro]
pub fn winit_pkeyseq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let keys = read_key_chords(input.into(), winit::to_modifiers, winit::get_pkey);
    let keys = read_key_chords(input.into(), to_keyseq_modifiers, winit::get_pkey);
    quote! {
        [#(#keys),*]
    }
    .into()
}

/// Short hand notation describes a sequence of logical key chord as `[(modifiers:`
/// [winit::keyboard::ModifiersState][mods]`, key: `[winit::keyboard::Key][key]`)]`.
///
/// [mods]: https://docs.rs/winit/latest/winit/keyboard/struct.ModifiersState.html
/// [key]: https://docs.rs/winit/latest/winit/keyboard/enum.Key.html
#[cfg(feature = "winit")]
#[proc_macro_error]
#[proc_macro]
pub fn winit_lkeyseq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let keys = read_key_chords(input.into(), winit::to_modifiers, winit::get_key);
    let keys = read_key_chords(input.into(), to_keyseq_modifiers, winit::get_key);
    quote! {
        [#(#keys),*]
    }
    .into()
}

#[cfg(any(feature = "winit", feature = "bevy", feature = "poor"))]
fn read_key_chords<F, G>(mut input: TokenStream, to_modifiers: F, get_key: G) -> Vec<TokenStream>
where
    F: Fn(u8) -> TokenStream,
    G: Fn(TokenTree) -> Option<TokenStream>,
{
    let mut keys = vec![];

    loop {
        let (result, leftover) = read_key_chord(input, &to_modifiers, &get_key);
        keys.push(result);
        if leftover.is_empty() {
            break;
        }
        input = leftover;
    }
    keys
}

#[cfg(feature = "poor")]
fn key_code_path(id: Ident) -> TokenStream {
    let x = format!("{}", id);
    let s = proc_macro2::Literal::string(&x);
    quote! { #s }
}

#[cfg(feature = "poor")]
fn get_pkey(tree: TokenTree) -> Option<TokenStream> {
    match tree {
        TokenTree::Literal(ref literal) => {
            let x = literal.span().source_text().unwrap();
            if x.len() == 1 && x.parse::<u8>().is_ok() {
                Some(Ident::new(&format!("Key{x}"), Span::call_site()))
                // Some(Ident::new("Keyx", Span::call_site()))
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
                    'A'..='Z' => Some(label.into()),
                    x @ 'a'..='z' => {
                        abort!(x, "Use uppercase key names for physical keys");
                        // let s = x.to_ascii_uppercase().to_string();
                        // Some(s.into())
                    }
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

#[allow(dead_code)]
#[rustfmt::skip]
#[derive(Clone, Copy)]
enum Modifier {
    None    = 0,
    // Use the OS X Human interface guidelines order.
    Control = 1,
    Alt     = 2,
    Shift   = 3,
    Super   = 4,
}

impl Modifier {
    #[allow(dead_code)]
    fn bitflag(&self) -> u8 {
        let mut number = *self as u8;
        if number != 0 {
            number = 1 << (number - 1);
        }
        number
    }
}

#[allow(dead_code)]
fn to_keyseq_modifiers(bitflags: u8) -> TokenStream {
    let x = proc_macro2::Literal::u8_suffixed(bitflags);
    quote! { ::keyseq::Modifiers(#x) }
}

#[cfg(feature = "poor")]
#[allow(unused_variables)]
fn to_modifiers_u8(bitflags: u8) -> TokenStream {
    let x = proc_macro2::Literal::u8_suffixed(bitflags);
    quote! { #x }
}

#[cfg(feature = "poor")]
fn get_key(tree: TokenTree) -> Option<TokenStream> {
    get_key_raw(tree).map(|r| match r {
        Ok(c) => {
            let l = Literal::string(&c.to_string());
            quote! { #l }
        }
        Err(cow) => {
            let l = Literal::string(&cow);
            quote! { #l }
        }
    })
}

#[cfg(any(feature = "poor", feature = "winit"))]
fn get_key_raw(tree: TokenTree) -> Option<Result<char, Cow<'static, str>>> {
    match tree {
        TokenTree::Literal(ref literal) => {
            let x = literal.span().source_text().unwrap();
            if x.len() == 1 {
                Some(Ok(x.chars().next().unwrap()))
            } else {
                let name = match x.as_str() {
                    "'\\''" => Some("Apostrophe"),
                    "'`'" => Some("Grave"),
                    "'\\\\'" => Some("Backslash"),
                    _ => todo!("literal char {x} {:?}", literal),
                };
                Some(Err(name.map(|n| n.to_string()).unwrap_or(x).into()))
            }
        }
        TokenTree::Punct(ref punct) => Some(Ok(punct.as_char())),
        TokenTree::Ident(ref ident) => {
            let label = ident.span().source_text().unwrap();
            if label.len() == 1 {
                Some(Ok(label.chars().next().unwrap()))
            } else {
                Some(Err(label.into()))
            }
        }
        _ => None,
    }
}

#[cfg(any(feature = "winit", feature = "bevy", feature = "poor"))]
fn read_modifiers<F: Fn(u8) -> TokenStream>(
    input: TokenStream,
    to_modifiers: F,
) -> (TokenStream, TokenStream) {
    let mut i = input.into_iter().peekable();
    let mut last_tree = None;

    fn is_dash(tree: &TokenTree) -> bool {
        match tree {
            TokenTree::Punct(ref punct) => punct.as_char() == '-',
            _ => false,
        }
    }
    let mut bitflags: u8 = 0;

    let mut accum_mods = |modifier: Modifier| {
        let bitflag = modifier.bitflag();
        if bitflag < bitflags {
            // emit_warning!(gcc
            // emit_call_site_warning!("Modifiers must occur in this order: control, alt, shift, super.");
            if cfg!(feature = "strict-order") {
                abort_call_site!("Modifiers must occur in this order: control, alt, shift, super.");
            } else {
                emit_call_site_warning!(
                    "Modifiers must occur in this order: control, alt, shift, super."
                );
            }
        }
        bitflags |= bitflag;
    };

    while let Some(tree) = i.next() {
        if i.peek().is_none() || (!is_dash(&tree) && !is_dash(i.peek().unwrap())) {
            last_tree = Some(tree);
            break;
        } else {
            match tree {
                TokenTree::Ident(ref ident) => match ident.span().source_text().unwrap().as_str() {
                    "ctrl" => accum_mods(Modifier::Control),
                    "alt" => accum_mods(Modifier::Alt),
                    "shift" => accum_mods(Modifier::Shift),
                    "super" => accum_mods(Modifier::Super),
                    x => abort!(x, "Should be a modifier or a hyphen"),
                },
                TokenTree::Punct(ref punct) => match punct.as_char() {
                    // We could allow + notation too.
                    '-' => {}
                    x => abort!(x, "Should be a modifier or a hyphen"),
                },
                x => abort!(x, "Should be a modifier or a hyphen"),
            };
        }
    }
    // This will add an empty to finish the expression:
    //
    //    ctrl-alt-EMPTY -> Control | Alt | EMPTY.
    //
    //  And it will provide a valid Modifier when none have been provided.
    // r.extend([to_modifiers(Modifier::None)]);

    // let x = proc_macro2::Literal::u8_suffixed(bitflags);
    (
        // r,
        to_modifiers(bitflags),
        // quote!{ ::keyseq::Modifiers(#x) },
        TokenStream::from_iter(last_tree.into_iter().chain(i)),
    )
}

#[cfg(any(feature = "winit", feature = "bevy", feature = "poor"))]
fn read_key<F: Fn(TokenTree) -> Option<TokenStream>>(
    input: TokenStream,
    get_key: F,
) -> (TokenStream, TokenStream) {
    let mut i = input.into_iter();
    let tree = i.next().expect("No token tree");
    let key = get_key(tree).expect("No key found");
    (
        quote! {
            #key
        },
        TokenStream::from_iter(i),
    )
}

#[cfg(any(feature = "winit", feature = "bevy", feature = "poor"))]
fn read_key_chord<F, G>(
    input: TokenStream,
    to_modifiers: F,
    get_key: G,
) -> (TokenStream, TokenStream)
where
    F: Fn(u8) -> TokenStream,
    G: Fn(TokenTree) -> Option<TokenStream>,
{
    let (mods, input) = read_modifiers(input, to_modifiers);
    let (key, rest) = read_key(input, get_key);
    (
        quote! {
            (#mods, #key)
        },
        rest,
    )
}
