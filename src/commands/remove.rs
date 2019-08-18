use clap::ArgMatches;

use crate::config::Config;

pub fn remove(args: &ArgMatches) -> Result<(), failure::Error> {
    let mut cfg = Config::load()?;
    let name = args.value_of("name").unwrap();
    
    cfg.remove(&name)?;
    cfg.save()?;

    return Ok(());
}
