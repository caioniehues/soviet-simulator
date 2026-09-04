use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsUnnamed};

mod args;
use args::{InspectArgs, InspectFieldArgs, InspectFieldArgsDefault, InspectStructArgs};

pub fn impl_inspect_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_args = match InspectStructArgs::from_derive_input(&input) {
        Ok(args) => args,
        // darling errors are already spanned; surface them instead of panicking.
        Err(e) => return proc_macro::TokenStream::from(e.write_errors()),
    };
    let field_args = match parse_field_args(&input, &struct_args) {
        Ok(fields) => fields,
        Err(e) => return proc_macro::TokenStream::from(e.to_compile_error()),
    };
    generate(&input, struct_args, field_args)
}

#[derive(Debug)]
struct ParsedField {
    render: TokenStream,
    render_mut: TokenStream,
    //skip: bool
}

/// Every trait needs to be checked here
fn handle_inspect_types(
    parsed_field: &mut Option<ParsedField>,
    f: &syn::Field,
) -> syn::Result<()> {
    // These are effectively constants
    #[allow(non_snake_case)]
    // `quote!(inspect)` is a single identifier, so parsing it as a path cannot fail.
    let INSPECT_DEFAULT_PATH =
        syn::parse2::<syn::Path>(quote!(inspect)).expect("`inspect` parses as a path");

    try_handle_inspect_type::<InspectFieldArgsDefault, InspectArgs>(
        parsed_field,
        f,
        &INSPECT_DEFAULT_PATH,
        quote!(egui_inspect::Inspect),
        quote!(egui_inspect::InspectArgs),
    )
}

/// Display name of a field for error messages. Named fields use their ident;
/// tuple-struct fields have none, so they fall back to a placeholder.
fn field_name(f: &syn::Field) -> String {
    match &f.ident {
        Some(ident) => ident.to_string(),
        None => "<unnamed>".to_string(),
    }
}

fn parse_field_args(
    input: &DeriveInput,
    struct_args: &InspectStructArgs,
) -> syn::Result<Vec<ParsedField>> {
    match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    // Parse the fields
                    let mut parsed_fields = Vec::with_capacity(fields.named.len());
                    for f in &fields.named {
                        let mut parsed_field: Option<ParsedField> = None;

                        handle_inspect_types(&mut parsed_field, f)?;

                        if parsed_field.is_none() {
                            handle_inspect_type::<InspectFieldArgsDefault, InspectArgs>(
                                &mut parsed_field,
                                f,
                                quote!(egui_inspect::Inspect),
                                quote!(egui_inspect::InspectArgs),
                            )?;
                        }

                        let parsed_field = parsed_field.ok_or_else(|| {
                            syn::Error::new_spanned(
                                f,
                                format!(
                                    "egui-inspect: could not determine inspect handling for field `{}`",
                                    field_name(f)
                                ),
                            )
                        })?;
                        parsed_fields.push(parsed_field);
                    }

                    Ok(parsed_fields)
                }
                Fields::Unnamed(field) => {
                    if field.unnamed.len() != 1 {
                        return Err(syn::Error::new_spanned(
                            field,
                            format!(
                                "egui-inspect: tuple structs with {} fields are not supported; only single-field tuple structs are supported",
                                field.unnamed.len()
                            ),
                        ));
                    }
                    Ok(vec![ParsedField {
                        render: create_render_call_unit_struct(field)?,
                        render_mut: create_render_mut_call_unit_struct(field)?,
                    }])
                }
                Fields::Unit => Ok(vec![]),
            }
        }
        Data::Enum(data) => Ok(vec![ParsedField {
            render: create_render_call_enum(data, struct_args)?,
            render_mut: create_render_mut_call_enum(data, struct_args)?,
        }]),
        _ => Err(syn::Error::new_spanned(
            &input.ident,
            "egui-inspect: only structs and fieldless enums are supported",
        )),
    }
}

fn try_handle_inspect_type<
    FieldArgsT: darling::FromField + InspectFieldArgs + Clone,
    ArgsT: From<FieldArgsT> + ToTokens,
