//! Contains the core result type used throughout Aegistrate, the [Aegis] type.

#![allow(clippy::missing_errors_doc)]

/// The core result type used throughout Aegistrate. It is just a name alias
/// from [`anyhow::Result`].
pub type Aegis<T> = anyhow::Result<T>;

/// Converts an arbitrary result into the [Aegis] type.
pub fn aegisize<T, E, M, R>(result: Result<T, E>, mapper: M) -> Aegis<R>
where
	E: std::error::Error + Send + Sync + 'static,
	M: FnOnce(T) -> R, {
	result.map(mapper).map_err(anyhow::Error::new)
}

/// Converts an arbitrary result into the [Aegis] type, flushing whatever result
/// was in the parameter down the toilet operator.
pub fn aegisize_unit<T, E>(result: Result<T, E>) -> Aegis<()>
where
	E: std::error::Error + Send + Sync + 'static, {
	aegisize(result, |_| ())
}

/// Converts an arbitrary result into the [Aegis] type, preserving the [Ok]
/// variant's data.
pub fn aegisize_preserve<T, E>(result: Result<T, E>) -> Aegis<T>
where
	E: std::error::Error + Send + Sync + 'static, {
	aegisize(result, |ok| ok)
}
