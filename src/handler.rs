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
	model::{
		prelude::Ready,
		user::CurrentUser,
	},
	prelude::{
		Context,
		EventHandler,
	},
};
use tokio::sync::OnceCell;

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
	async fn ready(&self, _context: Context, bot_data: Ready) {
		DISCORD_READY.store(true, Ordering::Relaxed);
		AEGISTRATE_USER.set(bot_data.user).unwrap();
		info!(
			"Aegistrate is up and running!\n{:#?}",
			get_aegistrate_user()
		);
	}
}

pub fn get_aegistrate_user<'a>() -> &'a CurrentUser {
	AEGISTRATE_USER.get().unwrap()
}

static DISCORD_READY: AtomicBool = AtomicBool::new(false);

static READY_UP_TIME: Duration = Duration::new(10, 0);

static AEGISTRATE_USER: OnceCell<CurrentUser> = OnceCell::const_new();
