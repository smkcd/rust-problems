use std::alloc::System;
use std::time::SystemTime;
use time::format_description;
use time::OffsetDateTime;

pub fn systemtime_strftime<T>(dt: T) -> String
where
    T: Into<OffsetDateTime>,
{
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second]", //[offset_hour \
                                                         //sign:mandatory]:[offset_minute]:[offset_second]",
    )
    .expect("could not parse format");
    dt.into()
        .format(&format)
        .expect("could not create date string")
}

pub fn now_as_string() -> String {
    systemtime_strftime(SystemTime::now())
}
