//! Contains an event handler struct that complies with [serenity]'s
//! [`EventHandler`] trait API, and also some helper function that controls
//! Aegistrate's state.

use std::{
	env::var,
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
use pluralizer::pluralize;
use serenity::{
	client::Cache,
	http::{
		CacheHttp,
		Http,
	},
	model::{
		prelude::{
			interaction::{
				application_command::ApplicationCommandInteraction,
				Interaction,
			},
			Activity,
			GuildId,
			Ready,
			UnavailableGuild,
		},
		user::{
			CurrentUser,
			OnlineStatus,
		},
	},
	prelude::{
		Context,
		EventHandler,
	},
};
use tokio::{
	sync::OnceCell,
	time::sleep,
};

use crate::{
	aegis::Aegis,
	commands::{
		components::embed::create_error_embed,
		util::message::{
			respond_with_embed,
			ResponseOptions,
		},
	},
	core::command::{
		all_commands,
		command_by_name,
		Command,
	},
};

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
		set_up_database().await.unwrap_or_else(|why| {
			panic!("Database failed to set up: {why}");
		});
		if !DISCORD_READY.load(Ordering::Relaxed) {
			Self::discord_ready_up(&context, &bot_data).await;
			Self::initialize_systems(&context, &bot_data)
				.await
				.unwrap_or_else(|why| error!("Initializing system failed: {why}"));
		}
		Self::appear_active(&context, &bot_data).await;
	}

	async fn interaction_create(&self, context: Context, interaction: Interaction) {
		match interaction {
			Interaction::ApplicationCommand(ref app_interaction) => {
				Self::run_application_command(&context, app_interaction).await;
			},
			_ => return,
		};
	}
}

impl Handler {
	/// Readies up the Discord service portion of Aegistrate.
	///
	/// # Panics
	///
	/// This function may panic if [`AEGISTRATE_USER`] is not successfully set
	/// to the provided user.
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

	/// Appear active to both user and developer ends.
	async fn appear_active(context: &Context, bot_data: &Ready) {
		context
			.set_presence(
				Some(Activity::watching("over the server")),
				OnlineStatus::Online,
			)
			.await;
		READY_TO_GO.store(true, Ordering::Relaxed);
		info!(
			"{} has reached the Out-Post!\nUser: {:#?}",
			bot_data.user.tag(),
			get_aegistrate_user()
		);
	}

	/// Registers a command to a guild.
	async fn register_command(
		http: &Http,
		cache: &Cache,
		guild: GuildId,
		command: Box<dyn Command>,
	) -> Aegis<()> {
		for name in command.metadata().all_names() {
			guild
				.create_application_command(http, |endpoint| command.register(endpoint).name(name))
				.await?;
		}
		info!(
			"Guild \"{}\" ({}) registered /{}",
			guild.name(cache).unwrap_or_else(|| "<null>".to_string()),
			guild.0,
			command.metadata().name
		);
		Ok(())
	}

	/// Registers multiple commands to a guild.
	#[allow(clippy::cast_possible_wrap)]
	async fn register_commands(
		http: &Http,
		cache: &Cache,
		guild: GuildId,
		commands: Vec<Box<dyn Command>>,
	) -> Aegis<()> {
		let commands_count = commands.len();
		for command in commands {
			Self::register_command(http, cache, guild, command).await?;
			sleep(Duration::from_secs_f32(REGISTER_COMMAND_INTERVAL)).await;
		}
		info!(
			"Guild \"{}\" ({}) registered {}",
			guild.name(cache).unwrap_or_else(|| "<null>".to_string()),
			guild.0,
			pluralize("command", commands_count as isize, true),
		);
		Ok(())
	}

	/// Handles command registration for a guild, using the commands from the
	/// guild's enabled plugins.
	async fn set_up_commands(context: &Context, guild: &UnavailableGuild) -> Aegis<()> {
		let guild_commands = all_commands(); // TODO: Plugin system implementation
		Self::register_commands(
			context.http(),
			context.cache().unwrap(),
			guild.id,
			guild_commands,
		)
		.await
	}

