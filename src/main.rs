const IDENT_ERROR: &str = ":ERROR:UNKNOWN-ERROR\r\n";

// FIXME: Add get ports from request
fn show_fake_id(ident_request: &String) -> String {
  let error = String::from(IDENT_ERROR);
  return error;
}

#[test]
fn bogus_request() {
  let reply = show_fake_id(&String::from("GURKEN"));
  assert_eq!(reply, String::from(IDENT_ERROR));
}

#[test]
fn correct_request() {
  let reply = show_fake_id(&String::from("13,37\r\n"));
  assert_eq!(reply, String::from("13,37:USERID:UNIX:aerokid"));
}

fn main() {
  let request = String::from("13,37\r\n");
  let reply = show_fake_id(&request);
  println!("{}", reply);
}
