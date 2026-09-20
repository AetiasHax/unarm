use anyhow::{Result, anyhow, bail};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use serde::Deserialize;
use syn::Ident;

use crate::{
    isa::{
        DataType, DataTypeEnumVariant, DataTypeKind, DataTypeName, DataTypeStruct, DataTypeUnion,
        Isa,
    },
    util::str::snake_to_pascal_case,
};

#[derive(Deserialize, Debug)]
pub struct Getters(Vec<Getter>);

impl Getters {
    pub fn validate(&self, isa: &Isa) -> Result<()> {
        for getter in self.0.iter() {
            getter.validate(isa)?;
        }
        Ok(())
    }

    pub fn impl_tokens(&self, isa: &Isa) -> TokenStream {
        let impls = self.0.iter().map(|g| g.impl_tokens(isa));
        quote!(#(#impls)*)
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Getter {
    name: GetterName,
    description: String,
    kind: GetterKind,
}

impl Getter {
    pub fn validate(&self, isa: &Isa) -> Result<()> {
        match &self.kind {
            GetterKind::InsParamOfType(type_name) => {
                let data_type = isa
                    .types()
                    .get(type_name)
                    .ok_or_else(|| anyhow!("Type name '{type_name}' not found"))?;
                data_type
                    .default_expr_tokens(isa)
                    .ok_or_else(|| anyhow!("Data type '{type_name}' has no default value"))?;
            }
            GetterKind::FieldWithName(field_name) => {
                let fields = collect_fields_with_name(isa, field_name);
                let Some(first) = fields.first() else {
                    bail!("No field named '{field_name}' found in any data type or opcode");
                };
                let expected = first.type_tokens(isa).to_string();
                for field in fields.iter().skip(1) {
                    let found = field.type_tokens(isa).to_string();
                    if found != expected {
                        bail!(
                            "Fields named '{field_name}' must all have the same type, \
                             found both '{expected}' and '{found}'"
                        );
                    }
                }
            }
        }
        Ok(())
    }

    pub fn impl_tokens(&self, isa: &Isa) -> TokenStream {
        match &self.kind {
            GetterKind::InsParamOfType(param_type_name) => {
                self.ins_param_of_type_impl_tokens(isa, param_type_name)
            }
            GetterKind::FieldWithName(field_name) => {
                self.field_with_name_impl_tokens(isa, field_name)
            }
        }
    }

    fn ins_param_of_type_impl_tokens(
        &self,
        isa: &Isa,
        param_type_name: &DataTypeName,
    ) -> TokenStream {
        let getter_ident = self.name.as_ident();
        let data_type = isa.types.get(param_type_name).unwrap();
        let type_tokens = data_type.type_tokens(isa);

        let cases = isa.opcodes().iter().filter_map(|o| {
            let ident = Ident::new(&snake_to_pascal_case(o.mnemonic()), Span::call_site());
            let cfg = o.cfg_attribute_tokens(isa);
            o.params().iter().find_map(|(name, type_name)| {
                if type_name == param_type_name {
                    let param_ident = Ident::new(&name.0, Span::call_site());
                    Some(quote! {
                        #cfg
                        Ins::#ident { #param_ident, .. } => *#param_ident
                    })
                } else {
                    None
                }
            })
        });

        let default_expr_tokens = data_type.default_expr_tokens(isa).unwrap();
        let description = &self.description;

        quote! {
            impl Ins {
                #[doc = #description]
                pub fn #getter_ident(&self) -> #type_tokens {
                    match self {
                        #(#cases),*,
                        _ => #default_expr_tokens,
                    }
                }
            }
        }
    }

    /// Generates a getter for the field with the given name on every data type which directly or
    /// indirectly contains one, as well as on [`Ins`] itself.
    fn field_with_name_impl_tokens(&self, isa: &Isa, field_name: &str) -> TokenStream {
        let getter_ident = self.name.as_ident();
        let description = &self.description;

        let type_impls = isa.types().iter().filter_map(|data_type| {
            let found = data_type.find_field(isa, field_name)?;
            let body = data_type.field_getter_body_tokens(isa, field_name, &getter_ident)?;
            let return_type = found.presence.return_type_tokens(found.data_type.type_tokens(isa));
            let type_ident = data_type.name().as_pascal_ident();
            let cfg = data_type.cfg_attribute_tokens(isa);
            Some(quote! {
                #cfg
                impl #type_ident {
                    #[doc = #description]
                    pub fn #getter_ident(&self) -> #return_type {
                        #body
                    }
                }
            })
        });

        let ins_impl = self.field_with_name_ins_impl_tokens(isa, field_name, &getter_ident);

        quote! {
            #(#type_impls)*
            #ins_impl
        }
    }

