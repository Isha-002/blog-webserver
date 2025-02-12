use clap::{command, value_parser, Arg, ArgMatches};

pub fn arguments() -> ArgMatches {
    command!().about("This is a web server for managing blog posts, text, and comments.")
  // you can pass arguments like this:
  .arg(
      // -d or --db-url
      Arg::new("database url")
          .short('d')
          .long("db-url")
          .aliases(["db", "url", "database", "psql", "dburl", "db_url"])
          .help("a url that connects your postgres database to the server")
  )
  .arg(
      // -o or --open-port
      Arg::new("set origin")
          .short('o')
          .long("open-port")
          .aliases(["origin", "open", "openport"])
          .help("this argument exposes one port for the frontend to access the server (Default: 4446)")
          .value_parser(value_parser!(u16))
  )
  .arg(
      // --log
      Arg::new("log level").long("log")
      .help("assign the log level (Default: info) - options(debug, info, warn, error)")
  )
  .arg(
      // -p or --port or --server
      Arg::new("server port")
          .short('p')
          .long("port")
          .alias("server")
          .help("expose a port for the server to listen to (Default: 4445)")
          .value_parser(value_parser!(u16))
  )
  .arg(
    // --config
    Arg::new("config")
      .long("config")
      .help("save your passing arguments into a toml file")
      .value_parser(value_parser!(bool))
  )
  .get_matches()
}
