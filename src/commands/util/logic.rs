//! Contains some utility functions that perform some kind of logic for a
//! result. Sorry, I ran out of vocabulary to name this module, and "query" just
//! didn't sound right.

/// A ternary operator ditto.
#[must_use]
pub fn yes_no<T>(condition: bool, yes: T, no: T) -> T {
	if condition {
		yes
	} else {
		no
	}
}

/// A ternary operator ditto, but with closures! Woohoo! (I hate these kinds of
/// doc comments)
#[must_use]
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

/// Ternary operator ditto for strings, with the `true` argument defaulting to
/// `"Yes"` and the `false` argument defaulting to `"No"`.
#[must_use]
pub fn yes_no_str<'a>(condition: bool, yes: Option<&'a str>, no: Option<&'a str>) -> &'a str {
	yes_no(condition, yes.unwrap_or("Yes"), no.unwrap_or("No"))
}
