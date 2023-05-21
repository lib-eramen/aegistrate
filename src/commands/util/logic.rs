use serenity::{
	client::Cache,
	model::prelude::{
		Guild,
		Member,
	},
};

pub fn highest_role_position(cache: &Cache, member: &Member, guild: &Guild) -> i64 {
	if let Some((_, position)) = member.highest_role_info(cache) {
		position
	} else if member.user.id == guild.owner_id {
		i64::MAX
	} else {
		0
	}
}

pub fn yes_no<T>(condition: bool, yes: T, no: T) -> T {
	if condition {
		yes
	} else {
		no
	}
}

pub fn yes_no_eval<F1, F2, R>(condition: bool, yes: F1, no: F2) -> R
where
	F1: FnOnce() -> R,
	F2: FnOnce() -> R, {
	if condition {
		yes()
	} else {
		no()
	}
}

#[must_use]
pub fn yes_no_str<'a>(condition: bool, yes: Option<&'a str>, no: Option<&'a str>) -> &'a str {
	yes_no(condition, yes.unwrap_or("Yes"), no.unwrap_or("No"))
}
