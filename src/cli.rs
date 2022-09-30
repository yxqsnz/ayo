use clap::Parser;

#[derive(Parser)]
#[command(name = "ayo")]
#[command(about = "A another nicer daemon.", long_about = None)]
pub(crate) struct Ayo {
    #[clap(short, long, default_value = "/etc/ayo.toml")]
    pub(crate) config_path: String,

    #[clap(short, long, default_value = "/etc/ayo.d")]
    pub(crate) rules_path: String,
}
