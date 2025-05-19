const T_UTC: time::OffsetDateTime = compile_time::datetime!();
const T_LOCAL: time::OffsetDateTime = compile_time::datetime_local!();

const U_UTC: i64 = compile_time::unix!();
const U_LOCAL: i64 = compile_time::unix_local!();

#[test]
// FIXME: this is going to fail if executed in a timezone where local time = universal time
fn test_local_unix() {
  let tl = time::OffsetDateTime::now_local().unwrap();
  let tu = time::OffsetDateTime::now_utc();
  println!("Local time: {}", tl);
  println!("Uni__ time: {}", tu);

  assert_ne!(T_UTC, T_LOCAL, "local time and universal time are the same");

  println!("const");
  println!("Local time: {}", T_LOCAL);
  println!("Uni__ time: {}", T_UTC);

  assert_ne!(U_UTC, U_LOCAL, "unix universal time and unix local time from proc macro are the same");

  assert_ne!(
    T_UTC.unix_timestamp(),
    T_LOCAL.unix_timestamp(),
    "unix timestamp of universal and local times are the same"
  );
  assert_eq!(
    T_UTC.unix_timestamp(),
    U_UTC,
    "unix timestamp of universal time is not the same as unix timestamp gotten with proc macro"
  );
  assert_eq!(
    T_LOCAL.unix_timestamp(),
    U_LOCAL,
    "unix timestamp of local time is not the same as unix timestamp gotten with proc macro"
  );
}
