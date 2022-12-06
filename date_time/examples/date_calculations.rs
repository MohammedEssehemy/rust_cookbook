use chrono::{DateTime, Duration, Utc};

fn main() {
    let now = Utc::now();
    println!("{}", now);

    let almost_three_weeks_from_now = now
        .checked_add_signed(Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(|in_3weeks| in_3weeks.checked_sub_signed(Duration::days(1)));

    match almost_three_weeks_from_now {
        Some(x) => println!("almost_three_weeks_from_now {}", x),
        None => eprintln!("Almost three weeks from now overflows!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(x) => println!("Duration::max_value {}", x),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."),
    }
}
