//! Contains all commands, organized into plugins that each have a corresponding
//! submodule for it.

#![allow(clippy::module_inception)]

pub mod information;
pub mod moderation;
pub mod plugins;

/// Appends a piece of help text to the end of a string on how to use duration
/// options.
#[must_use]
pub fn append_duration_helptext(string: &str) -> String {
	format!(
		"{string} For example: 1d, 2h, 3m, 4s, 1h + 30m, etc. Spaces in between the duration and \
		 the unit are optional. Duration must not be more than 28 days from now."
	)
}

/// Appends a piece of help text to the end of a string on how to use duration
/// options.
#[must_use]
pub fn append_date_helptext(string: &str) -> String {
	format!(
		"{string} Format: YYYY-MM-DD. For example: 1234-56-78, 1970-01-01, etc. Date must be \
		 valid."
	)
}
