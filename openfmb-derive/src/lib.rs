use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, DeriveInput, Type};
use heck::{ShoutySnakeCase, SnakeCase};


/// Check if the given type path is a known
/// copy type (numeric or bool)
fn is_copy(ty: &Type) -> bool {
    const KNOWN_COPY_TYPES: [&str; 11] = [
        "i8",
        "i16",
        "i32",
        "i64",
        "u8",
        "u16",
        "u32",
        "u64",
        "f32",
        "f64",
        "bool"
    ];
    if let Type::Path(ty) = ty {
        if let Some(check_ident) = ty.path.get_ident() {
            let mut check_ident = check_ident.clone();
            for copy_typ in &KNOWN_COPY_TYPES {
                let known_ident = Ident::new(copy_typ, Span::call_site());
                check_ident.set_span(Span::call_site());
                if known_ident == check_ident {
                    //println!("is_copy match known {:?}, checking {:?}", known_ident, check_ident);
                    return true;
                }
            }
        }
    }
    false
}

fn get_option_ty(ty: &Type) -> Option<Type> {
    let opt_ident = Ident::new("Option", Span::call_site());
    match ty {
        Type::Path(typath) => {
            if let Some(_) = typath.path.get_ident() {
                None
            } else if let Some(pathseg) = typath.path.segments.last() {
                let mut ident = pathseg.ident.clone();
                ident.set_span(Span::call_site());
                if ident == opt_ident {
                    if let syn::PathArguments::AngleBracketed(opt_arg) = &pathseg.arguments {
                        if let Some(syn::GenericArgument::Type(option_ty)) = opt_arg.args.first() {
                            //println!("Option<T> found for {:?}, args {:?}, T {:?}", ty, opt_arg, option_ty);
                            // we found an Option<T> type here, so if T is a real type
                            // we want to return T as a Type
                            Some(option_ty.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None
    }
}

// Determines if a field has #[openfmb(inherit)] as an attribute
fn is_inherit(field: &syn::Field) -> bool {
    let attrs = &field.attrs;
    let openfmb_ident = Ident::new("openfmb", Span::call_site());
    let inherit_ident = Ident::new("inherit", Span::call_site());
    for attr in attrs.iter() {
        if let Some(path_seg) = attr.path.segments.first() {
            if path_seg.ident == openfmb_ident {
                if let Ok(syn::Meta::List(meta)) = attr.parse_meta() {
                    for m in meta.nested {
                        if let syn::NestedMeta::Meta(nm) = m {
                            if let Some(n_path_seg) = nm.path().segments.first() {
                                if n_path_seg.ident == inherit_ident {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

/// Generates a trait and an implementation of that trait for the type
///
/// This can be used in conjuction with the OpenFMBInherits to build a type tree
/// and shortcut accessor/mutator functions
///
/// Any field that is marked as #[openfmb_inherit] will be taken to mean
/// the type while holding a field of type T, implements the functionality of type T
/// inherently
#[proc_macro_derive(OpenFMB, attributes(openfmb))]
pub fn derive_openfmb(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let ident = &ast.ident;
    let snake_ident = Ident::new(&format!("{}", ident).to_snake_case(), Span::call_site());
    let trait_ident = format_ident!("Is{}", ident);
    let get_ident = format_ident!("_get_{}", snake_ident);
    //TODO
    //* map fields to getters/mutators, for Option<T>'s add clear_#field_ident
    //  funcs as well to the trait
    //* Generate implementation for base type
    let variant_data = match ast.data {
        syn::Data::Struct(variant_data) => variant_data,
        syn::Data::Enum(..) => return TokenStream::new(),
        syn::Data::Union(..) => return TokenStream::new(),
    };

    // map fields to accessors named get_#fieldname(&self)
    // where the return is the field type if is_copy returns true
    // or a reference to the field type if is_copy is not true
    // Option<T> is treated special in that we return
    // either the default *or* the current value with the same rules as above
    // This is done by defining const default values the trait may use if the
    // option is None letting rust do the work of keeping things immutable
    let trait_def_accessor = variant_data.fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let field_accessor_ident = format_ident!("get_{}", field_ident);
        if let Some(ty) = get_option_ty(ty) {
            if is_copy(&ty) {
                quote!{
                    fn #field_accessor_ident(&self) -> #ty {
                         self.#get_ident().#field_ident.unwrap_or_default()
                    }
                }
            } else {
                let name = format!("{}_{}", ident, field_ident);
                let const_name = &name.to_shouty_snake_case();
                let const_ident = Ident::new(const_name, Span::call_site());
                quote!{
                    fn #field_accessor_ident(&self) -> &#ty {
                         self.#get_ident().#field_ident.as_ref().unwrap_or(&#snake_ident::#const_ident)
                    }
                }
            }
        } else {
            let field_accessor_ident = format_ident!("get_{}", field_ident);
            quote!{
                fn #field_accessor_ident(&self) -> #ty {
                    self.#get_ident().#field_ident
                }
            }
        }
    });

    let trait_const_defaults = variant_data.fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        if let Some(ty) = get_option_ty(ty) {
             if !is_copy(&ty) {
                 let name = format!("{}_{}", ident, field_ident);
                 let const_name = &name.to_shouty_snake_case();
                 let const_ident = Ident::new(const_name, Span::call_site());
                 quote! {
                         pub(super) static ref #const_ident: #ty = Default::default();
                 }
             } else {
                 quote!{}
             }
        } else {
            quote!{}
        }
    });
    let supertraits: Vec<Ident> = variant_data.fields.iter().filter_map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ty = if let Some(field_ty) = get_option_ty(&field.ty) {
            field_ty
        } else {
            field.ty.clone()
        };
        let is_inherited = is_inherit(field);
        if let syn::Type::Path(path_ty) = field_ty {
            let ty_ident = path_ty.path.segments.first().unwrap().ident.clone();
            if is_inherited {
                Some(Ident::new(&format!("Is{}", ty_ident), Span::call_site()))
            } else {
                None
            }
        } else {
            None
        }
    }).collect();

    let trait_inherit_impls = variant_data.fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ty = if let Some(field_ty) = get_option_ty(&field.ty) {
            field_ty
        } else {
            field.ty.clone()
        };
        let is_inherited = is_inherit(field);
        if let syn::Type::Path(path_ty) = field_ty {
            let ty_ident = path_ty.path.segments.first().unwrap().ident.clone();
            if is_inherited {
                let name = format!("{}_{}", ident, field_ident);
                let const_name = &name.to_shouty_snake_case();
                let const_ident = Ident::new(const_name, Span::call_site());
                let inherit_trait_ident = Ident::new(&format!("Is{}", ty_ident), Span::call_site());
                let snake_ty = Ident::new(&format!("{}", ty_ident).to_snake_case(), Span::call_site());
                let inherit_get_ident = format_ident!("_get_{}", snake_ty);
                //quote!{}
                quote!{
                    impl #inherit_trait_ident for #ident {
                        fn #inherit_get_ident(&self) -> &#ty_ident {
                            self.#field_ident.as_ref().unwrap_or(&#snake_ident::#const_ident)
                        }
                    }
                    impl #inherit_trait_ident for dyn #trait_ident + 'static {
                        fn #inherit_get_ident(&self) -> &#ty_ident {
                            self.#get_ident().#field_ident.as_ref().unwrap_or(&#snake_ident::#const_ident)
                        }
                    }
                }
            } else {
                quote!{}
            }
        } else {
            quote!{}
        }
    });

    // TODO investigate using static here instead to avoid potential allocations per usage
    let const_defaults = quote!{
        mod #snake_ident {
            use super::*;
            use lazy_static::lazy_static;
            lazy_static! {
                #(#trait_const_defaults)*
            }
        }
    };

    let trait_def = //if supertraits.len() == 0 {
        quote! {
            trait #trait_ident  {
                fn #get_ident(&self) -> &#ident;
                #(#trait_def_accessor)*
            }
        };
    //} else {
    //     quote! {
    //        trait #trait_ident: #(#supertraits)+*  {
    //            fn #get_ident(&self) -> &#ident;
    //            #(#trait_def_accessor)*
    //        }
    //    }
    //};

    let trait_impl = quote! {
        impl #trait_ident for #ident {
            fn #get_ident(&self) -> &#ident {
                self
            }
        }
    };

    let expanded = quote! {
        #const_defaults
        #trait_def
        #trait_impl
        #(#trait_inherit_impls)*
    };

    expanded.into()
}


#[test]
fn derive_openfmb_test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-simple.rs");
    t.pass("tests/02-prost-opt.rs");
}
