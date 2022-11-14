use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    help_template = "\
    {before-help}{name} {version} {author-with-newline}{about-with-newline} \
    {usage-heading} {usage} {all-args}{after-help}"
)]
struct Args {
    /// The value you'd like to convert to an MD5 hash
    #[arg(short, long)]
    value: String,
}

fn main() {
    let args = Args::parse();

    let digest = md5::compute(args.value.clone());
    println!("md5({:?}) = {:?}", args.value, digest);
}
