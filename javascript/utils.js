function __part_num_to_str(num) {
    switch (num) {
        case 0: return MOVE;
        case 1: return WORK;
        case 2: return CARRY;
        case 3: return ATTACK;
        case 4: return RANGED_ATTACK;
        case 5: return TOUGH;
        case 6: return HEAL;
        case 7: return CLAIM;
        default: throw new Error("unknown part integer encoding " + num);
    }
}

function __part_str_to_num(str) {
    switch (str) {
        case MOVE: return 0;
        case WORK: return 1;
        case CARRY: return 2;
        case ATTACK: return 3;
        case RANGED_ATTACK: return 4;
        case TOUGH: return 5;
        case HEAL: return 6;
        case CLAIM: return 7;
        default: throw new Error("unknown part type " + str);
    }
}

function __look_num_to_str(num) {
    switch (num) {
        case 0: return LOOK_CREEPS;
        case 1: return LOOK_ENERGY;
        case 2: return LOOK_RESOURCES;
        case 3: return LOOK_SOURCES;
        case 4: return LOOK_MINERALS;
        case 5: return LOOK_STRUCTURES;
        case 6: return LOOK_FLAGS;
        case 7: return LOOK_CONSTRUCTION_SITES;
        case 8: return LOOK_NUKES;
        case 9: return LOOK_TERRAIN;
        case 10: return LOOK_TOMBSTONES;
        case 11: return LOOK_POWER_CREEPS;
        case 12: return LOOK_DEPOSITS;
        case 13: return LOOK_RUINS;
        case 10011: return LOOK_SCORE_CONTAINERS;
        case 10012: return LOOK_SCORE_COLLECTORS;
        default: throw new Error("unknown look integer encoding " + num);
    }
}

function __look_str_to_num(num) {
    switch (num) {
        case LOOK_CREEPS: return 0;
        case LOOK_ENERGY: return 1;
        case LOOK_RESOURCES: return 2;
        case LOOK_SOURCES: return 3;
        case LOOK_MINERALS: return 4;
        case LOOK_STRUCTURES: return 5;
        case LOOK_FLAGS: return 6;
        case LOOK_CONSTRUCTION_SITES: return 7;
        case LOOK_NUKES: return 8;
        case LOOK_TERRAIN: return 9;
        case LOOK_TOMBSTONES: return 10;
        case LOOK_POWER_CREEPS: return 11;
        case LOOK_DEPOSITS: return 12;
        case LOOK_RUINS: return 13;
        case LOOK_SCORE_CONTAINERS: return 10011;
        case LOOK_SCORE_COLLECTORS: return 10012;
        default: throw new Error("unknown look constant " + num);
    }
}

function __structure_type_num_to_str(num) {
    switch (num) {
        case 0: return STRUCTURE_SPAWN;
        case 1: return STRUCTURE_EXTENSION;
        case 2: return STRUCTURE_ROAD;
        case 3: return STRUCTURE_WALL;
        case 4: return STRUCTURE_RAMPART;
        case 5: return STRUCTURE_KEEPER_LAIR;
        case 6: return STRUCTURE_PORTAL;
        case 7: return STRUCTURE_CONTROLLER;
        case 8: return STRUCTURE_LINK;
        case 9: return STRUCTURE_STORAGE;
        case 10: return STRUCTURE_TOWER;
        case 11: return STRUCTURE_OBSERVER;
        case 12: return STRUCTURE_POWER_BANK;
        case 13: return STRUCTURE_POWER_SPAWN;
        case 14: return STRUCTURE_EXTRACTOR;
        case 15: return STRUCTURE_LAB;
        case 16: return STRUCTURE_TERMINAL;
        case 17: return STRUCTURE_CONTAINER;
        case 18: return STRUCTURE_NUKER;
        case 19: return STRUCTURE_FACTORY;
        case 20: return STRUCTURE_INVADER_CORE;
        default: throw new Error("unknown structure type integer encoding " + num);
    }
}

function __structure_type_str_to_num(str) {
    switch (str) {
        case STRUCTURE_SPAWN: return 0;
        case STRUCTURE_EXTENSION: return 1;
        case STRUCTURE_ROAD: return 2;
        case STRUCTURE_WALL: return 3;
        case STRUCTURE_RAMPART: return 4;
        case STRUCTURE_KEEPER_LAIR: return 5;
        case STRUCTURE_PORTAL: return 6;
        case STRUCTURE_CONTROLLER: return 7;
        case STRUCTURE_LINK: return 8;
        case STRUCTURE_STORAGE: return 9;
        case STRUCTURE_TOWER: return 10;
        case STRUCTURE_OBSERVER: return 11;
        case STRUCTURE_POWER_BANK: return 12;
        case STRUCTURE_POWER_SPAWN: return 13;
        case STRUCTURE_EXTRACTOR: return 14;
        case STRUCTURE_LAB: return 15;
        case STRUCTURE_TERMINAL: return 16;
        case STRUCTURE_CONTAINER: return 17;
        case STRUCTURE_NUKER: return 18;
        case STRUCTURE_FACTORY: return 19;
        case STRUCTURE_INVADER_CORE: return 20;
        default: throw new Error("unknown structure type " + str);
    }
}

