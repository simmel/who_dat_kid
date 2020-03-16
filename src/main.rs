use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Ident {
  remote_port: i32,
  local_port: i32,
}

impl FromStr for Ident {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let coords: Vec<&str> = s
      .trim_matches(|p| p == ' ' || p == '\r' || p == '\n')
      .split(|c| c == ',' || c == ':')
      .collect();

    let remote_port_fromstr = coords[0].parse::<i32>()?;
    let local_port_fromstr = coords[1].parse::<i32>()?;

    Ok(Ident {
      remote_port: remote_port_fromstr,
      local_port: local_port_fromstr,
    })
  }
}

const IDENT_ERROR: &str = ":ERROR:UNKNOWN-ERROR\r\n";

fn show_fake_id(ident_request: &String) -> String {
  let ports = get_ports(ident_request);
  match ports {
    Ok(ident) => create_reply(&ident),
    Err(_) => String::from(IDENT_ERROR),
  }
}

fn get_ports(ident_request: &String) -> Result<Ident, ParseIntError> {
  return Ident::from_str(ident_request);
}

fn create_reply(ident_request: &Ident) -> String {
  return String::from(format!(
    "{},{}:USERID:UNIX:aerokid\r\n",
    ident_request.remote_port, ident_request.local_port,
  ));
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

fn main() {
  let request = String::from("13,37\r\n");
  let reply = show_fake_id(&request);
  println!("{}", reply);
}