>(
    parsed_field: &mut Option<ParsedField>,
    f: &syn::Field,
    path: &syn::Path,
    default_render_trait: TokenStream,
    arg_type: TokenStream,
) -> syn::Result<()> {
    let mut matches = f.attrs.iter().filter(|x| x.path == *path);
    if matches.next().is_none() {
        return Ok(());
    }
    if matches.next().is_some() {
        return Err(syn::Error::new_spanned(
            f,
            format!(
                "egui-inspect: too many `inspect` attributes on field `{}`",
                field_name(f)
            ),
        ));
    }
    handle_inspect_type::<FieldArgsT, ArgsT>(parsed_field, f, default_render_trait, arg_type)
}

// Does common data gathering and error checking, then calls create_render_call and create_render_mut_call to emit
// code for inspecting.
fn handle_inspect_type<
    FieldArgsT: darling::FromField + InspectFieldArgs + Clone,
    ArgsT: From<FieldArgsT> + ToTokens,
>(
    parsed_field: &mut Option<ParsedField>,
    f: &syn::Field,
    default_render_trait: TokenStream,
    arg_type: TokenStream,
) -> syn::Result<()> {
    if parsed_field.is_some() {
        return Err(syn::Error::new_spanned(
            f,
            format!(
                "egui-inspect: too many `inspect` attributes on field `{}`",
                field_name(f)
            ),
        ));
    }

    let field_args = FieldArgsT::from_field(f).map_err(|e| {
        syn::Error::new_spanned(
            f,
            format!(
                "egui-inspect: invalid `inspect` attribute on field `{}`: {e}",
                field_name(f)
            ),
        )
    })?;

    if field_args.skip() {
        *parsed_field = Some(ParsedField {
            render: quote!(),
            render_mut: quote!(),
            //skip: true
        });

        return Ok(());
    }

    let render_trait = match field_args.render_trait() {
        Some(t) => t.clone(),
        // Built with `quote!` from a valid path by every caller, so this only
        // fails on malformed caller tokens; report it spanned on the field.
        None => syn::parse2::<syn::Path>(default_render_trait).map_err(|e| {
            syn::Error::new_spanned(
                f,
                format!(
                    "egui-inspect: invalid default render trait for field `{}`: {e}",
                    field_name(f)
                ),
            )
        })?,
    };

    // Built with `quote!` by every caller; same spanned-error policy as above.
    let arg_type = syn::parse2::<syn::Type>(arg_type).map_err(|e| {
        syn::Error::new_spanned(
            f,
            format!(
                "egui-inspect: invalid inspect arg type for field `{}`: {e}",
                field_name(f)
            ),
        )
    })?;
    let args: ArgsT = field_args.clone().into();

    // Only named-field paths reach this handler, so ident is expected; a
    // tuple-struct field landing here is an unsupported shape, not a panic.
    let field_ident = field_args.ident().clone().ok_or_else(|| {
        syn::Error::new_spanned(
            f,
            "egui-inspect: `inspect` attributes are only supported on named fields",
        )
    })?;

    let render = create_render_call(
        &field_ident,
        field_args.name(),
        field_args.ty(),
        &render_trait,
        field_args.proxy_type(),
        &arg_type,
        &args,
    );

    let render_mut = create_render_mut_call(
        &field_ident,
        field_args.name(),
        field_args.ty(),
        &render_trait,
        field_args.proxy_type(),
        &arg_type,
        &args,
    );

    *parsed_field = Some(ParsedField {
        render,
        render_mut,
        //skip: false
    });
    Ok(())
}

fn create_render_call_unit_struct(data: &FieldsUnnamed) -> syn::Result<TokenStream> {
    // The caller checks for exactly one field first; this guards the helper itself.
    let ty = data
        .unnamed
        .iter()
        .next()
        .map(|field| &field.ty)
        .ok_or_else(|| {
            syn::Error::new_spanned(
                data,
                "egui-inspect: tuple structs must have exactly one field",
            )
        })?;

    Ok(quote! {{
        <#ty as egui_inspect::Inspect<#ty>>::render(&data.0, "", ui, args)
    }})
}

fn create_render_mut_call_unit_struct(data: &FieldsUnnamed) -> syn::Result<TokenStream> {
    // Same single-field guard as the immutable helper.
    let ty = data
        .unnamed
        .iter()
        .next()
        .map(|field| &field.ty)
        .ok_or_else(|| {
            syn::Error::new_spanned(
                data,
                "egui-inspect: tuple structs must have exactly one field",
            )
        })?;

    Ok(quote! {{
        <#ty as egui_inspect::Inspect<#ty>>::render_mut(&mut data.0, "", ui, args)
    }})
}

