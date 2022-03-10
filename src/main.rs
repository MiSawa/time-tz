use std::process::Command;

use time::{
    macros::{datetime, format_description},
    OffsetDateTime,
};
use time_tz::{parse_tz::parse, Offset, OffsetDateTimeExt, TimeZone};

fn test_tz_with_timestamp(tz: &str, timestamp: i64) {
    let expected = String::from_utf8(
        Command::new("date")
            .env("TZ", tz)
            .args(["-d", &format!("@{timestamp}"), "+%Y-%m-%d %H:%M:%S %z %Z"])
            .output()
            .expect(&format!(
                "Failed to execute the date command for timezone {tz} and timestamp {timestamp}"
            ))
            .stdout,
    )
    .expect(&format!(
        "date command for timezone {tz} and timestamp {timestamp} output non-utf8"
    ));
    let parsed_tz = parse(tz)
        .map_err(|e| format!("{e:?}"))
        .expect(&format!("Failed to parse timezone {tz})"));
    let datetime = OffsetDateTime::from_unix_timestamp(timestamp)
        .unwrap()
        .to_timezone(&parsed_tz);
    let offset = parsed_tz.get_offset_utc(&datetime);
    let s = datetime
        .format(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory][offset_minute]"
        ))
        .expect(&format!("Failed to format {timestamp} with {tz}"));
    let actual = s + " " + offset.name() + "\n";
    assert_eq!(actual, expected);
}

fn test_tz_with_stride(tz: &str, stride_second: usize) {
    // Non-leap (2019) and leap (2020) year, plus a bit wider.
    let start = datetime!(2018-12-29 00:00 UTC).unix_timestamp();
    let end = datetime!(2021-01-03 00:00 UTC).unix_timestamp();
    for timestamp in (start..end).step_by(stride_second) {
        test_tz_with_timestamp(tz, timestamp)
    }
}

fn test_tz(tz: &str) {
    println!("Quick test on {tz}");
    // Check for every day
    test_tz_with_stride(tz, 24 * 60 * 60);
    println!("More test on {tz}");
    // Check for every 30 minutes
    test_tz_with_stride(tz, 30 * 60);
}

fn main() {
    test_tz("FOO+12:34");
    test_tz("STD-5DST,10,20");
    test_tz("STD-5DST,100/3,120/4");
    test_tz("STD-5DST,J100,J120");

    test_tz("EST+5EDT,M3.2.0/2,M11.1.0/2");
    // test_tz("IST-2IDT,M3.4.4/26,M10.5.0");
    test_tz("IST-2IDT,M3.4.4/23,M10.5.0");
    // test_tz("WART4WARST,J1/0,J365/25");
    test_tz("ABC4DEF,J2/0,J364/23");
    // test_tz("WGT3WGST,M3.5.0/-2,M10.5.0/-1");
}
