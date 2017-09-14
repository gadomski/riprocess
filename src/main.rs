extern crate docopt;
extern crate riprocess;
#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;

const USAGE: &'static str = "
Query and/or generate material for RiPROCESS projects.

Usage:
    riprocess image-list <config>

Options:
    -h --help           Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_image_list: bool,
    arg_config: PathBuf,
}

fn main() {
    use docopt::Docopt;
    use riprocess::Config;

    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());

    if args.cmd_image_list {
        let config = Config::from_path(args.arg_config).unwrap();
        for image in config.image_list().unwrap() {
            println!("{:.6};{}", image.timestamp, image.path.display());
        }
    }
}