fn create_render_call_enum(
    data: &syn::DataEnum,
    args: &InspectStructArgs,
) -> syn::Result<TokenStream> {
    let mut variants = Vec::with_capacity(data.variants.len());
    for v in &data.variants {
        if !v.fields.is_empty() {
            return Err(syn::Error::new_spanned(
                &v.ident,
                format!(
                    "egui-inspect: only fieldless enums are supported (variant `{}` has fields)",
                    v.ident
                ),
            ));
        }
        variants.push(&v.ident);
    }

    let sname = &args.ident;

    Ok(quote! {{
        match data {
            #(#sname::#variants => {
                ui.label(stringify!(#variants));
            })*
        }
    }})
}

fn create_render_mut_call_enum(
    data: &syn::DataEnum,
    args: &InspectStructArgs,
) -> syn::Result<TokenStream> {
    let mut variants = Vec::with_capacity(data.variants.len());
    for v in &data.variants {
        if !v.fields.is_empty() {
            return Err(syn::Error::new_spanned(
                &v.ident,
                format!(
                    "egui-inspect: only fieldless enums are supported (variant `{}` has fields)",
                    v.ident
                ),
            ));
        }
        variants.push(&v.ident);
    }

    let sname = &args.ident;

    Ok(quote! {{
        match data {
            #(#sname::#variants => {
                ui.label(stringify!(#variants));
            })*
        }
    }})
}

