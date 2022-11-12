use anyhow::Result;
use icalendar::*;
use std::collections::HashMap;

use crate::time::{parse_duration, parse_time};

pub struct CalendarParseError {
    pub err: String,
}
const DEFAULT_EVENT_TITLE: &str = "New Calendar Event";

pub fn create_calendar(params: HashMap<String, String>) -> Result<Calendar, CalendarParseError> {
    let mut event = Event::new();

    if let Some(title) = params.get("title") {
        event.summary(title);
    } else {
        event.summary(DEFAULT_EVENT_TITLE);
    }
    if let Some(desc) = params.get("desc") {
        event.description(desc);
    } else {
        event.description("Powered by zerocal.shuttleapp.rs");
    }

    match params.get("start") {
        Some(start) if !start.is_empty() => {
            let start = match parse_time(start) {
                Ok(start) => start,
                Err(e) => {
                    return Err(CalendarParseError {
                        err: format!("Invalid start time: {}", e),
                    });
                }
            };
            event.starts(start);
            if let Some(duration) = params.get("duration") {
                let duration = match parse_duration(duration) {
                    Ok(duration) => duration,
                    Err(e) => {
                        return Err(CalendarParseError {
                            err: format!("Invalid duration: {}", e),
                        });
                    }
                };
                event.ends(start + duration);
            }
        }
        _ => {
            // start is not set or empty; default to 1 hour event
            event.starts(chrono::Utc::now());
            event.ends(chrono::Utc::now() + chrono::Duration::hours(1));
        }
    }

    match params.get("end") {
        Some(end) if !end.is_empty() => {
            let end = match parse_time(end) {
                Ok(end) => end,
                Err(e) => {
                    return Err(CalendarParseError {
                        err: format!("Invalid end time: {}", e),
                    });
                }
            };
            event.ends(end);
            if let Some(duration) = params.get("duration") {
                if params.get("start").is_none() {
                    // If only duration is given but no start, set start to end - duration
                    let duration = match parse_duration(duration) {
                        Ok(duration) => duration,
                        Err(e) => {
                            return Err(CalendarParseError {
                                err: format!("Invalid duration: {}", e),
                            });
                        }
                    };
                    event.starts(end - duration);
                }
            }
        }
        _ => {
            // end is not set or empty; default to 1 hour event
            // TODO: handle case where start is set
            event.starts(chrono::Utc::now());
            event.ends(chrono::Utc::now() + chrono::Duration::hours(1));
        }
    }

    if let Some(location) = params.get("location") {
        event.location(location);
    }

    Ok(Calendar::new().push(event.done()).done())
}
