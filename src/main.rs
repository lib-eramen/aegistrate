//! Your Discord guild's aegis and magistrate! Yes, some people get that
//! portmanteau. This project is way too optimized for everyone's use, but
//! please feel free to remix, remove, add, change, build new parts, etc. for
//! your own Aegistrate.

mod handler;
mod log;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	log4rs::init_file("log4rs.yaml", Default::default())?;
	Ok(())
}
