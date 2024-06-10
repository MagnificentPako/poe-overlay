use chrono::{DateTime, Local, TimeZone};
use nom::{
    branch::Alt,
    bytes::streaming::tag,
    character::complete::{alphanumeric1, anychar, char, digit1, none_of, space1},
    multi::{many1, many_till, separated_list1},
    IResult,
};
use notify::{Config, RecommendedWatcher, RecursiveMode, Result, Watcher};
use shared::{Log, LogEvent};
use std::{
    io::{BufRead, BufReader, Seek},
    path::Path,
    sync::mpsc::SyncSender,
};

fn time(input: &str) -> IResult<&str, (u32, u32, u32)> {
    separated_list1(char(':'), digit1)(input).map(|x| {
        let mut iter = x.1.into_iter().take(3).map(|y| y.parse::<u32>().unwrap());
        (
            x.0,
            (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ),
        )
    })
}

fn date(input: &str) -> IResult<&str, (i32, u32, u32)> {
    separated_list1(char('/'), digit1)(input).map(|x| {
        let mut iter = x.1.into_iter().take(3).map(|y| y.parse::<u32>().unwrap());
        (
            x.0,
            (
                i32::try_from(iter.next().unwrap()).unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ),
        )
    })
}

fn date_time(input: &str) -> IResult<&str, DateTime<Local>> {
    let (input, (year, month, day)) = date(input)?;
    let (input, _) = space1(input)?;
    let (input, (hour, minute, second)) = time(input)?;
    Ok((
        input,
        Local
            .with_ymd_and_hms(year, month, day, hour, minute, second)
            .unwrap(),
    ))
}

fn ip_and_port(input: &str) -> IResult<&str, String> {
    let (input, ip) = separated_list1(char('.'), digit1)(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, port) = digit1(input)?;
    let ip_joined = ip.join(".");
    let port_string = port.to_string();
    let full_ip = [ip_joined, port_string];
    Ok((input, full_ip.join(":")))
}

fn connect_string(input: &str) -> IResult<&str, Option<LogEvent>> {
    let (input, _) = tag("Connecting to instance server at ")(input)?;
    let (input, ip) = ip_and_port(input)?;
    Ok((input, Some(LogEvent::Connect { server: ip })))
}

fn zone_enter(input: &str) -> IResult<&str, Option<LogEvent>> {
    let (input, _) = tag(": You have entered ")(input)?;
    let (input, (zone_vec, _)) = many_till(anychar, tag("."))(input)?;
    Ok((
        input,
        Some(LogEvent::ZoneChange {
            zone: zone_vec.into_iter().collect(),
        }),
    ))
}

fn generate_zone(input: &str) -> IResult<&str, Option<LogEvent>> {
    let (input, _) = tag("Generating level ")(input)?;
    let (input, zone_level) = digit1(input)?;
    let (input, _) = tag(" area \"")(input)?;
    let (input, (zone_name, _)) = many_till(anychar, tag("\""))(input)?;

    Ok((
        input,
        Some(LogEvent::GenerateZone {
            zone_level: zone_level.parse().unwrap(),
            zone_name: zone_name.into_iter().collect(),
        }),
    ))
}

fn parse_log_line(input: &str) -> IResult<&str, Option<Log>> {
    let (input, dt) = date_time(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = many1(none_of("]"))(input)?;
    let (input, _) = tag("] ")(input)?;

    let fail_parser = |input| Ok((input, None));

    let (input, event) = (connect_string, zone_enter, generate_zone, fail_parser).choice(input)?;

    if let Some(evt) = event {
        Ok((input, Some(Log { dt: dt, event: evt })))
    } else {
        Ok((input, None))
    }
}

pub fn parse_log_tail(path: &Path, log_sender: SyncSender<Log>) -> Result<()> {
    let mut f = std::fs::File::open(path)?;
    let mut pos = std::fs::metadata(path)?.len();

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(_) => {
                if f.metadata()?.len() == pos {
                    continue;
                }

                f.seek(std::io::SeekFrom::Start(pos + 1))?;
                pos = f.metadata()?.len();
                let reader = BufReader::new(&f);
                for line in reader.lines() {
                    let l = line?;
                    if let Ok((_, log)) = parse_log_line(&l) {
                        if let Some(parsed_log) = log {
                            log_sender.send(parsed_log).unwrap();
                        }
                    }
                }
            }
            Err(error) => println!("{error:?}"),
        }
    }

    Ok(())
}
