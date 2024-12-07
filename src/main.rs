mod app;
mod codes;
mod config;
mod domain;
mod infrastructure;

use std::fs::File;
use simplelog::*;

fn init_logger() {
    /*
     Error = 1,
     Warning = 2,
     Info = 3,
     Debug = 4,
     Trace = 5,
    */
    simplelog::CombinedLogger::init(vec![
        // 標準出力にはDebug以上を表示する
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Debug,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // ファイル server.logにはinfo以上を表示する
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create("log/server.log").unwrap(),
        ),
    ])
    .unwrap();
}

fn main() -> std::io::Result<()> {
    init_logger();
    // infrastructure::actix::router::run()
    todo!()
}
