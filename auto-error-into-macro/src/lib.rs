use proc_macro::{TokenStream, TokenTree};
use quote::{format_ident, ToTokens};
use syn::{parse_macro_input, parse_quote, FnArg, ItemFn, Pat, PatIdent, ReturnType};

/// Wraps a function that returns a Result<T, E> in such a way that will always return the T type, relying on an `Into<T>` implemetation to exist for the E type.
///
/// # Remarks
/// Use `#[auto_error_into(force_inline)]` to force the wrapped function to be inlined. This will only work on a subset of functions, and will never work on methods.
#[proc_macro_attribute]
pub fn auto_error_into(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut func = parse_macro_input!(input as ItemFn);

    let mut original_signature = func.sig.clone();
    let ReturnType::Type(_, t) = func.sig.output else { panic!("Unable to use auto_error_convert on functions that return ()"); };

    let block = func.block;
    func.sig.output = parse_quote!(-> <#t as ::auto_error_into::__::ResultResolver>::Ok);

    let mut args = args.into_iter();

    let first = args.next();

    if args.next().is_some() {
        panic!("More than two arguments to macro");
    }

    match first {
        Some(TokenTree::Ident(arg)) if arg.to_string() == "force_inline" => {
            let parameter_names: Vec<_> = (0u32..)
                .map(|num| format_ident!("param{}", num))
                .take(original_signature.inputs.len())
                .collect();

            let new_args = original_signature
                .inputs
                .clone()
                .into_iter()
                .zip(&parameter_names)
                .map(|(a, ident)| match a {
                    FnArg::Typed(mut typ) => {
                        typ.pat = Box::new(Pat::Ident(PatIdent {
                            attrs: Vec::new(),
                            by_ref: None,
                            mutability: None,
                            ident: ident.clone(),
                            subpat: None,
                        }));
                        FnArg::Typed(typ)
                    }
                    _ => panic!("Cannot force inline on methods"),
                });

            func.sig.inputs = parse_quote!(#(#new_args),*);

            original_signature.ident = format_ident!("__internal_invoke");
            func.block = Box::new(parse_quote!({
                #[inline(always)] #original_signature #block
                match __internal_invoke(#(#parameter_names),*) {
                    Ok(o) => o,
                    Err(e) => e.into(),
                }
            }));

            func.into_token_stream().into()
        }
        Some(_) => panic!("Only supported argument is force_inline"),
        _ => {
            func.block = Box::new(parse_quote!({
                match (move || -> #t #block)() {
                    Ok(o) => o,
                    Err(e) => e.into(),
                }
            }));
            func.into_token_stream().into()
        }
    }
}
