fn show_fake_id(ident_request: &String) -> String {
  let error = String::from(":ERROR:UNKNOWN-ERROR\r\n");
  return error;
}

#[test]
fn bogus_request() {
  let reply = show_fake_id(&String::from("GURKEN"));
  assert_eq!(reply, String::from(":ERROR:UNKNOWN-ERROR\r\n"));
}

fn main() {
  let request = String::from("13,37\r\n");
  let reply = show_fake_id(&request);
  println!("{}", reply);
}
