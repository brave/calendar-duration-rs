use calendar_duration::CalendarDuration;
use time::OffsetDateTime;

fn main() {
    let duration = CalendarDuration::from("3d2h10m");

    let mut time = OffsetDateTime::now_utc();

    println!("The time is now {time}");
    println!("Will add {duration} to the time");

    time = time + duration;

    println!("The result is {time}");

    let duration = CalendarDuration::from("1y1mon");
    println!("Will subtract {duration} from the time");

    time = time - duration;

    println!("The result is {time}");
}
