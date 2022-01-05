#![allow(warnings)] // NOT RECOMMENDED

use std::time::{Duration, Instant};
use chrono::{Utc, DateTime, Duration as ChronoDuration, Local, FixedOffset, Datelike, Timelike};

fn main() {
    let now = Instant::now();

    // for i in 0..1_000_000_00 {
    //     let _ = i;
    // }

    let duration = now.elapsed();

    dbg!(&duration);
    dbg!(&duration.as_millis());
    dbg!(duration.as_secs());

    let now = Utc::now();
    let future = now.checked_add_signed(ChronoDuration::weeks(2));

    dbg!(&future);

    let past = now.checked_sub_signed(ChronoDuration::days(60));

    dbg!(&past);
    dbg!(&now);
    dbg!(Local::now());
    dbg!(FixedOffset::east(19800));// 5.5 hours ahead of UTC (19800 = 5.5 * 60 * 60)
    dbg!(now.with_timezone(&FixedOffset::east(19800)));


    let (is_pm, hour) = now.hour12();
    println!("{} {}", hour, if is_pm { "PM" } else { "AM" });
    println!("{}:{}:{} {}", hour, now.minute(), now.second(), if is_pm { "PM" } else { "AM" });

    dbg!(now.date());
    dbg!(now.to_rfc2822());
    dbg!(now.to_rfc3339());
    // dbg!(now.format("%Y-%m-%d %H:%M:%S %z"));
    println!("{}", now.format("%Y-%m-%d %H:%M:%S %z"));

    let dt = DateTime::parse_from_rfc2822("Fri, 21 Nov 1997 09:55:06 -0600").unwrap();
    dbg!(&dt);

    let custom = DateTime::parse_from_str("2020-11-01 08:15:27 +0000", "%Y-%m-%d %H:%M:%S %z").unwrap();
    dbg!(&custom);

}