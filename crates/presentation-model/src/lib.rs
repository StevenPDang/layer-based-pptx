use serde::{Deserialize, Serialize}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(renameAll = "camelCase")]
pub struct Presentation {
    pub slides: Vec<Slide>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(renameAll = "camelCase")]
pub struct Slide {
    pub id: SlideId,
    pub layers: Vec<Layer>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(renameAll = "camelCase")]
pub struct Layer {
    pub id: LayerId,
    pub frame: Frame,
    pub content: LayerContent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(renameAll = "camelCase")]
pub enum LayerContent {
    Text(TextContent),
    Image(ImageContent),
    Vector(VectorContent),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(renameAll = "camelCase")]
pub struct TextContent {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(renameAll = "camelCase")]
pub struct ImageContent {
    pub source: String,
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(renameAll = "camelCase")]
pub struct Frame {
    pub x: i64
    pub y: i64,
    pub width: i64,
    pub heigth: i64
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LayerId(pub String);


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SlideId(pub String);