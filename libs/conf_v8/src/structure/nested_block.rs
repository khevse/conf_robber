
use structure::attributes::Attributes;

// Вложенный блок
#[derive(Clone)]
pub struct NestedBlock {
    pub attrs: Attributes,
    pub data: Vec<u8>,
}

impl NestedBlock {
    pub fn new(attrs: &Attributes, data: &Vec<u8>) -> NestedBlock {

        return NestedBlock {
            attrs: attrs.clone(),
            data: data.clone(),
        };
    }
}
