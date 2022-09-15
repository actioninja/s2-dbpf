use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro_error::{abort, proc_macro_error, OptionExt};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    parse_macro_input, Attribute, Data, DataEnum, DeriveInput, Expr, Lit, Meta, MetaNameValue,
    NestedMeta, Variant,
};

fn find_dbpf_attr(v: &Variant) -> Option<&Attribute> {
    v.attrs.iter().find(|attr| {
        if let Some(first) = attr.path.segments.first() {
            first.ident == "dbpf"
        } else {
            false
        }
    })
}

fn get_meta_attr(attr: &Attribute, meta_name: &'static str) -> Option<MetaNameValue> {
    let parsed = attr.parse_meta().unwrap_or_else(|err| {
        abort!(
            attr.span(),
            "Failed to meta parse attribute \"{}\": {}",
            attr.tokens.to_string(),
            err
        )
    });
    let attr_span = &attr.span();
    if let Meta::List(list) = parsed {
        list.nested
            .iter()
            .map(|nested| {
                if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                    nv
                } else {
                    unimplemented!()
                }
            })
            .find(|mnv| mnv.path.segments[0].ident == meta_name)
            .cloned()
    } else {
        abort!(attr_span, "Severely malformed attribute in some fashion");
    }
}

fn extract_variant_discriminant(variant: &Variant) -> &Lit {
    if let Some(disc) = &variant.discriminant {
        if let Expr::Lit(lit) = &disc.1 {
            &lit.lit
        } else {
            abort!(disc.1.span(), "Discriminants must be literals");
        }
    } else {
        abort!(variant.span(), "All variants must have a discriminant");
    }
}

#[proc_macro_error]
#[proc_macro_derive(DbpfKindsDerive, attributes(dbpf))]
pub fn dbpf_kinds_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let ident = ast.ident;

    let variants = if let Data::Enum(DataEnum { variants, .. }) = ast.data {
        variants
    } else {
        unimplemented!()
    };

    let id_from_matches = variants.iter().map(|v| {
        let name = &v.ident;
        let id = extract_variant_discriminant(v);

        quote! { #id => ::std::option::Option::Some(Self::#name) }
    });

    let name_to_matches = variants.iter().map(|v| {
        let name = &v.ident;
        let attribute = find_dbpf_attr(v).unwrap_or_else(|| {
            abort!(
                v.span(),
                "Did not find dbpf attribute for field, all variants require dbpf attribute"
            )
        });
        let meta = get_meta_attr(attribute, "short_name")
            .expect_or_abort("Failed to find short_name, required attr");
        let id = &meta.lit;

        quote! { Self::#name => #id }
    });

    let kind_name = Ident::new(&format!("{}Kind", ident), ident.span());

    let variants_wrapped = variants.iter().filter_map(|v| {
        let name = &v.ident;
        let attribute = find_dbpf_attr(v).unwrap();
        let meta = get_meta_attr(attribute, "kind_type");
        meta.map(|mnv| {
            let found_type = &mnv.lit;
            let ident = if let Lit::Str(str) = found_type {
                Ident::new(&str.value(), v.span())
            } else {
                abort!(attribute.span(), "kind_type expects string literal")
            };
            quote! { #name(#ident) }
        })
    });

    let from_into_variants = variants.iter().filter_map(|v| {
        let name = &v.ident;
        let attribute = find_dbpf_attr(v).unwrap();
        let meta = get_meta_attr(attribute, "kind_type");
        meta.map(|mnv| {
            let found_type = &mnv.lit;
            let ident = if let Lit::Str(str) = found_type {
                Ident::new(&str.value(), v.span())
            } else {
                abort!(attribute.span(), "kind_type expects string literal")
            };
            quote! {
                impl ::std::convert::From<#ident> for #kind_name {
                    fn from(item: #ident) -> Self {
                        Self::#name(item)
                    }
                }

                impl ::std::convert::TryFrom<#kind_name> for #ident {
                    type Error = &'static str;

                    fn try_from(item: #kind_name) -> ::std::result::Result<Self, Self::Error> {
                        match item {
                            #kind_name::#name(inner) => ::std::result::Result::Ok(inner),
                            _ => ::std::result::Result::Err("Incorrect variant found in the enum"),
                        }
                    }
                }
            }
        })
    });

    let parse_matches = variants.iter().filter_map(|v| {
        let name = &v.ident;
        let attribute = find_dbpf_attr(v).unwrap();
        let meta = get_meta_attr(attribute, "kind_type");
        meta.map(|mnv| {
            let found_type = &mnv.lit;
            let inner_ident = if let Lit::Str(str) = found_type {
                Ident::new(&str.value(), v.span())
            } else {
                abort!(attribute.span(), "kind_type expects string literal")
            };
            quote! {
                #ident::#name => Ok(#inner_ident::read_options(reader, options, parser_args)?.into())
            }
        })
    });

    let kind_to_id = variants.iter().filter_map(|v| {
        let name = &v.ident;
        let attribute = find_dbpf_attr(v).unwrap();
        let meta = get_meta_attr(attribute, "kind_type");
        meta.map(|_mnv| {
            quote! {
                Self::#name(_) => #ident::#name
            }
        })
    });

    let tokens = quote! {
        impl #ident {
            pub const fn short_name(&self) -> &'static str {
                match self {
                    #(#name_to_matches,)*
                }
            }

            pub const fn from_id(id: u32) -> ::std::option::Option<Self> {
                match id {
                    #(#id_from_matches,)*
                    _ => ::std::option::Option::None,
                }
            }
        }

        pub enum #kind_name {
            #(#variants_wrapped,)*
        }

        #(#from_into_variants)*

        impl #kind_name {
            pub fn id(&self) -> #ident {
                match self {
                    #(#kind_to_id,)*
                }
            }

            pub fn parse<R: ::std::io::Read + ::std::io::Seek>(
                reader: &mut R,
                id: #ident,
                options: &::binrw::ReadOptions,
                parser_args: ParserArgs,
            ) -> ::binrw::BinResult<Self>{
                match id {
                    #(#parse_matches,)*
                    _ => Ok(Unimplemented::read_options(reader, options, parser_args)?.into())
                }
            }
        }
    };

    tokens.into()
}
