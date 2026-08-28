use presentation_model::{
    Emu, Frame, Layer, LayerContent, LayerId, Presentation, Slide, SlideId, SlideSize, TextContent,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn presentation_round_trips_through_json() {
        let original = Presentation {
            size: SlideSize {
                width: Emu(50),
                height: Emu(50),
            },
            slides: vec![Slide {
                slide_id: SlideId("slide-1".to_owned()),
                layers: vec![Layer {
                    layer_id: LayerId("layer-1".to_owned()),
                    frame: Frame {
                        x: Emu(5),
                        y: Emu(5),
                        width: Emu(20),
                        height: Emu(10),
                    },
                    content: LayerContent::Text(TextContent {
                        text: "Hello".to_owned(),
                    }),
                }],
            }],
        };

        let json = serde_json::to_string(&original).expect("presentation should serialize");

        let decoded: Presentation =
            serde_json::from_str(&json).expect("Presentation should deserialize");

        assert_eq!(decoded, original);
    }
}
