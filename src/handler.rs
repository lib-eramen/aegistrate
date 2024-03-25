use serenity::all::EventHandler;

/// Directory to store all Aegistrate-related files.
pub const DIRECTORY: &str = "~/.aegistrate";

/// Handles events coming from Discord, using the `[EventHandler]` trait.
pub struct Handler;

impl EventHandler for Handler {}
