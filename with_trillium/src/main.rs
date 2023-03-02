use trillium_async_std::{config, Stopper};
use trillium_conn_id::log_formatter::conn_id;

use signal_hook::consts::signal::*;
use signal_hook_async_std::Signals;
use trillium_logger::apache_common;
use trillium_logger::Logger;
use trillium_router::Router;

use async_std::prelude::*;
use async_std::task;

use std::io::Error;

use clap::{App, Arg};

mod handlers;
use handlers::reqlimit::Limiter;
use handlers::v1::FizzBuzz;

use crate::config::Config;

mod config;

async fn handle_signals(mut signals: Signals, server: Stopper) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => {
                // Reload configuration
                // Reopen the log file
                log::info!("received sighup");
            }
            SIGTERM | SIGINT | SIGQUIT => {
                // Shutdown the system;
                log::info!("sigterm|int|quit");
                server.stop();
            }
            _ => unreachable!(),
        }
    }
}

async fn server(cfg: config::Config) -> Result<(), Error> {
    let signals = Signals::new(&[SIGHUP, SIGTERM, SIGINT, SIGQUIT])?;
    let handle = signals.handle();
    let stopper = Stopper::new();

    let signals_task = async_std::task::spawn(handle_signals(signals, stopper.clone()));

    // Execute your main program logic
    let handler = FizzBuzz {};

    // per IP request limiter
    let router: Router;
    if let Some(limiter) = cfg.requests {
        let limiter = Limiter::new(limiter);
        router = Router::new().get("/v1/fb", (limiter, handler));
    } else {
        router = Router::new().get("/v1/fb", handler);
    }

    config()
        .with_host("localhost")
        .with_port(8080)
        .with_stopper(stopper)
        .with_max_connections(cfg.max_conn)
        .run_async((
            Logger::new().with_formatter(apache_common(conn_id, "-")),
            router,
        ))
        .await;

    // Terminate the signal stream.
    handle.close();
    signals_task.await;

    Ok(())
}

fn main() {
    env_logger::init();

    let matches = App::new("fizzbuzz_server_rs")
        .version("0.1.0")
        .about("Server that compute fizzbuzz lists")
        .arg(
            Arg::with_name("config")
                .long("cfg")
                .short("c")
                .help("Path to the configuration file")
                .takes_value(true),
        )
        .get_matches();

    let cfg = match matches.value_of("config") {
        Some(path) => config::read(path),
        None => Config::default(),
    };

    let task = task::spawn(server(cfg));
    log::info!("server start\n");
    task::block_on(task);
}