    fn field_with_name_ins_impl_tokens(
        &self,
        isa: &Isa,
        field_name: &str,
        getter_ident: &Ident,
    ) -> TokenStream {
        let field_type = collect_fields_with_name(isa, field_name)
            .first()
            .expect("Getter was validated")
            .type_tokens(isa);

        let cases = isa.opcodes().iter().filter_map(|opcode| {
            let params = opcode.params().iter().map(|(param_name, type_name)| {
                (param_name.0.as_str(), isa.types().get(type_name).unwrap())
            });
            let paths = FieldPath::find_all(params, isa, field_name);
            let value = FieldPath::value_tokens(
                &paths,
                getter_ident,
                MemberAccess::Binding,
                FieldPresence::Sometimes,
            )?;

            let variant_ident =
                Ident::new(&snake_to_pascal_case(opcode.mnemonic()), Span::call_site());
            let bindings = paths.iter().map(|p| p.member());
            let cfg = opcode.cfg_attribute_tokens(isa);
            Some(quote! {
                #cfg
                Ins::#variant_ident { #(#bindings),*, .. } => #value
            })
        });

        let description = &self.description;

        quote! {
            impl Ins {
                #[doc = #description]
                pub fn #getter_ident(&self) -> Option<#field_type> {
                    match self {
                        #(#cases),*,
                        _ => None,
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct GetterName(pub String);

impl GetterName {
    pub fn as_ident(&self) -> Ident {
        Ident::new(&self.0, Span::call_site())
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub enum GetterKind {
    #[serde(rename = "ins_param_of_type")]
    InsParamOfType(DataTypeName),
    #[serde(rename = "field_with_name")]
    FieldWithName(String),
}

/// Describes how many values of a type contain a field with some name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldPresence {
    /// Every value of the type contains the field.
    Always,
    /// Only some values of the type contain the field.
    Sometimes,
}

impl FieldPresence {
    /// The presence of a field which is reachable through any one of several members.
    fn any(self, other: Self) -> Self {
        if self == Self::Always || other == Self::Always {
            Self::Always
        } else {
            Self::Sometimes
        }
    }

    fn return_type_tokens(self, type_tokens: TokenStream) -> TokenStream {
        match self {
            Self::Always => type_tokens,
            Self::Sometimes => quote!(Option<#type_tokens>),
        }
    }

    /// Converts an expression yielding a field with `self` presence into one yielding a field with
    /// `presence` presence.
    fn coerce_tokens(self, presence: Self, expr: TokenStream) -> TokenStream {
        match (self, presence) {
            (Self::Always, Self::Sometimes) => quote!(Some(#expr)),
            _ => expr,
        }
    }
}

/// A field with the wanted name, found by traversing the hierarchy of a data type.
struct FoundField<'a> {
    presence: FieldPresence,
    /// The data type of the field itself, which determines the return type of its getter.
    data_type: &'a DataType,
}

/// How the members of a struct, union variant or opcode are accessed in a getter body.
#[derive(Clone, Copy)]
enum MemberAccess {
    /// Members are fields of `self`.
    Field,
    /// Members are bound by reference in a pattern, e.g. in a match arm.
    Binding,
}

impl MemberAccess {
    /// An expression yielding the value of the given member.
    fn value_tokens(self, member: &Ident) -> TokenStream {
        match self {
            Self::Field => quote!(self.#member),
            Self::Binding => quote!(*#member),
        }
    }

    /// An expression to call the given member's own getter on.
    fn receiver_tokens(self, member: &Ident) -> TokenStream {
        match self {
            Self::Field => quote!(self.#member),
            Self::Binding => quote!(#member),
        }
    }
}

/// A member (struct field, union variant payload or opcode parameter) through which a field with
/// the wanted name can be reached.
enum FieldPath {
    /// The member is the field itself.
    Direct(Ident),
    /// The field lives inside the member, and is read using the member's own getter.
    Nested(Ident, FieldPresence),
}

impl FieldPath {
    /// Collects the members of a struct, union variant or opcode through which a field with the
    /// given name can be reached, in order of priority. Members after the first one which always
    /// contains the field are left out, since they can never be reached.
    fn find_all<'a>(
        members: impl IntoIterator<Item = (&'a str, &'a DataType)>,
        isa: &'a Isa,
        field_name: &str,
    ) -> Vec<Self> {
        let mut paths = Vec::new();
        for (member_name, data_type) in members {
            let ident = Ident::new(member_name, Span::call_site());
            if member_name == field_name {
                paths.push(Self::Direct(ident));
                break;
            }
            let Some(found) = data_type.find_field(isa, field_name) else {
                continue;
            };
            paths.push(Self::Nested(ident, found.presence));
            if found.presence == FieldPresence::Always {
                break;
            }
        }
        paths
    }

    fn member(&self) -> &Ident {
        match self {
            Self::Direct(member) | Self::Nested(member, _) => member,
        }
    }

    fn presence(&self) -> FieldPresence {
        match self {
            Self::Direct(_) => FieldPresence::Always,
            Self::Nested(_, presence) => *presence,
        }
    }

    fn expr_tokens(&self, getter_ident: &Ident, access: MemberAccess) -> TokenStream {
        match self {
            Self::Direct(member) => access.value_tokens(member),
            Self::Nested(member, _) => {
                let receiver = access.receiver_tokens(member);
                quote!(#receiver.#getter_ident())
            }
        }
    }

    /// Builds an expression reading the field out of the given members, falling back to the next
    /// member whenever the previous one turns out not to contain it.
    fn value_tokens(
        paths: &[Self],
        getter_ident: &Ident,
        access: MemberAccess,
        presence: FieldPresence,
    ) -> Option<TokenStream> {
        let (first, rest) = paths.split_first()?;
        let mut expr = first.expr_tokens(getter_ident, access);
        let mut expr_presence = first.presence();
        for path in rest {
            let next = path.expr_tokens(getter_ident, access);
            expr = match path.presence() {
                FieldPresence::Always => quote!(#expr.unwrap_or_else(|| #next)),
                FieldPresence::Sometimes => quote!(#expr.or_else(|| #next)),
            };
            expr_presence = expr_presence.any(path.presence());
        }
        Some(expr_presence.coerce_tokens(presence, expr))
    }
}

/// Collects every field, union variant payload and opcode parameter with the given name.
fn collect_fields_with_name<'a>(isa: &'a Isa, field_name: &str) -> Vec<&'a DataType> {
    let mut fields = Vec::new();
    for opcode in isa.opcodes().iter() {
        for (param_name, type_name) in opcode.params() {
            if param_name.0 == field_name {
                fields.push(isa.types().get(type_name).unwrap());
            }
        }
    }
    for data_type in isa.types().iter() {
        data_type.collect_fields_with_name(isa, field_name, &mut fields);
    }
    fields
}

impl DataType {
    /// Finds a field with the given name in this type, searching nested types as well.
    fn find_field<'a>(&'a self, isa: &'a Isa, field_name: &str) -> Option<FoundField<'a>> {
        match self.kind() {
            DataTypeKind::Bool { .. } => None,
            DataTypeKind::UInt(_) => None,
            DataTypeKind::Int(_) => None,
            DataTypeKind::Enum(_) => None,
            DataTypeKind::Union(data_type_union) => data_type_union.find_field(isa, field_name),
            DataTypeKind::Struct(data_type_struct) => data_type_struct.find_field(isa, field_name),
            DataTypeKind::Type(_, _) => self.canonical(isa).find_field(isa, field_name),
            DataTypeKind::Custom(_) => None,
        }
    }

    fn collect_fields_with_name<'a>(
        &'a self,
        isa: &'a Isa,
        field_name: &str,
        fields: &mut Vec<&'a DataType>,
    ) {
        match self.kind() {
            DataTypeKind::Union(data_type_union) => {
                for variant in data_type_union.variants().values() {
                    let Some(data) = variant.data() else {
                        continue;
                    };
                    if variant.payload_is_field(field_name) {
                        fields.push(data);
                    }
                    data.collect_fields_with_name(isa, field_name, fields);
                }
            }
            DataTypeKind::Struct(data_type_struct) => {
                for field in data_type_struct.fields() {
                    if field.name().0 == field_name {
                        fields.push(field);
                    }
                    field.collect_fields_with_name(isa, field_name, fields);
                }
            }
            DataTypeKind::Type(_, _) => {
                self.canonical(isa).collect_fields_with_name(isa, field_name, fields)
            }
            _ => {}
        }
    }

    /// Generates the body of the getter for the field with the given name, or [`None`] if this
    /// type has no generated Rust type to implement the getter on.
    fn field_getter_body_tokens(
        &self,
        isa: &Isa,
        field_name: &str,
        getter_ident: &Ident,
    ) -> Option<TokenStream> {
        match self.kind() {
            DataTypeKind::Union(data_type_union) => {
                data_type_union.field_getter_body_tokens(isa, field_name, getter_ident)
            }
            DataTypeKind::Struct(data_type_struct) => {
                data_type_struct.field_getter_body_tokens(isa, field_name, getter_ident)
            }
            _ => None,
        }
    }
}

impl DataTypeStruct {
    fn field_paths(&self, isa: &Isa, field_name: &str) -> Vec<FieldPath> {
        let fields = self.fields().iter().map(|field| (field.name().0.as_str(), field));
        FieldPath::find_all(fields, isa, field_name)
    }

    fn find_field<'a>(&'a self, isa: &'a Isa, field_name: &str) -> Option<FoundField<'a>> {
        if let Some(field) = self.fields().iter().find(|f| f.name().0 == field_name) {
            return Some(FoundField { presence: FieldPresence::Always, data_type: field });
        }
        let mut found: Option<FoundField> = None;
        for field in self.fields() {
            let Some(nested) = field.find_field(isa, field_name) else {
                continue;
            };
            found = Some(match found {
                // Keep the data type of the first match, all of them have the same type anyway
                Some(prev) => FoundField {
                    presence: prev.presence.any(nested.presence),
                    data_type: prev.data_type,
                },
                None => nested,
            });
        }
        found
    }

    fn field_getter_body_tokens(
        &self,
        isa: &Isa,
        field_name: &str,
        getter_ident: &Ident,
    ) -> Option<TokenStream> {
        let paths = self.field_paths(isa, field_name);
        let presence = self.find_field(isa, field_name)?.presence;
        FieldPath::value_tokens(&paths, getter_ident, MemberAccess::Field, presence)
    }
}

impl DataTypeUnion {
    fn find_field<'a>(&'a self, isa: &'a Isa, field_name: &str) -> Option<FoundField<'a>> {
        let mut found: Option<FoundField> = None;
        let mut in_all_variants = true;
        for variant in self.variants().values() {
            match variant.find_field(isa, field_name) {
                Some(variant_found) => {
                    in_all_variants &= variant_found.presence == FieldPresence::Always;
                    // Keep the data type of the first match, all of them have the same type anyway
                    found.get_or_insert(variant_found);
                }
                None => in_all_variants = false,
            }
        }
        found.map(|found| FoundField {
            presence: if in_all_variants {
                FieldPresence::Always
            } else {
                FieldPresence::Sometimes
            },
            data_type: found.data_type,
        })
    }

    fn field_getter_body_tokens(
        &self,
        isa: &Isa,
        field_name: &str,
        getter_ident: &Ident,
    ) -> Option<TokenStream> {
        let presence = self.find_field(isa, field_name)?.presence;
        let cases: Vec<_> = self
            .variants()
            .values()
            .filter_map(|variant| {
                variant.field_getter_case_tokens(isa, field_name, getter_ident, presence)
            })
            .collect();
        // Only add a fallback case if some variants aren't covered, otherwise it's unreachable
        let fallback = (cases.len() < self.variants().len()).then(|| quote!(_ => None,));
        Some(quote! {
            match self {
                #(#cases)*
                #fallback
            }
        })
    }
}

impl DataTypeEnumVariant {
    /// Returns whether this variant's payload is itself the field with the given name. Payloads
    /// which are structs are inlined into the variant, so their name isn't part of the enum.
    fn payload_is_field(&self, field_name: &str) -> bool {
        match self.data() {
            Some(data) => {
                !matches!(data.kind(), DataTypeKind::Struct(_)) && data.name().0 == field_name
            }
            None => false,
        }
    }

    fn find_field<'a>(&'a self, isa: &'a Isa, field_name: &str) -> Option<FoundField<'a>> {
        let data = self.data()?;
        if self.payload_is_field(field_name) {
            return Some(FoundField { presence: FieldPresence::Always, data_type: data });
        }
        data.find_field(isa, field_name)
    }

    fn field_getter_case_tokens(
        &self,
        isa: &Isa,
        field_name: &str,
        getter_ident: &Ident,
        presence: FieldPresence,
    ) -> Option<TokenStream> {
        let data = self.data()?;
        let variant_ident = self.name().as_pascal_ident();

        if let DataTypeKind::Struct(data_type_struct) = data.kind() {
            let paths = data_type_struct.field_paths(isa, field_name);
            let value =
                FieldPath::value_tokens(&paths, getter_ident, MemberAccess::Binding, presence)?;
            let bindings = paths.iter().map(|p| p.member());
            Some(quote!(Self::#variant_ident { #(#bindings),*, .. } => #value,))
        } else {
            let paths = FieldPath::find_all([(data.name().0.as_str(), data)], isa, field_name);
            let value =
                FieldPath::value_tokens(&paths, getter_ident, MemberAccess::Binding, presence)?;
            let binding = data.name().as_ident();
            Some(quote!(Self::#variant_ident(#binding) => #value,))
        }
    }
}
