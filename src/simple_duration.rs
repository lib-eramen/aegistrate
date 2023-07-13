//! Contains a type of time duration representer that is used over other time
//! representations in Aegistrate operations.

//! A simpler, dumbed down version of [`std::time::Duration`] with types of time
//! that are more appropriate for actions performed within Aegistrate.
//!
//! Note that one should enforce some form of restriction/number range on what
//! the user can put in here, as the type used to contain the time number is
//! [`u64`].

use std::fmt::Display;
#[allow(missing_docs)]
#[must_use]
#[derive(Clone, Copy)]
pub enum SimpleDuration {
	Seconds(u64),
	Minutes(u64),
	Hours(u64),
	Days(u64),
	Weeks(u64),
	Months(u64),
}

impl Display for SimpleDuration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Seconds(n) => write!(f, "{n} seconds"),
			Self::Minutes(n) => write!(f, "{n} minutes"),
			Self::Hours(n) => write!(f, "{n} hours"),
			Self::Days(n) => write!(f, "{n} days"),
			Self::Weeks(n) => write!(f, "{n} weeks"),
			Self::Months(n) => write!(f, "{n} months"),
		}
	}
}

impl SimpleDuration {
	/// Converts the duration to a human-friendly format.
	#[must_use]
	pub fn human_fmt(self) -> String {
		match self {
			Self::Seconds(n) => format!("{n} seconds"),
			Self::Minutes(n) => format!("{n} minutes"),
			Self::Hours(n) => format!("{n} hours"),
			Self::Days(n) => format!("{n} days"),
			Self::Weeks(n) => format!("{n} weeks"),
			Self::Months(n) => format!("{n} months"),
		}
	}
}
