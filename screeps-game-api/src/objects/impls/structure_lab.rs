use stdweb::unstable::TryInto;

use {
    constants::{ResourceType, ReturnCode},
    {Creep, StructureLab},
};

simple_accessors! {
    StructureLab;
    (mineral_amount -> mineralAmount -> u32),
    // mineralType
    (mineral_capacity -> mineralCapacity -> u32),
}

impl StructureLab {
    pub fn mineral_type(&self) -> ResourceType {
        let resource: String = js_unwrap! {
            return @{self.as_ref()}.mineralType;
        };
        resource.try_into().unwrap()
    }

    pub fn boost_creep(&self, creep: &Creep, body_part_count: Option<u32>) -> ReturnCode {
        match body_part_count {
            None => js_unwrap! {@{self.as_ref()}.boostCreep(@{creep.as_ref()})},
            Some(count) => js_unwrap! {@{self.as_ref()}.boostCreep(@{creep.as_ref()}, @{count})}
        }
    }

    pub fn run_reaction(&self, lab1: &StructureLab, lab2: &StructureLab) -> ReturnCode {
        js_unwrap!{@{self.as_ref()}.runReaction(@{lab1.as_ref()}, @{lab2.as_ref()})}
    }
}