use crate::{constants::ResourceType, objects::SymbolContainer};

impl SymbolContainer {
    pub fn resource_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.resourceType))
    }
}
