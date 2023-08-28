use clap::Parser;
use log::LevelFilter;

#[derive(Debug, Clone, Parser, Eq, PartialEq)]
#[clap(name = "AdminClient", version, author = "Phyrone <phyrone@phyrone.de>")]
pub struct StartupParams {
    #[clap(short, long, default_value = "info")]
    pub log_level: LevelFilter,
}

pub(crate) fn init_logger(level: LevelFilter) {
    let logger_config = fast_log::Config::new().level(level).console();
    fast_log::init(logger_config).expect("logger init error");
}