	/// Initializes all systems for Aegistrate.
	async fn initialize_systems(context: &Context, bot_data: &Ready) -> Aegis<()> {
		for guild in &bot_data.guilds {
			Self::set_up_commands(context, guild).await?;
		}
		Ok(())
	}

	/// Responds to the interaction with a non-ready message.
	async fn non_ready_respond(
		http: &Http,
		app_interaction: &ApplicationCommandInteraction,
	) -> Aegis<()> {
		respond_with_embed(
			http,
			app_interaction,
			ResponseOptions::CreateOrignial(true),
			|embed| {
				create_error_embed(
					embed,
					"Rude! I'm not even done getting ready!",
					"Aegistrate hasn't finished all of its ready-up procedures. Coffee takes time \
					 to brew, y'know!",
					Some(
						"Give Aegistrate a bit of time - you should see its status turn from Do \
						 Not Disturb to Online soon!",
					),
				)
			},
		)
		.await
		.map(|_| ())
	}

	/// Responds to the interaction with an unknown-command message.
	async fn unknown_command_respond(
		http: &Http,
		app_interaction: &ApplicationCommandInteraction,
		command_name: &str,
	) -> Aegis<()> {
		respond_with_embed(
			http,
			app_interaction,
			ResponseOptions::CreateOrignial(true),
			|embed| {
				create_error_embed(
					embed,
					format!(
						"Sorry champ, can't help you out there. I dunno what a \
						 \"/{command_name}\" is."
					),
					format!("Failed to get a command with the name \"/{command_name}\"."),
					None,
				)
			},
		)
		.await
		.map(|_| ())
	}

	/// Handles a command execution for Aegistrate.
	async fn handle_command_execution(
		command: Box<dyn Command>,
		context: &Context,
		app_interaction: &ApplicationCommandInteraction,
	) {
		let command_id = app_interaction.id;
		let command_name = command.metadata().name;
		match command.execute(context, app_interaction).await {
			Ok(_) => {
				info!("Interaction {command_id}: Command /{command_name} executed successfully");
			},
			Err(why) => {
				error!(
					"Interaction {command_id}: Command /{command_name} executed with an error: \
					 {why}"
				);
			},
		}
	}

	/// Runs an application command for Aegistrate.
	async fn run_application_command(
		context: &Context,
		app_interaction: &ApplicationCommandInteraction,
	) {
		let http = context.http();
		if !READY_TO_GO.load(Ordering::Relaxed) {
			let _ = Self::non_ready_respond(http, app_interaction).await;
			return;
		}

		let command_name = &app_interaction.data.name;
		let Some(command) = command_by_name(command_name) else {
			let _ = Self::unknown_command_respond(http, app_interaction, command_name).await;
			return;
		};

		// TODO: Cooldown system comes in play here
		Self::handle_command_execution(command, context, app_interaction).await;
	}
}

/// Sets up the database service portion for Aegistrate.
#[allow(clippy::missing_errors_doc)]
pub async fn set_up_database() -> Aegis<()> {
	let _ = MONGODB_CLIENT.set(mongod::Client::from_client(
		mongodb::Client::with_uri_str(var("MONGODB_URI")?).await?,
		"development",
	));
	Ok(())
}

/// Gets the MongoDB client that is under a layer of [`OnceCell`].
///
/// # Panics
///
/// This function panics if the [`MONGODB_CLIENT`] static variable
/// hasn't been initialized.
pub fn get_mongodb_client<'a>() -> &'a mongod::Client {
	MONGODB_CLIENT.get().unwrap()
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

/// Controls whether Aegistrate is up and running.
pub static READY_TO_GO: AtomicBool = AtomicBool::new(false);

/// The interval to sleep for between each command registration.
pub static REGISTER_COMMAND_INTERVAL: f32 = 1.0;

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

/// The MongoDB client that will interact with Aegistrate's MongoDB database.
pub static MONGODB_CLIENT: OnceCell<mongod::Client> = OnceCell::const_new();
