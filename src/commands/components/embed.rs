#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]

use serenity::{
	builder::CreateEmbed,
	model::Timestamp,
	utils::Colour,
};

pub enum ResponseEmbedType {
	Success,
	Info,
	Warn,
	Error,
}

#[must_use]
pub fn get_embed_color(kind: &ResponseEmbedType) -> Colour {
	match kind {
		ResponseEmbedType::Success => Colour::from(0x57_F2_87),
		ResponseEmbedType::Info => Colour::from(0x58_65_F2),
		ResponseEmbedType::Warn => Colour::from(0xFE_E7_5C),
		ResponseEmbedType::Error => Colour::from(0xED_42_45),
	}
}

pub fn embed_template<S: ToString>(
	embed: &mut CreateEmbed,
	title: S,
	description: S,
	color: Colour,
) -> &mut CreateEmbed {
	embed
		.title(format!("**{}**", title.to_string()))
		.description(description)
		.color(color)
		.timestamp(Timestamp::now())
}

pub fn create_success_embed<S: ToString>(
	embed: &mut CreateEmbed,
	title: S,
	description: S,
) -> &mut CreateEmbed {
	embed_template(
		embed,
		title,
		description,
		get_embed_color(&ResponseEmbedType::Success),
	)
}

pub fn create_info_embed<S: ToString>(
	embed: &mut CreateEmbed,
	title: S,
	description: S,
) -> &mut CreateEmbed {
	embed_template(
		embed,
		title,
		description,
		get_embed_color(&ResponseEmbedType::Info),
	)
}

pub fn create_warning_embed<S: ToString>(
	embed: &mut CreateEmbed,
	title: S,
	warning: S,
) -> &mut CreateEmbed {
	embed_template(
		embed,
		title,
		warning,
		get_embed_color(&ResponseEmbedType::Warn),
	)
}

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
		get_embed_color(&ResponseEmbedType::Error),
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
