use clap::Parser;

#[derive(Parser)]
pub(crate) struct Args {
    #[arg(long, default_value_t = 6)]
    pub(crate) length: usize,

    #[arg(short)]
    pub(crate) upper: bool,

    #[arg(short)]
    pub(crate) lower: bool,

    #[arg(short)]
    pub(crate) spec: bool,

    #[arg(long, default_value = "")]
    pub(crate) word: String
}
