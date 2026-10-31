use env_logger::Env;

pub fn init_logger() -> Result<(), log::SetLoggerError> {
    let env = Env::default().filter_or("LOG_LEvEL", "info");

    env_logger::Builder::from_env(env).try_init()
}
