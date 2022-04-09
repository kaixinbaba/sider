use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[clap(author, version, about, long_about = "A redis-client TUI project written by Rust")]
#[clap(propagate_version = true)]
struct Args {

    #[clap(short, long, default_value = "xjj")]
    name: String,
    
}
