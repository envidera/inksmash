use clap::Parser;

/// Command-line tool to compress Inkscape '.svg' files containing embedded images without a visual loss of quality. https://inkscape.org
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Inkscape .svg file
    pub file: String,

    /// range 1-100, where 1 is the worst quality and 100 is the best.
    #[arg(short, long, default_value_t = 90, value_parser = clap::value_parser!(u8).range(1..=100))]
    pub quality: u8,

    /// move completed files to the 'compressed' folder.
    #[arg(short, long)]
    pub move_completed: bool,

    /// extract compressed images to the 'extracted' folder.
    #[arg(short, long)]
    pub extract: bool,
}

pub fn parse() -> Args {
    Args::parse()
}
