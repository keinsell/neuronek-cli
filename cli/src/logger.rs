pub fn setup_logger(verbosity: Option<u8>)
{
    let mut base_config = fern::Dispatch::new();

    base_config = match verbosity
    {
        | Some(0) => base_config.level(log::LevelFilter::Info),
        | Some(1) => base_config.level(log::LevelFilter::Debug),
        | Some(2) => base_config.level(log::LevelFilter::Debug),
        | None => base_config.level(log::LevelFilter::Info),
        | _3_or_more => base_config.level(log::LevelFilter::Trace),
    };

    let _ = base_config.chain(std::io::stdout()).apply();
}
