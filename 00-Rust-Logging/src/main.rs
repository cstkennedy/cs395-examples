use std::time::SystemTime;
use std::{thread, time};

use eyre::WrapErr;

use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, error, info, trace, warn};

fn set_up_logging(global_level: log::LevelFilter) -> Result<(), fern::InitError>{
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);

    let colors_level = colors_line.info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date} {color_line}{level:<5} {target}]{color_line} {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = humantime::format_rfc3339_seconds(SystemTime::now()),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        .level(global_level)
        .chain(std::io::stderr())
        .apply()?;

    debug!("finished setting up logging! yay!");
    Ok(())
}

fn main() -> eyre::Result<()>{
    set_up_logging(log::LevelFilter::Warn).wrap_err("Logging set up failed")?;

    info!("starting simulation!");
    for i in 0..26 {
        trace!("loading: {}%, very verbose debbuging information", 4 * i);

        match i {
            5 => {
                debug!("this is taking so long... boooring!");
            }
            10 => {
                debug!("still alive! yay!");
            }
            13 => {
                info!("halfway there!");
            }
            16 => {
                debug!("*scratches nose*");
                warn!("nose is itching, continuing anyways");
            }
            20 => {
                debug!("uh oh");
                warn!(">nose itching intensifies");
                error!("HATCHOOO!");
                debug!("encountered minor problem, trying to recover");
                info!("gesundheit");
                debug!("recovered from minor problem, continuing");
            }
            25 => {
                info!("successfully loaded nothing");
                info!("have a good time!");
            }
            _ => {}
        }

        thread::sleep(time::Duration::from_millis(250));
    }

    Ok(())
}


