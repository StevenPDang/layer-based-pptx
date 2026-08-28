use presentation_model::*;

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
