use crate::{constants::ResourceType, objects::SymbolDecoder};

simple_accessors! {
    impl SymbolDecoder {
        pub fn score_multiplier() -> u32 = scoreMultiplier;
    }
}

impl SymbolDecoder {
    pub fn resource_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.resourceType))
    }
}
