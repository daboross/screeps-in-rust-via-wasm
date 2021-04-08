use stdweb::{InstanceOf, Reference, ReferenceType, Value};

use super::*;
use crate::{
    constants::StructureType,
    objects::{Attackable, CanDecay, HasCooldown, HasEnergyForSpawn, HasStore},
    traits::FromExpectedType,
    ConversionError,
};

/// Wrapper which can be any of the game Structures.
///
/// This is somewhat useful by itself, but has additional utility methods. Some
/// tricks:
///
/// To get a particular type, `match` on the structure:
///
/// ```no_run
/// use screeps::Structure;
///
/// # let my_struct: Structure = unimplemented!();
/// match my_struct {
///     Structure::Container(cont) => {
///         // cont here is StructureContainer
///     }
///     _ => {
///         // other structure
///     }
/// }
/// ```
///
/// To use structures of a particular type, like something that can be attacked,
/// or something that can be transferred to, use helper methods:
/// ```no_run
/// use screeps::Structure;
///
/// # let my_struct: Structure = unimplemented!();
/// match my_struct.as_transferable() {
///     Some(transf) => {
///         // transf is a reference to `dyn Transferable`, and you can transfer to it.
///     }
///     None => {
///         // my_struct is not transferable
///     }
/// }
/// ```
///
/// See method documentation for a full list of possible helpers.
#[derive(Clone)]
pub enum Structure {
    Container(StructureContainer),
    Controller(StructureController),
    Extension(StructureExtension),
    Extractor(StructureExtractor),
    Factory(StructureFactory),
    InvaderCore(StructureInvaderCore),
    KeeperLair(StructureKeeperLair),
    Lab(StructureLab),
    Link(StructureLink),
    Nuker(StructureNuker),
    Observer(StructureObserver),
    PowerBank(StructurePowerBank),
    PowerSpawn(StructurePowerSpawn),
    Portal(StructurePortal),
    Rampart(StructureRampart),
    Road(StructureRoad),
    Spawn(StructureSpawn),
    Storage(StructureStorage),
    Terminal(StructureTerminal),
    Tower(StructureTower),
    Wall(StructureWall),
}

impl Structure {
    /// Cast this structure as something Transferable, or return None if it
    /// isn't.
    ///
    /// Example usage:
    ///
    /// ```no_run
    /// use screeps::{Creep, ResourceType, Structure};
    ///
    /// # let my_struct: Structure = unimplemented!();
    /// # let my_creep: Creep = unimplemented!();
    /// match my_struct.as_transferable() {
    ///     Some(transf) => {
    ///         // transf is a reference to `dyn Transferable`, and you can transfer to it.
    ///         my_creep.transfer_all(transf, ResourceType::Energy);
    ///     }
    ///     None => {
    ///         // my_struct cannot be transferred to
    ///     }
    /// }
    /// ```
    pub fn as_transferable(&self) -> Option<&dyn Transferable> {
        match_some_structure_variants!(
            self,
            {
                Container, Extension, Factory, Lab, Link, Nuker, PowerSpawn, Spawn, Storage, Terminal, Tower
            },
            v => v
        )
    }

    /// Cast this as something which can be withdrawn from
    pub fn as_withdrawable(&self) -> Option<&dyn Withdrawable> {
        match_some_structure_variants!(
            self,
            {
                Container, Extension, Factory, Lab, Link, PowerSpawn, Spawn, Storage, Terminal, Tower
            },
            v => v
        )
    }

    /// Cast this as something which can be attacked and has hit points.
    ///
    /// The only Structure which cannot be attacked is `StructureController`.
    pub fn as_attackable(&self) -> Option<&dyn Attackable> {
        // We're not using `match_some_structure_variants!` here or in `as_owned` so we
        // won't have a `_ => None` branch and instead we'll be forced to add
        // new structures to the match explicitly. The other functions would be
        // using `_ => None` anyways since they have more `None` branches.
        match self {
            Structure::Controller(_) => None,
            Structure::Container(v) => Some(v),
            Structure::Extension(v) => Some(v),
            Structure::Extractor(v) => Some(v),
            Structure::Factory(v) => Some(v),
            Structure::InvaderCore(v) => Some(v),
            Structure::KeeperLair(v) => Some(v),
            Structure::Lab(v) => Some(v),
            Structure::Link(v) => Some(v),
            Structure::Nuker(v) => Some(v),
            Structure::Observer(v) => Some(v),
            Structure::PowerBank(v) => Some(v),
            Structure::PowerSpawn(v) => Some(v),
            Structure::Portal(_) => None,
            Structure::Rampart(v) => Some(v),
            Structure::Road(v) => Some(v),
            Structure::Spawn(v) => Some(v),
            Structure::Storage(v) => Some(v),
            Structure::Terminal(v) => Some(v),
            Structure::Tower(v) => Some(v),
            Structure::Wall(v) => Some(v),
        }
    }

