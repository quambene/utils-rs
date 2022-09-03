use chrono::Utc;
use fern::colors::{Color, ColoredLevelConfig};
use log::info;

pub fn start() -> Result<(), anyhow::Error> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .info(Color::Green)
        .warn(Color::Yellow)
        .debug(Color::Blue)
        .trace(Color::Cyan);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
