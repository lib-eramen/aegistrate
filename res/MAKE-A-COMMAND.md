# `Command` implementation in Aegistrate

`Command`s require quite a lot of planning and execution in Aegistrate.
I (`@developer-ramen`) blame myself for the system that I have put in place.

Here are a few instructions to add commands so that it actually makes the destination, that is the Slash Command registraton Discord API endpoint:

1. Fill all of the metadata points. What is the name, the plugin it belongs to, and what cooldown should it have?
2. If the command requires a new plugin, add the plugin to these places first:
    - The `Plugin` enum. (`src/core/plugin.rs`)
    - Clippy/`cargo check` should tell you all the places to add the new enum variant to.
    - Make a new command plugin submodule. (`src/commands/plugins/mod.rs`).
    - Always expose a function that goes `<plugin_name>_commands()` from that command plugin module.
    - Don't have a plugin exactly? Goes in the `Miscellaneous` module!
3. Add a file to the submodule, and start implementing!