    /// Cast this as something which can be owned.
    ///
    /// Example:
    ///
    /// ```no_run
    /// use screeps::Structure;
    ///
    /// # let my_struct: Structure = unimplemented!();
    /// let is_my = my_struct.as_owned().map(|os| os.my()).unwrap_or(false);
    /// ```
    pub fn as_owned(&self) -> Option<&dyn OwnedStructureProperties> {
        match self {
            Structure::Container(_) => None,
            Structure::Controller(v) => Some(v),
            Structure::Extension(v) => Some(v),
            Structure::Extractor(v) => Some(v),
            Structure::Factory(v) => Some(v),
            Structure::InvaderCore(v) => Some(v),
            Structure::KeeperLair(v) => Some(v),
            Structure::Lab(v) => Some(v),
            Structure::Link(v) => Some(v),
            Structure::Nuker(v) => Some(v),
            Structure::Observer(v) => Some(v),
            Structure::PowerBank(v) => Some(v),
            Structure::PowerSpawn(v) => Some(v),
            Structure::Portal(_) => None,
            Structure::Rampart(v) => Some(v),
            Structure::Road(_) => None,
            Structure::Spawn(v) => Some(v),
            Structure::Storage(v) => Some(v),
            Structure::Terminal(v) => Some(v),
            Structure::Tower(v) => Some(v),
            Structure::Wall(_) => None,
        }
    }

    pub fn as_can_decay(&self) -> Option<&dyn CanDecay> {
        match_some_structure_variants!(
            self,
            {
                Container, Portal, PowerBank, Rampart, Road
            },
            v => v
        )
    }

    pub fn as_has_cooldown(&self) -> Option<&dyn HasCooldown> {
        match_some_structure_variants!(
            self,
            {
                Extractor, Factory, Lab, Link, Nuker, Terminal
            },
            v => v
        )
    }

    pub fn as_has_energy_for_spawn(&self) -> Option<&dyn HasEnergyForSpawn> {
        match_some_structure_variants!(
            self,
            {
                Extension, Spawn
            },
            v => v
        )
    }
    pub fn as_has_store(&self) -> Option<&dyn HasStore> {
        match_some_structure_variants!(
            self,
            {
                Container, Extension, Factory, Lab, Link, Nuker, PowerSpawn, Spawn, Storage, Terminal, Tower
            },
            v => v
        )
    }
}

impl AsRef<Reference> for Structure {
    fn as_ref(&self) -> &Reference {
        match_structure_variants!(self, v => v.as_ref())
    }
}

impl From<Structure> for Reference {
    fn from(wrapper: Structure) -> Reference {
        match_structure_variants!(wrapper, v => v.0)
    }
}

fn get_structure_type(structure: &Reference) -> Result<StructureType, ConversionError> {
    (js! {
        return __structure_type_str_to_num(@{structure}.structureType);
    })
    .try_into()
    .map_err(Into::into)
}

impl FromExpectedType<Reference> for Structure {
    fn from_expected_type(reference: Reference) -> Result<Self, ConversionError> {
        let structure_type = get_structure_type(&reference)?;
        let structure = construct_structure_variants!(
            structure_type => reference.into_expected_type()?
        );

        Ok(structure)
    }
}

impl TryFrom<Reference> for Structure {
    type Error = ConversionError;

    fn try_from(reference: Reference) -> Result<Self, ConversionError> {
        let structure_type = get_structure_type(&reference)?;

        let structure = construct_structure_variants!(
            structure_type => reference.try_into()?
        );

        Ok(structure)
    }
}

impl InstanceOf for Structure {
    fn instance_of(reference: &Reference) -> bool {
        js_unwrap!(@{reference} instanceof Structure)
    }
}

impl TryFrom<Value> for Structure {
    type Error = ConversionError;

    fn try_from(v: Value) -> Result<Structure, Self::Error> {
        Reference::try_from(v).and_then(Self::try_from)
    }
}

impl ReferenceType for Structure {
    unsafe fn from_reference_unchecked(reference: Reference) -> Self {
        let structure_type = js_unwrap!(__structure_type_str_to_num(@{&reference}.structureType));

        construct_structure_variants!(
            structure_type => ReferenceType::from_reference_unchecked(reference)
        )
    }
}
