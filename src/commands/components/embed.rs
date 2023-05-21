//! Functions for creating embeds that are used for messages sent by Aegistrate.

#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]

use serenity::{
	builder::CreateEmbed,
	model::Timestamp,
	utils::Colour,
};

/// The kind of embed sent by Aegistrate.
#[derive(Clone, Copy)]
pub enum EmbedKind {
	/// A success dialog embed.
	Success,

	/// An information dialog embed.
	Info,

	/// A warning dialog embed.
	Warn,

	/// An error dialog.
	Error,
}

impl EmbedKind {
	/// Gets the color associated to the embed type.
	#[must_use]
	pub fn get_color(&self) -> Colour {
		match self {
			EmbedKind::Success => Colour::from(0x57_F2_87),
			EmbedKind::Info => Colour::from(0x58_65_F2),
			EmbedKind::Warn => Colour::from(0xFE_E7_5C),
			EmbedKind::Error => Colour::from(0xED_42_45),
		}
	}
}

/// The template for the embeds that Aegistrate send.
pub fn embed_template<S: ToString>(
	embed: &mut CreateEmbed,
	title: S,
	description: S,
	kind: EmbedKind,
) -> &mut CreateEmbed {
	embed
		.title(format!("**{}**", title.to_string()))
		.description(description)
		.color(kind.get_color())
		.timestamp(Timestamp::now())
}

/// Creates a success dialog embed.
pub fn create_success_embed<S: ToString>(
	embed: &mut CreateEmbed,
	title: S,
	description: S,
) -> &mut CreateEmbed {
	embed_template(embed, title, description, EmbedKind::Success)
}

/// Creates an information dialog embed.
pub fn create_info_embed<S: ToString>(
	embed: &mut CreateEmbed,
	title: S,
	description: S,
) -> &mut CreateEmbed {
	embed_template(embed, title, description, EmbedKind::Info)
}

/// Creates a warning dialog embed.
pub fn create_warning_embed<S: ToString>(
	embed: &mut CreateEmbed,
	title: S,
	warning: S,
) -> &mut CreateEmbed {
	embed_template(embed, title, warning, EmbedKind::Warn)
}

/// Creates an error dialog embed.
pub fn create_error_embed<S: ToString>(
	embed: &mut CreateEmbed,
	error: S,
	cause: S,
	hint: Option<S>,
) -> &mut CreateEmbed {
	let e = embed_template(
		embed,
		"Error!".to_string(),
		error.to_string(),
		EmbedKind::Error,
	)
	.field("Cause", cause, false);

	if let Some(hint) = hint {
		e.field("Hint", hint, false)
	} else {
		e
	}
	.footer(|footer| {
		footer.text(
			"Report issues here in our GitHub repo! (https://github.com/out-post/aegistrate/issues) ",
		)
	})
}
