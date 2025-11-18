//! # EVE ESI Asset Enums
//!
//! Provides enums related to assets in EVE Online
//!
//! ## Enums
//! - [`LocationType`]: The type of location for an asset's location ID
//! - [`LocationFlag`]: Indicates where an item, module, or ship in EVE Online is located

use serde::{Deserialize, Serialize};

/// The type of location for an asset's location ID
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdAssetsGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum LocationType {
    /// Item location type is a station
    #[serde(rename = "station")]
    Station,
    /// Item location type is a solar system
    #[serde(rename = "solar_sytem")]
    SolarSystem,
    /// Item location type is within an item
    #[serde(rename = "item")]
    Item,
    /// Item location type is other
    #[serde(rename = "other")]
    Other,
}

/// Indicates where an item, module, or ship in EVE Online is located
///
/// This enum represents the location of items, ships, and modules returned by primarily assets
/// endpoints and other item or ship-related ESI endpoints
///
/// Note: most fields have documentation, if they are documented with `???` then it is unknown
/// what exactly this location field actually applies to in-game.
///
/// # Documentation
/// - <https://developers.eveonline.com/api-explorer#/schemas/CharactersCharacterIdBlueprintsGet>
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum LocationFlag {
    /// ???
    AutoFit,
    /// Ship's cargo bay
    Cargo,
    /// Specialized corpse bay (Blood Raider ships)
    CorpseBay,
    /// Ship's drone bay
    DroneBay,
    /// Ship's fleet hangar
    FleetHangar,
    /// Station deliveries hangar
    Deliveries,
    /// ???
    HiddenModifiers,
    /// Station hangar
    Hangar,
    /// Station hangar?
    HangarAll,
    /// Ship or structure's low slot #1
    LoSlot0,
    /// Ship or structure's low slot #2
    LoSlot1,
    /// Ship or structure's low slot #3
    LoSlot2,
    /// Ship or structure's low slot #4
    LoSlot3,
    /// Ship or structure's low slot #5
    LoSlot4,
    /// Ship or structure's low slot #6
    LoSlot5,
    /// Ship or structure's low slot #7
    LoSlot6,
    /// Ship or structure's low slot #8
    LoSlot7,
    /// Ship or structure's middle slot #1
    MedSlot0,
    /// Ship or structure's middle slot #2
    MedSlot1,
    /// Ship or structure's middle slot #3
    MedSlot2,
    /// Ship or structure's middle slot #4
    MedSlot3,
    /// Ship or structure's middle slot #5
    MedSlot4,
    /// Ship or structure's middle slot #6
    MedSlot5,
    /// Ship or structure's middle slot #7
    MedSlot6,
    /// Ship or structure's middle slot #8
    MedSlot7,
    /// Ship or structure's high slot #1
    HiSlot0,
    /// Ship or structure's high slot #2
    HiSlot1,
    /// Ship or structure's high slot #3
    HiSlot2,
    /// Ship or structure's high slot #4
    HiSlot3,
    /// Ship or structure's high slot #5
    HiSlot4,
    /// Ship or structure's high slot #6
    HiSlot5,
    /// Ship or structure's high slot #7
    HiSlot6,
    /// Ship or structure's high slot #8
    HiSlot7,
    /// Station asset safety storage
    AssetSafety,
    /// ???
    Locked,
    /// ???
    Unlocked,
    /// Implant slot on a character
    Implant,
    /// Ship's specialized quafe storage bay
    QuafeBay,
    /// Ship or structure's rig slot #1
    RigSlot0,
    /// Ship or structure's rig slot #2
    RigSlot1,
    /// Ship or structure's rig slot #3
    RigSlot2,
    /// Ship or structure's rig slot #4
    RigSlot3,
    /// Ship or structure's rig slot #5
    RigSlot4,
    /// Ship or structure's rig slot #6
    RigSlot5,
    /// Ship or structure's rig slot #7
    RigSlot6,
    /// Ship or structure's rig slot #8
    RigSlot7,
    /// Station's ship hangar storage bay
    ShipHangar,
    /// Ship's specialized fuel storage bay
    SpecializedFuelBay,
    /// Ship's specialized ore storage bay
    SpecializedOreHold,
    /// Ship's specialized gas storage bay
    SpecializedGasHold,
    /// Ship's specialized mineral storage bay
    SpecializedMineralHold,
    /// Ship's specialized salvage storage bay
    SpecializedSalvageHold,
    /// Ship's specialized ship storage bay
    SpecializedShipHold,
    /// Ship's specialized small ship storage bay
    SpecializedSmallShipHold,
    /// Ship's specialized medium ship storage bay
    SpecializedMediumShipHold,
    /// Ship's specialized large ship storage bay
    SpecializedLargeShipHold,
    /// Ship's specialized industrial ship storage bay
    SpecializedIndustrialShipHold,
    /// Ship's specialized ammo storage bay
    SpecializedAmmoHold,
    /// Ship's specialized command center storage bay (Epithal, Primae)
    SpecializedCommandCenterHold,
    /// Ship's specialized command center storage bay (Epithal, Primae)
    SpecializedPlanetaryCommoditiesHold,
    /// Ship's specialized material bay
    SpecializedMaterialBay,
    /// Ship's subsystem slot #1 (T3 cruisers)
    SubSystemSlot0,
    /// Ship's subsystem slot #2 (T3 cruisers)
    SubSystemSlot1,
    /// Ship's subsystem slot #3 (T3 cruisers)
    SubSystemSlot2,
    /// Ship's subsystem slot #4 (T3 cruisers)
    SubSystemSlot3,
    /// Ship's subsystem slot #5 (T3 cruisers)
    SubSystemSlot4,
    /// Ship's subsystem slot #6 (T3 cruisers)
    SubSystemSlot5,
    /// Ship's subsystem slot #7 (T3 cruisers)
    SubSystemSlot6,
    /// Ship's subsystem slot #8 (T3 cruisers)
    SubSystemSlot7,
    /// Carrier or structure's fighter bay
    FighterBay,
    /// Carrier or structure's fighter tube #1
    FighterTube0,
    /// Carrier or structure's fighter tube #2
    FighterTube1,
    /// Carrier or structure's fighter tube #3
    FighterTube2,
    /// Carrier or structure's fighter tube #4
    FighterTube3,
    /// Carrier or structure's fighter tube #5
    FighterTube4,
    /// ???
    Module,
}
