use log::{debug, error, info, LevelFilter};
use std::num::ParseIntError;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(argh::FromArgs, Debug)]
#[argh(
  description = "Who dat kid?",
  example = "Listen on localhost port 1337 and set log level to debug.\n$ {command_name} -l localhost:1337 -vv"
)]
struct WhoDatKid {
  /// address to listen to. E.g. localhost:1337
  #[argh(option, short = 'l', default = r#""[::]:1337".to_string()"#)]
  listen: String,

  /// control the verbosity of logging. One = info, two = debug
  #[argh(switch, short = 'v')]
  verbose: i32,
}

async fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];

  info!("Connection from: {}", stream.peer_addr().unwrap());
  stream.read(&mut buffer).await.unwrap();

  let request = String::from_utf8_lossy(&buffer)
    .trim_end_matches('\0')
    .to_string();
  debug!("request: {:#?}", request);
  let reply = show_fake_id(&request);
  stream.write(reply.as_bytes()).await.unwrap();
}

#[derive(Debug, PartialEq)]
struct Ident {
  remote_port: i32,
  local_port: i32,
}

impl FromStr for Ident {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    debug!("s: {:#?}", s);
    let coords: Vec<&str> = s
      .trim_matches(|p| p == ' ' || p == '\r' || p == '\n')
      .split(|c| c == ',' || c == ':')
      .collect();

    debug!("coords: {:#?}", coords);
    let remote_port_fromstr = coords[0].parse::<i32>()?;
    let local_port_fromstr = coords[1].parse::<i32>()?;

    Ok(Ident {
      remote_port: remote_port_fromstr,
      local_port: local_port_fromstr,
    })
  }
}

const IDENT_ERROR: &str = ":ERROR:UNKNOWN-ERROR\r\n";

fn show_fake_id(ident_request: &str) -> String {
  let ports = get_ports(ident_request);
  debug!("Ident request: {:?}", ident_request);
  let trimmed_ident_request = ident_request.trim_matches(char::from(0));
  match ports {
    Ok(ident) => {
      debug!("Ident request: {:?}", trimmed_ident_request);
      create_reply(&ident)
    }
    Err(_) => {
      error!("Bogus ident reply: {:?}", trimmed_ident_request);
      String::from(IDENT_ERROR)
    }
  }
}

fn get_ports(ident_request: &str) -> Result<Ident, ParseIntError> {
  Ident::from_str(ident_request)
}

fn create_reply(ident_request: &Ident) -> String {
  return format!(
    "{},{}:USERID:UNIX:aerokid\r\n",
    ident_request.remote_port, ident_request.local_port,
  );
}

#[test]
fn bogus_request() {
  let reply = show_fake_id(&String::from("GURKEN"));
  assert_eq!(reply, String::from(IDENT_ERROR));
}

#[test]
fn correct_request() {
  let reply = show_fake_id(&String::from("13,37\r\n"));
  assert_eq!(reply, String::from("13,37:USERID:UNIX:aerokid\r\n"));
}

#[test]
fn correct_ports() {
  let p = get_ports(&String::from("13,37\r\n"));
  assert_eq!(
    p.unwrap(),
    Ident {
      remote_port: 13,
      local_port: 37
    }
  )
}
#[test]
fn bogus_ports() {
  let p = get_ports(&String::from("pewpew\r\n"));
  assert!(p.is_err())
}

#[test]
fn correct_reply() {
  let p = create_reply(&Ident {
    remote_port: 13,
    local_port: 37,
  });
  assert_eq!(p, String::from("13,37:USERID:UNIX:aerokid\r\n"))
}

#[tokio::main]
async fn main() {
  let args: WhoDatKid = argh::from_env();
  let mut loglevel: LevelFilter = LevelFilter::Error;
  if args.verbose == 1 {
    loglevel = LevelFilter::Info;
  } else if args.verbose == 2 {
    loglevel = LevelFilter::Debug;
  }
  env_logger::Builder::from_default_env()
    .format_level(true)
    .format_module_path(false)
    .format_timestamp(None)
    .filter(None, loglevel)
    .init();

  let mut listener = TcpListener::bind(&args.listen).await.unwrap();
  info!("Listening on {}", &args.listen);

  loop {
    let (stream, _) = listener.accept().await.unwrap();

    tokio::spawn(async move {
      handle_connection(stream).await;
    });
  }
}
