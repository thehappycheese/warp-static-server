use clap::Parser;
use warp::Filter;
use std::{error::Error, net::IpAddr, path::Path};


use log::{Record, Level, Metadata, LevelFilter};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}


#[derive(Parser, Debug)]
#[command(version)]
#[command(author = "Nick Archer")]
#[command(about = "Serves Static Files, Fast and Easy", long_about = None)]
struct Args {
    
    /// The path containing website files to serve
    #[arg(long)]
    path: Option<String>,

    /// The port on which to serve the files.
    #[arg(long, default_value_t = 80)]
    port: u16,

    /// If present, serve at http://0.0.0.0:<port> Otherwise serve at http://127.0.0.1:<port>
    #[arg(long, default_value_t = false)]
    allowlocal: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!(
r#"
                                _        _   _
__      ____ _ _ __ _ __    ___| |_ __ _| |_(_) ___ 
\ \ /\ / / _` | '__| '_ \  / __| __/ _` | __| |/ __|
 \ V  V / (_| | |  | |_) | \__ \ || (_| | |_| | (__ 
  \_/\_/ \__,_|_|  | .__/  |___/\__\__,_|\__|_|\___|
  ___  ___ _ ____  |_|__ _ __
/ __|/ _ \ '__\ \ / / _ \ '__|
\__ \  __/ |   \ V /  __/ |
|___/\___|_|    \_/ \___|_|

By Nick Archer :)
"#
    );

    match log::set_logger(&CONSOLE_LOGGER){
        Ok(_)=>log::set_max_level(LevelFilter::Info),
        Err(_)=>println!("Logger did not start. Will continue trying to run the server anyway.")
    }
    
    let args = Args::parse();
    let ip_addr: IpAddr = if args.allowlocal {
        [0, 0, 0, 0].into()
    } else {
        [127, 0, 0, 1].into()
    };
    let path = match args.path {
        Some(user_path) => match Path::new(user_path.as_str().into()).is_dir() {
            true => user_path,
            false => {
                return Err(format!("The path provided does not exist `--path {user_path}`. Use --help for more info.").into());
            }
        },
        None => match Path::new("./static/").is_dir() {
            true => "./static/".into(),
            false => {
                return Err("The default path `./static/` is not a folder or does not exist. Please either move your website files into a folder called `./static/` or use powershell /command prompt provide a path like `warp-static-server.exe --path \"some path/to website/files/\"`. Use --help for more info.".into());
            }
        },
    };

    println!("Attempting to start server.");
    let server = warp::serve(
        warp::fs::dir(path.clone()).with(warp::log("output"))
    ).run((ip_addr, args.port));
    println!(
        "Server started at http://{}:{} serving folder `{}`",
        ip_addr, args.port, path
    );
    server.await;
    Ok(())
}