function __resource_type_num_to_str(num) {
    switch (num) {
        case 1: return RESOURCE_ENERGY;
        case 2: return RESOURCE_POWER;
        case 3: return RESOURCE_HYDROGEN;
        case 4: return RESOURCE_OXYGEN;
        case 5: return RESOURCE_UTRIUM;
        case 6: return RESOURCE_LEMERGIUM;
        case 7: return RESOURCE_KEANIUM;
        case 8: return RESOURCE_ZYNTHIUM;
        case 9: return RESOURCE_CATALYST;
        case 10: return RESOURCE_GHODIUM;
        case 11: return RESOURCE_HYDROXIDE;
        case 12: return RESOURCE_ZYNTHIUM_KEANITE;
        case 13: return RESOURCE_UTRIUM_LEMERGITE;
        case 14: return RESOURCE_UTRIUM_HYDRIDE;
        case 15: return RESOURCE_UTRIUM_OXIDE;
        case 16: return RESOURCE_KEANIUM_HYDRIDE;
        case 17: return RESOURCE_KEANIUM_OXIDE;
        case 18: return RESOURCE_LEMERGIUM_HYDRIDE;
        case 19: return RESOURCE_LEMERGIUM_OXIDE;
        case 20: return RESOURCE_ZYNTHIUM_HYDRIDE;
        case 21: return RESOURCE_ZYNTHIUM_OXIDE;
        case 22: return RESOURCE_GHODIUM_HYDRIDE;
        case 23: return RESOURCE_GHODIUM_OXIDE;
        case 24: return RESOURCE_UTRIUM_ACID;
        case 25: return RESOURCE_UTRIUM_ALKALIDE;
        case 26: return RESOURCE_KEANIUM_ACID;
        case 27: return RESOURCE_KEANIUM_ALKALIDE;
        case 28: return RESOURCE_LEMERGIUM_ACID;
        case 29: return RESOURCE_LEMERGIUM_ALKALIDE;
        case 30: return RESOURCE_ZYNTHIUM_ACID;
        case 31: return RESOURCE_ZYNTHIUM_ALKALIDE;
        case 32: return RESOURCE_GHODIUM_ACID;
        case 33: return RESOURCE_GHODIUM_ALKALIDE;
        case 34: return RESOURCE_CATALYZED_UTRIUM_ACID;
        case 35: return RESOURCE_CATALYZED_UTRIUM_ALKALIDE;
        case 36: return RESOURCE_CATALYZED_KEANIUM_ACID;
        case 37: return RESOURCE_CATALYZED_KEANIUM_ALKALIDE;
        case 38: return RESOURCE_CATALYZED_LEMERGIUM_ACID;
        case 39: return RESOURCE_CATALYZED_LEMERGIUM_ALKALIDE;
        case 40: return RESOURCE_CATALYZED_ZYNTHIUM_ACID;
        case 41: return RESOURCE_CATALYZED_ZYNTHIUM_ALKALIDE;
        case 42: return RESOURCE_CATALYZED_GHODIUM_ACID;
        case 43: return RESOURCE_CATALYZED_GHODIUM_ALKALIDE;
        case 44: return RESOURCE_OPS;
        case 45: return RESOURCE_SILICON;
        case 46: return RESOURCE_METAL;
        case 47: return RESOURCE_BIOMASS;
        case 48: return RESOURCE_MIST;
        case 49: return RESOURCE_UTRIUM_BAR;
        case 50: return RESOURCE_LEMERGIUM_BAR;
        case 51: return RESOURCE_ZYNTHIUM_BAR;
        case 52: return RESOURCE_KEANIUM_BAR;
        case 53: return RESOURCE_GHODIUM_MELT;
        case 54: return RESOURCE_OXIDANT;
        case 55: return RESOURCE_REDUCTANT;
        case 56: return RESOURCE_PURIFIER;
        case 57: return RESOURCE_BATTERY;
        case 58: return RESOURCE_COMPOSITE;
        case 59: return RESOURCE_CRYSTAL;
        case 60: return RESOURCE_LIQUID;
        case 61: return RESOURCE_WIRE;
        case 62: return RESOURCE_SWITCH;
        case 63: return RESOURCE_TRANSISTOR;
        case 64: return RESOURCE_MICROCHIP;
        case 65: return RESOURCE_CIRCUIT;
        case 66: return RESOURCE_DEVICE;
        case 67: return RESOURCE_CELL;
        case 68: return RESOURCE_PHLEGM;
        case 69: return RESOURCE_TISSUE;
        case 70: return RESOURCE_MUSCLE;
        case 71: return RESOURCE_ORGANOID;
        case 72: return RESOURCE_ORGANISM;
        case 73: return RESOURCE_ALLOY;
        case 74: return RESOURCE_TUBE;
        case 75: return RESOURCE_FIXTURES;
        case 76: return RESOURCE_FRAME;
        case 77: return RESOURCE_HYDRAULICS;
        case 78: return RESOURCE_MACHINE;
        case 79: return RESOURCE_CONDENSATE;
        case 80: return RESOURCE_CONCENTRATE;
        case 81: return RESOURCE_EXTRACT;
        case 82: return RESOURCE_SPIRIT;
        case 83: return RESOURCE_EMANATION;
        case 84: return RESOURCE_ESSENCE;
        case 1001: return SUBSCRIPTION_TOKEN;
        case 1002: return CPU_UNLOCK;
        case 1003: return PIXEL;
        case 1004: return ACCESS_KEY;
        case 10001: return RESOURCE_SCORE;
        default: throw new Error("unknown resource type integer encoding " + num);
    }
}

