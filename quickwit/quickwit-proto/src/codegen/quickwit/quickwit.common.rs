/// The corresponding Rust struct \[`crate::types::DocMappingUid`\] is defined manually and
/// externally provided during code generation (see `build.rs`).
///
/// Modify at your own risk.
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocMappingUid {
    #[prost(bytes = "vec", tag = "1")]
    pub doc_mapping_uid: ::prost::alloc::vec::Vec<u8>,
}
