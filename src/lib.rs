//! Derive values for structs and enums
//!

// /// Attribute macro: shorthand for `#[cfg_attr(feature = "c_compatible", repr(C))]`
// ///
// /// Applies C-compatible memory layout when the "`c_compatible`" feature is enabled.
// ///
// /// # Example
// /// ```ignore
// /// #[c_compatible]
// /// pub struct CData {
// ///     field1: i32,
// ///     field2: u8,
// /// }
// /// ```
// #[proc_macro_attribute]
// pub fn c_compatible(_args: TokenStream, input: TokenStream) -> TokenStream {
//     let mut item = parse_macro_input!(input as DeriveInput);

//     item.attrs.push(syn::parse_quote! {
//         #[cfg_attr(feature = "c_compatible", repr(C))]
//     });

//     quote! { #item }.into()
// }

/// Attribute macro to conditionally derive codec traits.
/// If you use this in conjunction with the builtin `derive` and `derive_const`, place this macro above the derive calls.
///
/// ---
///
/// Applies serialization/deserialization derives based on enabled features.
///
/// Automatically chooses which features to enable/disable based on what the item the derive is used on.
///
/// Options:
/// - Disable: `false`
/// - Non const version: `nonconst`/`non_const`
///
/// Configurable attributes:
/// - Supported crates: `wincode`, `bitcode`, `serde`, `strum`, `enum_ext`, `c_compatible`, `zerocopy`, and `compactly`
/// - Attributes: `read_only`
/// - Builtin: `Debug`, `Clone`, `Clone`, `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Default`, and `Hash` (requires snake case instead of camel case)
///
/// ### Warning:
/// `wincode` has been temporarily disabled until the author fixes their `overly complex generic constant` problem. Please use `bitcode` in the meanwhile.
///
/// # Example
/// If `wincode`, `serde`, and `zerocopy` were to give issues, you could just disable them
/// ``` ignore
/// #[mirl_macros::derive_all(wincode = false, serde = false, zerocopy = false)]
/// pub struct MyData {
///     value: i32,
/// }
/// ```
///
/// # Panics
/// When it cannot parse the given item
// TODO: Automatically add `default=true` if `#[default]` is used in an enum
#[proc_macro_attribute]
pub fn derive_all(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = syn::parse_macro_input!(input as syn::DeriveInput);
    let mut flags = syn::parse_macro_input!(args as mirl_macros_core::flag_list::FlagList);

    let (read_only, dependencies, feature_flags) =
        mirl_macros_core::get_crate_and_feature_and_read_only_from_flag_list(&mut flags);
    mirl_macros_core::handle_derive_all(
        item,
        flags,
        read_only,
        dependencies.as_deref(),
        feature_flags.as_deref(),
    )
    .into()
}
