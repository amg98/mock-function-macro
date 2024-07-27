use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn};

#[proc_macro_attribute]
pub fn mock_fn(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let func_name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;

    let param_names: Vec<_> = inputs
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                let pat = &*pat_type.pat;
                quote! { #pat }
            } else {
                panic!("Unsupported argument type")
            }
        })
        .collect();

    let param_types: Vec<_> = inputs
        .iter()
        .map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                let ty = &*pat_type.ty;
                quote! { #ty }
            } else {
                panic!("Unsupported argument type")
            }
        })
        .collect();

    let mock_name = format!("MOCK_{}", func_name.to_string().to_uppercase());
    let mock_name = Ident::new(&mock_name, Span::call_site());
    let set_mock_fn_name = format!("set_mock_for_{}", func_name);
    let set_mock_fn_name = Ident::new(&set_mock_fn_name, Span::call_site());
    let clear_mock_fn_name = format!("clear_mock_for_{}", func_name);
    let clear_mock_fn_name = Ident::new(&clear_mock_fn_name, Span::call_site());

    let expanded = quote! {
        use std::borrow::BorrowMut;

        #[cfg(not(test))]
        #input

        #[cfg(test)]
        thread_local! {
            static #mock_name: std::cell::RefCell<(u32, Option<Box<dyn FnMut(#(#param_types),*) #output>>)> = std::cell::RefCell::new((0, None));
        }

        #[cfg(test)]
        fn #func_name(#(#param_names: #param_types),*) #output {
            #mock_name.with(|mock| {
                let mut mock = mock.borrow_mut();
                if let Some(ref mut mock_fn) = mock.1 {
                    mock_fn(#(#param_names),*)
                } else {
                    panic!("No mock has been set");
                }
            })
        }

        #[cfg(test)]
        pub fn #set_mock_fn_name<F>(mock_fn: F)
        where
            F: FnMut(#(#param_types),*) #output + 'static,
        {
            #mock_name.with(|mock| {
                let mut mock = mock.borrow_mut();
                if mock.0 > 0 {
                    panic!("You forgot to clear this mock");
                }
                mock.1 = Some(Box::new(mock_fn));
                mock.0 = mock.0 + 1;
            });
        }

        #[cfg(test)]
        pub fn #clear_mock_fn_name() {
            #mock_name.with(|mock| {
                let mut mock = mock.borrow_mut();
                if mock.0 == 0 {
                    panic!("This mock is already cleared");
                }
                mock.1 = None;
                mock.0 = mock.0 - 1;
            });
        }
    };

    expanded.into()
}
