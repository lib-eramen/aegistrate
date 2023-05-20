pub type Aegis<T> = anyhow::Result<T>;

pub fn aegisize<T, E, M, R>(result: Result<T, E>, mapper: M) -> Aegis<R>
where
	E: std::error::Error + Send + Sync + 'static,
	M: FnOnce(T) -> R, {
	result.map(mapper).map_err(anyhow::Error::new)
}

pub fn aegisize_unit<T, E>(result: Result<T, E>) -> Aegis<()>
where
	E: std::error::Error + Send + Sync + 'static, {
	aegisize(result, |_| ())
}

pub fn aegisize_preserve<T, E>(result: Result<T, E>) -> Aegis<T>
where
	E: std::error::Error + Send + Sync + 'static, {
	aegisize(result, |ok| ok)
}
