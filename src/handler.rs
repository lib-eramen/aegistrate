use std::{
	sync::atomic::{
		AtomicBool,
		Ordering,
	},
	thread,
	time::Duration,
};

use async_trait::async_trait;
use log::{
	error,
	info,
};
use serenity::{
	model::prelude::Ready,
	prelude::{
		Context,
		EventHandler,
	},
};

pub fn spawn_timeout_checker() {
	thread::spawn(|| {
		thread::sleep(READY_UP_TIME);
		if !DISCORD_READY.load(Ordering::Relaxed) {
			error!("Services not ready for {READY_UP_TIME:#?}");
			std::process::exit(1);
		}
	});
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _context: Context, _bot_data: Ready) {
		DISCORD_READY.store(true, Ordering::Relaxed);
		info!("Aegistrate is up and running!");
	}
}

pub static DISCORD_READY: AtomicBool = AtomicBool::new(false);

pub static READY_UP_TIME: Duration = Duration::new(10, 0);
