use clap::Parser;

use crate::SDResult;


#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = "A redis-client TUI project written by Rust")]
#[clap(propagate_version = true)]
struct Args {

    #[clap(short = 'c', long = "config-path", default_value = ".sider", help = "The sider's config root path, default is .sider at $HOME")]
    config_path: String,
}


#[derive(Default, Clone, Debug)]
pub struct Config {

    config_path: String,
}




pub fn parse_cli() -> SDResult<Config> {


    let args = Args::parse();
    println!("{:?}", args);

    Ok(Config::default())

}


