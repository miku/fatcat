#![allow(missing_docs)]

#[macro_use]
extern crate slog;
#[macro_use]
extern crate hyper;

use cadence::prelude::*;
use cadence::{BufferedUdpMetricSink, QueuingMetricSink, StatsdClient};
use clap::{App, Arg};
use fatcat::errors::Result;
use fatcat::server;
use iron::middleware::AfterMiddleware;
use iron::modifiers::RedirectRaw;
use iron::{status, Chain, Iron, IronResult, Request, Response};
use iron_slog::{DefaultLogFormatter, LoggerMiddleware};
use sentry::integrations::panic;
use slog::{Drain, Logger};
use std::env;
use std::net::UdpSocket;

// HTTP header middleware
header! { (XClacksOverhead, "X-Clacks-Overhead") => [String] }

pub struct XClacksOverheadMiddleware;

impl AfterMiddleware for XClacksOverheadMiddleware {
    fn after(&self, _req: &mut Request, mut res: Response) -> iron::IronResult<Response> {
        res.headers
            .set(XClacksOverhead("GNU aaronsw, jpb".to_owned()));
        Ok(res)
    }
}

/// Create custom server, wire it to the autogenerated router,
/// and pass it to the web server.
fn main() -> Result<()> {
    let matches = App::new("server")
        .arg(
            Arg::with_name("https")
                .long("https")
                .help("Whether to use HTTPS or not"),
        )
        .get_matches();

    dotenv::dotenv().ok();

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(drain, o!());
    let _scope_guard = slog_scope::set_global_logger(logger.clone());
    let _log_guard = slog_stdlog::init().unwrap();
    let formatter = DefaultLogFormatter;

    // sentry exception handling
    let sentry_dsn = env::var("SENTRY_DSN");
    let _guard = if let Ok(dsn) = sentry_dsn {
        let client = sentry::init(dsn);
        panic::register_panic_handler();
        info!(logger, "Sentry configured via DSN");
        Some(client)
    } else {
        info!(logger, "Sentry not configured");
        None
    };

    let mut server = server::create_server()?;

    // metrics reporting
    match env::var("FATCAT_STATSD_HOST") {
        Err(_) => {
            info!(logger, "no metrics recipient configured");
        }
        Ok(host) => {
            let port: u16 = match env::var("FATCAT_STATSD_PORT") {
                Err(_) => cadence::DEFAULT_PORT,
                Ok(var) => var.parse::<u16>()?, // "expect FATCAT_STATSD_PORT to be null or an integer
            };
            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
            socket.set_nonblocking(true).unwrap();
            let udp_sink = BufferedUdpMetricSink::from((host.as_ref(), port), socket).unwrap();
            let queuing_sink = QueuingMetricSink::from(udp_sink);
            info!(
                logger,
                "sending statsd metrics via UDP to: {}:{}", host, port
            );
            server.metrics = StatsdClient::from_sink("fatcat.api", queuing_sink);
            //server.metrics = StatsdClient::from_udp_host("fatcat.api", (host.as_ref(), port))?;
            server.metrics.incr("restart").unwrap();
        }
    };

    info!(
        logger,
        "using primary auth key: {}", server.auth_confectionary.identifier,
    );
    info!(
        logger,
        "all auth keys: {:?}",
        server
            .auth_confectionary
            .root_keys
            .keys()
            .collect::<Vec<&String>>(),
    );
    let mut router = fatcat_openapi::router(server);

    router.get("/", root_handler, "root-redirect");
    router.get("/redoc", redoc_handler, "redoc-html");
    router.get("/swagger-ui", swaggerui_handler, "swagger-ui-html");
    router.get("/v0/openapi2.yml", yaml_handler, "openapi2-spec-yaml");

    fn root_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((
            status::Found,
            RedirectRaw("/redoc".to_string()),
        )))
    }
    fn redoc_handler(_: &mut Request) -> IronResult<Response> {
        let html_type = "text/html".parse::<iron::mime::Mime>().unwrap();
        Ok(Response::with((
            html_type,
            status::Ok,
            include_str!("../../redoc/index.html"),
        )))
    }
    fn swaggerui_handler(_: &mut Request) -> IronResult<Response> {
        let html_type = "text/html".parse::<iron::mime::Mime>().unwrap();
        Ok(Response::with((
            html_type,
            status::Ok,
            include_str!("../../swagger-ui/index.html"),
        )))
    }
    fn yaml_handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((
            status::Ok,
            include_str!("../../../fatcat-openapi2.yml"),
        )))
    }

    let host_port = "localhost:9411";
    info!(
        logger,
        "Starting fatcatd API server on http://{}", &host_port
    );

    let mut chain = Chain::new(LoggerMiddleware::new(router, logger, formatter));

    // authentication
    chain.link_before(fatcat_openapi::server::ExtractAuthData);
    chain.link_before(fatcat::auth::MacaroonAuthMiddleware::new());

    chain.link_after(XClacksOverheadMiddleware);

    if matches.is_present("https") {
        unimplemented!()
    } else {
        // Using HTTP
        Iron::new(chain)
            .http(host_port)
            .expect("failed to start HTTP server");
    }
    Ok(())
}
