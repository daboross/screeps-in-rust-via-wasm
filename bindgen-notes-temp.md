# Project setup

## Generate wasm-pack template

```
# stuff mine needed, your mileage may vary
export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu/
export OPENSSL_INCLUDE_DIR=/usr/include/openssl

cargo install cargo-generate

cargo generate --git https://github.com/rustwasm/wasm-pack-template
```

## Add some bits to the bot!

Add the library dependency to the `Cargo.toml` as well as web-sys which is used for console logging hooks:

```
screeps-game-api = { path = "../screeps-game-api" }
web-sys = { version = "0.3", features = ["console"] }
```

and some lib.rs contents:


```
use std::fmt;
use wasm_bindgen::prelude::*;

use screeps::{Game, Room, RoomName};
use web_sys::console;

#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    console::log_1(&JsString::from(format!("Hello from bindgen! {}", Game::time())));

    for room in Object::values(&Game::rooms()).iter().map(Room::from) {
        console::log_1(&JsString::from(format!("{} {}", String::from(room.name()), room.energy_available())));
    }
}
```

If this a bot which will be deployed by cargo-screeps (instead of an external builder depending
on this module like rollup), create the `javascript` directory (this name is in `include_files`
by default, which will deploy all of its js/wasm files), which will hold the loader for your wasm
binary.

This example `main.js` will safely load your wasm binary then replace the loop function:

```
"use strict";

Error.stackTraceLimit = Infinity;

function wasm_initialize() {
    // attempt to load the wasm only if there's enough bucket to do a bunch of work this tick
    if (Game.cpu.bucket < 500) {
        console.log("we are running out of time, pausing compile!" + JSON.stringify(Game.cpu));
        return;
    }
    
    // replace this initialize function on the module
    const loop =  require("my_crate").loop;
    // replace the export of this function with the module's
    module.exports.loop = loop;
    // go ahead and run the loop for its first tick
    loop()
}

module.exports.loop = wasm_initialize;

```

# Build

## Switch to bindgen branch tools

- Check out the bindgen branch in your screeps-game-api local copy referenced from Screeps.toml
- Check out cargo-screeps on the bindgen branch and install it with `cargo install --path .`

## Configure a screeps.toml in your wasm-pack template

..based on the screeps-default.toml in the cargo-screeps bindgen branch

### Deploy a full bot

`cargo screeps copy`/`cargo screeps upload`/`cargo screeps deploy -m deploy_mode`

### (or build a module for use by js/ts)

`cargo screeps build` (deploy options aren't required in the screeps.toml)

The npm module and its package.json and ts definitions will be compiled to `pkg`; depend on them from outside!