fn create_render_call<T: ToTokens>(
    field_name: &syn::Ident,
    field_rename: &Option<syn::Ident>,
    field_type: &syn::Type,
    render_trait: &syn::Path,
    proxy_type: &Option<syn::Path>,
    arg_type: &syn::Type,
    args: &T,
) -> TokenStream {
    use quote::format_ident;
    let args_name1 = format_ident!("_inspect_args_{}", field_name);
    let args_name2 = args_name1.clone();

    let field_name1 = field_name.clone();
    let field_name2 = field_rename.clone().unwrap_or_else(|| field_name.clone());

    let source_type = if let Some(w) = proxy_type {
        quote!(#w)
    } else {
        quote!(#field_type)
    };

    quote! {{
        #[allow(non_upper_case_globals)]
        const #args_name1 : #arg_type = #args;
        let value = &data.#field_name1;
        <#source_type as #render_trait<#field_type>>::render(value, stringify!(#field_name2), ui, &#args_name2);
    }}
}

fn create_render_mut_call<T: ToTokens>(
    field_name: &syn::Ident,
    field_rename: &Option<syn::Ident>,
    field_type: &syn::Type,
    render_trait: &syn::Path,
    proxy_type: &Option<syn::Path>,
    arg_type: &syn::Type,
    args: &T,
) -> TokenStream {
    use quote::format_ident;
    let args_name1 = format_ident!("_inspect_args_{}", field_name);
    let args_name2 = args_name1.clone();

    let field_name1 = field_name.clone();
    let field_name2 = field_rename.clone().unwrap_or_else(|| field_name.clone());

    let source_type = if let Some(w) = proxy_type {
        quote!(#w)
    } else {
        quote!(#field_type)
    };

    quote! {{
        #[allow(non_upper_case_globals)]
        const #args_name1 : #arg_type = #args;
        let mut value = &mut data.#field_name1;
        let mut changed = <#source_type as #render_trait<#field_type>>::render_mut(value, stringify!(#field_name2), ui, &#args_name2);

        _has_any_field_changed |= changed;
    }}
}

// Provide a way to early out and generate no code. It's going to be a common case for
// downstream users to want to only conditionally generate code, and it's easier to do this
// by adding an early-out here that can be configured via a cargo feature, than having to
// mark up all the downstream code with conditional compile directives.
#[cfg(not(feature = "generate_code"))]
fn generate(
    input: &syn::DeriveInput,
    struct_args: InspectStructArgs,
    parsed_fields: Vec<ParsedField>,
) -> proc_macro::TokenStream {
    return proc_macro::TokenStream::from(quote! {});
}

#[cfg(feature = "generate_code")]
fn generate(
    input: &DeriveInput,
    struct_args: InspectStructArgs,
    parsed_fields: Vec<ParsedField>,
) -> proc_macro::TokenStream {
    let struct_name1 = &struct_args.ident;
    let struct_name2 = &struct_args.ident;
    let struct_name3 = &struct_args.ident;
    let struct_name4 = &struct_args.ident;

    let mut render_impls = vec![];
    let mut render_mut_impls = vec![];

    for parsed_field in parsed_fields {
        render_impls.push(parsed_field.render);
        render_mut_impls.push(parsed_field.render_mut);
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let struct_impl = quote! {
        impl #impl_generics #struct_name2 #ty_generics #where_clause {
            fn impls(data: &Self, ui: &mut egui_inspect::egui::Ui, args: &egui_inspect::InspectArgs, header: bool, indent_children: bool) {
                #(#render_impls)*
            }

            fn impls_mut(data: &mut Self, ui: &mut egui_inspect::egui::Ui, args: &egui_inspect::InspectArgs, header: bool, indent_children: bool) -> bool {
                let mut _has_any_field_changed = false;
                #(#render_mut_impls)*
                ;_has_any_field_changed
            }
        }

        impl #impl_generics egui_inspect::Inspect<#struct_name1> for #struct_name2 #ty_generics #where_clause {
            fn render(data: &Self, label: &'static str, ui: &mut egui_inspect::egui::Ui, args: &egui_inspect::InspectArgs) {
                let header_name = stringify!(#struct_name3);

                let mut header = true;
                if let Some(h) = args.header {
                    header = h;
                }

                let mut indent_children = true;
                if let Some(ic) = args.indent_children {
                    header = ic;
                }

                if header {
                    egui_inspect::egui::CollapsingHeader::new(label).default_open(true).show(ui, |ui| {
                        Self::impls(data, ui, args, header, indent_children);
                    });
                } else {
                    Self::impls(data, ui, args, header, indent_children);
                };
            }

            fn render_mut(data: &mut Self, label: &'static str, ui: &mut egui_inspect::egui::Ui, args: &egui_inspect::InspectArgs) -> bool {
                let header_name = stringify!(#struct_name4);

                let mut header = true;
                if let Some(h) = args.header {
                    header = h;
                }

                let mut indent_children = true;
                if let Some(ic) = args.indent_children {
                    indent_children = ic;
                }


                let mut _has_any_field_changed = false;
                if header {
                    egui_inspect::egui::CollapsingHeader::new(label).default_open(true).show(ui, |ui| {
                        _has_any_field_changed = Self::impls_mut(data, ui, args, header, indent_children);
                    });
                } else {
                    _has_any_field_changed = Self::impls_mut(data, ui, args, header, indent_children);
                };

                _has_any_field_changed
            }
        }
    };

    proc_macro::TokenStream::from(quote! {
        #struct_impl
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn struct_args_of(input: &DeriveInput) -> InspectStructArgs {
        InspectStructArgs::from_derive_input(input).expect("test struct args parse")
    }

    #[test]
    fn tuple_struct_with_two_fields_is_a_named_error() {
        let input: DeriveInput = syn::parse_str("struct Bad(u8, u8);").expect("test input parses");
        let err = parse_field_args(&input, &struct_args_of(&input))
            .expect_err("two-field tuple struct is rejected");
        let msg = err.to_string();
        assert!(
            msg.contains("tuple structs with 2 fields are not supported"),
            "unexpected error: {msg}"
        );
    }

    #[test]
    fn enum_variant_with_fields_names_the_variant() {
        let input: DeriveInput =
            syn::parse_str("enum Bad { Unit, WithFields(u8) }").expect("test input parses");
        let err = parse_field_args(&input, &struct_args_of(&input))
            .expect_err("data-carrying variant is rejected");
        let msg = err.to_string();
        assert!(
            msg.contains("WithFields"),
            "error names the offending variant, got: {msg}"
        );
    }

    #[test]
    fn duplicate_inspect_attribute_names_the_field() {
        let input: DeriveInput = syn::parse_str(
            "struct Bad { #[inspect(skip)] #[inspect(skip)] health: u8 }",
        )
        .expect("test input parses");
        let err = parse_field_args(&input, &struct_args_of(&input))
            .expect_err("duplicate attribute is rejected");
        let msg = err.to_string();
        assert!(
            msg.contains("health"),
            "error names the offending field, got: {msg}"
        );
    }
}