function __resource_type_str_to_num(str) {
    switch (str) {
        case RESOURCE_ENERGY: return 1;
        case RESOURCE_POWER: return 2;
        case RESOURCE_HYDROGEN: return 3;
        case RESOURCE_OXYGEN: return 4;
        case RESOURCE_UTRIUM: return 5;
        case RESOURCE_LEMERGIUM: return 6;
        case RESOURCE_KEANIUM: return 7;
        case RESOURCE_ZYNTHIUM: return 8;
        case RESOURCE_CATALYST: return 9;
        case RESOURCE_GHODIUM: return 10;
        case RESOURCE_HYDROXIDE: return 11;
        case RESOURCE_ZYNTHIUM_KEANITE: return 12;
        case RESOURCE_UTRIUM_LEMERGITE: return 13;
        case RESOURCE_UTRIUM_HYDRIDE: return 14;
        case RESOURCE_UTRIUM_OXIDE: return 15;
        case RESOURCE_KEANIUM_HYDRIDE: return 16;
        case RESOURCE_KEANIUM_OXIDE: return 17;
        case RESOURCE_LEMERGIUM_HYDRIDE: return 18;
        case RESOURCE_LEMERGIUM_OXIDE: return 19;
        case RESOURCE_ZYNTHIUM_HYDRIDE: return 20;
        case RESOURCE_ZYNTHIUM_OXIDE: return 21;
        case RESOURCE_GHODIUM_HYDRIDE: return 22;
        case RESOURCE_GHODIUM_OXIDE: return 23;
        case RESOURCE_UTRIUM_ACID: return 24;
        case RESOURCE_UTRIUM_ALKALIDE: return 25;
        case RESOURCE_KEANIUM_ACID: return 26;
        case RESOURCE_KEANIUM_ALKALIDE: return 27;
        case RESOURCE_LEMERGIUM_ACID: return 28;
        case RESOURCE_LEMERGIUM_ALKALIDE: return 29;
        case RESOURCE_ZYNTHIUM_ACID: return 30;
        case RESOURCE_ZYNTHIUM_ALKALIDE: return 31;
        case RESOURCE_GHODIUM_ACID: return 32;
        case RESOURCE_GHODIUM_ALKALIDE: return 33;
        case RESOURCE_CATALYZED_UTRIUM_ACID: return 34;
        case RESOURCE_CATALYZED_UTRIUM_ALKALIDE: return 35;
        case RESOURCE_CATALYZED_KEANIUM_ACID: return 36;
        case RESOURCE_CATALYZED_KEANIUM_ALKALIDE: return 37;
        case RESOURCE_CATALYZED_LEMERGIUM_ACID: return 38;
        case RESOURCE_CATALYZED_LEMERGIUM_ALKALIDE: return 39;
        case RESOURCE_CATALYZED_ZYNTHIUM_ACID: return 40;
        case RESOURCE_CATALYZED_ZYNTHIUM_ALKALIDE: return 41;
        case RESOURCE_CATALYZED_GHODIUM_ACID: return 42;
        case RESOURCE_CATALYZED_GHODIUM_ALKALIDE: return 43;
        case RESOURCE_OPS: return 44;
        case RESOURCE_SILICON: return 45;
        case RESOURCE_METAL: return 46;
        case RESOURCE_BIOMASS: return 47;
        case RESOURCE_MIST: return 48;
        case RESOURCE_UTRIUM_BAR: return 49;
        case RESOURCE_LEMERGIUM_BAR: return 50;
        case RESOURCE_ZYNTHIUM_BAR: return 51;
        case RESOURCE_KEANIUM_BAR: return 52;
        case RESOURCE_GHODIUM_MELT: return 53;
        case RESOURCE_OXIDANT: return 54;
        case RESOURCE_REDUCTANT: return 55;
        case RESOURCE_PURIFIER: return 56;
        case RESOURCE_BATTERY: return 57;
        case RESOURCE_COMPOSITE: return 58;
        case RESOURCE_CRYSTAL: return 59;
        case RESOURCE_LIQUID: return 60;
        case RESOURCE_WIRE: return 61;
        case RESOURCE_SWITCH: return 62;
        case RESOURCE_TRANSISTOR: return 63;
        case RESOURCE_MICROCHIP: return 64;
        case RESOURCE_CIRCUIT: return 65;
        case RESOURCE_DEVICE: return 66;
        case RESOURCE_CELL: return 67;
        case RESOURCE_PHLEGM: return 68;
        case RESOURCE_TISSUE: return 69;
        case RESOURCE_MUSCLE: return 70;
        case RESOURCE_ORGANOID: return 71;
        case RESOURCE_ORGANISM: return 72;
        case RESOURCE_ALLOY: return 73;
        case RESOURCE_TUBE: return 74;
        case RESOURCE_FIXTURES: return 75;
        case RESOURCE_FRAME: return 76;
        case RESOURCE_HYDRAULICS: return 77;
        case RESOURCE_MACHINE: return 78;
        case RESOURCE_CONDENSATE: return 79;
        case RESOURCE_CONCENTRATE: return 80;
        case RESOURCE_EXTRACT: return 81;
        case RESOURCE_SPIRIT: return 82;
        case RESOURCE_EMANATION: return 83;
        case RESOURCE_ESSENCE: return 84;
        case SUBSCRIPTION_TOKEN: return 1001;
        case CPU_UNLOCK: return 1002;
        case PIXEL: return 1003;
        case ACCESS_KEY: return 1004;
        case RESOURCE_SCORE: return 10001;
        default: throw new Error("unknown resource type " + str);
    }
}

