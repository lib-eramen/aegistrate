//! Contains an event handler struct that complies with [serenity]'s
//! [`EventHandler`] trait API, and also some helper function that controls
//! Aegistrate's state.

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
		prelude::{Ready, Activity},
		user::{CurrentUser, OnlineStatus},
	},
	prelude::{
		Context,
		EventHandler,
	},
};
use tokio::sync::OnceCell;

/// Spawns a timeout checker that exits the program if [`DISCORD_READY`] is not
/// set to `true` after [`READY_UP_TIME`].
pub fn spawn_timeout_checker() {
	thread::spawn(|| {
		thread::sleep(READY_UP_TIME);
		if !DISCORD_READY.load(Ordering::Relaxed) {
			error!("Services not ready for {READY_UP_TIME:#?}");
			std::process::exit(1);
		}
	});
}

/// Unit struct that implements [`EventHandler`]. Is Aegistrate's core Discord
/// event handler.
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, context: Context, bot_data: Ready) {
		Self::discord_ready_up(&context, &bot_data).await;
		info!(
			"Aegistrate is up and running!\n{:#?}",
			get_aegistrate_user()
		);
	}

	// async fn _ready(&self, context: Context, bot_data: Ready) {
	// 	fire_up_database().await.unwrap();
	// 	if !DISCORD_READY.load(Relaxed) {
	// 		Self::discord_ready_up(&context, &bot_data).await;
	// 		Self::initialize_systems(&context, &bot_data)
	// 			.await
	// 			.unwrap_or_else(|why| error!("Initializing system failed: {why}"));
	// 	}
	// 	Self::appear_active(&context, &bot_data).await;
	// }

	// async fn interaction_create(&self, context: Context, interaction: Interaction) {
	// 	match interaction {
	// 		Interaction::ApplicationCommand(ref app_interaction) => {
	// 			Self::run_application_command(&context, app_interaction).await;
	// 		},
	// 		_ => return,
	// 	};
	// }
}

impl Handler {
	/// Readies up the Discord service portion of Aegistrate.
	/// 
	/// # Panics
	/// 
	/// This function may panic if [`AEGISTRATE_USER`] is not successfully set to
	/// the provided user.
	pub async fn discord_ready_up(context: &Context, bot_data: &Ready) {
		AEGISTRATE_USER.set(bot_data.user.clone()).unwrap();
		DISCORD_READY.store(true, Ordering::Relaxed);
		context
			.set_presence(
				Some(Activity::playing("the waiting game...")),
				OnlineStatus::DoNotDisturb,
			)
			.await;
	}
}

/// Gets the Aegistrate user that is under a layer of [`OnceCell`].
///
/// # Panics
///
/// This function panics if the [`AEGISTRATE_USER`] static variable somehow
/// hasn't been initialized.
pub fn get_aegistrate_user<'a>() -> &'a CurrentUser {
	AEGISTRATE_USER.get().unwrap()
}

/// Controls whether the Discord service portion is ready to go.
/// Take a look at [`spawn_timeout_checker`] to see how this variable is used.
pub static DISCORD_READY: AtomicBool = AtomicBool::new(false);

/// The time reserved for Aegistrate to spin up everything.
/// Take a look at [`spawn_timeout_checker`] to see how this variable is used.
pub static READY_UP_TIME: Duration = Duration::new(10, 0);

/// The bot user that Aegistrate assumes identity of.
/// To access with knowledge that it has been initialized, use
/// [`get_aegistrate_user`].
pub static AEGISTRATE_USER: OnceCell<CurrentUser> = OnceCell::const_new();
