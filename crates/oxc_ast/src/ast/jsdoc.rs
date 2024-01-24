//! [`JSDoc`](https://github.com/microsoft/TypeScript/blob/54a554d8af2657630307cbfa8a3e4f3946e36507/src/compiler/types.ts#L393)

use oxc_span::Span;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{ast::TSType, unconfigured_serde_ts};

#[derive(Debug, Hash)]
#[apply(unconfigured_serde_ts!)]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "camelCase"))]
pub struct JSDocNullableType<'a> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
    pub type_annotation: TSType<'a>,
    pub postfix: bool,
}

#[derive(Debug, Hash)]
#[apply(unconfigured_serde_ts!)]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "camelCase"))]
pub struct JSDocUnknownType {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub span: Span,
}