function __order_type_str_to_num(str) {
    switch (str) {
        case ORDER_SELL: return 0;
        case ORDER_BUY: return 1;
        default: throw new Error("unknown order type " + str);
    }
}

function __order_type_num_to_str(num) {
    switch (num) {
        case 0: return ORDER_SELL;
        case 1: return ORDER_BUY;
        default: throw new Error("unknown order type " + num);
    }
}

function __terrain_str_to_num(str) {
    switch (str) {
        case 'plain': return 0;
        case 'swamp': return TERRAIN_MASK_SWAMP;
        case 'wall': return TERRAIN_MASK_WALL;
        default: throw new Error("unknown terrain type " + str);
    }
}

function __power_creep_class_str_to_num(str) {
    switch (str) {
        case POWER_CLASS.OPERATOR: return 1;
        default: throw new Error("unknown power creep class " + str);
    }
}

function __power_creep_class_num_to_str(num) {
    switch (num) {
        case 1: return POWER_CLASS.OPERATOR;
        default: throw new Error("unknown power creep class " + num);
    }
}

function pos_from_packed(repr) {
    // mimick the RoomPosition constructor
    let pos = Object.create(RoomPosition.prototype);
    Object.defineProperty(pos, "__packedPos", {
        enumerable: false,
        value: repr,
        writable: true,
    });
    return pos;
}

function object_id_from_packed(slice) {
    // reconstruct string in JS
    let res = "";
    for (var i = 0; i < slice.length; i++) {
       if (i > 0) {
           res += slice[i].toString(16).padStart(8, "0");
       } else {
           res += slice[i].toString(16);
       }
    }
    return res.padStart(15, "0");
}

function object_id_to_packed(id) {
    let packed = [0, 0, 0];
    if (id.length > 16) {
        packed[0] = parseInt(id.slice(0, -16), 16);
    }
    if (id.length > 8) {
        packed[1] = parseInt(id.slice(-16, -8), 16);
    }
    if (id.length > 0) {
        packed[2] = parseInt(id.slice(-8), 16);
    }
    return packed;
}
