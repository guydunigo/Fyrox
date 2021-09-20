//! Derive input types by `darling`.
//!
//! They parse `#[attributes(..)]` in declartive style, different from `syn`.

use darling::*;
use syn::*;

#[derive(FromDeriveInput)]
#[darling(attributes(visit), supports(struct_any, enum_any))]
pub struct TypeArgs {
    pub ident: Ident,
    pub generics: Generics,
    pub data: ast::Data<VariantArgs, FieldArgs>,
}

/// Parsed from struct's or enum variant's field
#[derive(FromField, Clone)]
#[darling(attributes(visit))]
pub struct FieldArgs {
    pub ident: Option<Ident>,
    pub ty: Type,
}

#[derive(FromVariant)]
#[darling(attributes(inspect))]
pub struct VariantArgs {
    pub ident: Ident,
    pub fields: ast::Fields<FieldArgs>,
}
