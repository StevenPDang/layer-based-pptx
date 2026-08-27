//! Format-independent presentation document types.

mod model;
mod validation;

pub use model::{
    Emu, Frame, Layer, LayerContent, LayerId, Presentation, Slide, SlideId, SlideSize, TextContent,
};
pub use validation::{
    ValidationError, ValidationErrorKind, ValidationErrors, ValidationLocation, validate,
};
