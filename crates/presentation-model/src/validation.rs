use crate::Presentation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub location: ValidationLocation,
    pub kind: ValidationErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationErrors(pub Vec<ValidationError>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationLocation {
    Presentation,
    Slide {
        slide_index: usize,
    },
    Layer {
        slide_index: usize,
        layer_index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationErrorKind {
    NonPositiveSlideWidth,
    NonPositiveSlideHeight,
    EmptySlideId,
    EmptyLayerId,
    NonPositiveLayerWidth,
    NonPositiveLayerHeight,
    DuplicateSlideId,
    DuplicateLayerId,
}

pub fn validate(presentation: &Presentation) -> Result<(), ValidationErrors> {
    let mut errors = Vec::new();

    if presentation.size.width.0 <= 0 {
        errors.push(ValidationError {
            location: ValidationLocation::Presentation,
            kind: ValidationErrorKind::NonPositiveSlideWidth,
        })
    }

    if presentation.size.height.0 <= 0 {
        errors.push(ValidationError {
            location: ValidationLocation::Presentation,
            kind: ValidationErrorKind::NonPositiveSlideHeight,
        })
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ValidationErrors(errors))
    }
}

