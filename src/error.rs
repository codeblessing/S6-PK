use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Diagnostic,  Error)]
#[error("playfair key can contain only ASCII alphabetic characters")]
#[diagnostic(
    code(playfair::error::non_ascii_key),
    help("try to remove non-ascii characters from key.")
)]
pub struct NonAsciiKey {
    #[source_code]
    src: NamedSource,
    #[label("non-ASCII symbol")]
    span: SourceSpan
}

impl NonAsciiKey {
    pub fn new(src: impl Into<NamedSource>, span: impl Into<SourceSpan>) -> Self {
        Self {
            src: src.into(),
            span: span.into()
        }
    }
}