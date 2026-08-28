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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Emu, SlideSize};

    #[test]
    fn rejects_non_positive_slide_width() {
        // Arrange
        let presentation = Presentation {
            size: SlideSize {
                width: Emu(0),
                height: Emu(100),
            },
            slides: vec![],
        };

        let result = validate(&presentation);

        let errors = result.expect_err("zero width should be invalid");

        assert_eq!(
            errors,
            ValidationErrors(vec![ValidationError {
                location: ValidationLocation::Presentation,
                kind: ValidationErrorKind::NonPositiveSlideWidth
            }])
        );
    }

    #[test]
    fn rejects_non_positive_slide_height() {
        let presentation = Presentation {
            size: SlideSize {
                width: Emu(100),
                height: Emu(0),
            },
            slides: vec![],
        };

        let result = validate(&presentation);

        let errors = result.expect_err("zero height should be invalid");

        assert_eq!(
            errors,
            ValidationErrors(vec![ValidationError {
                location: ValidationLocation::Presentation,
                kind: ValidationErrorKind::NonPositiveSlideHeight
            }])
        );
    }
}
