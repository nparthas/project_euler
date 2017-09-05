pub fn q19() -> i64 {

    let start_year: i16 = 1901;
    let end_year: i16 = 2000;
    let day: i16 = 0;
    let mut count: i64 = 0;

    for year in start_year..end_year {
        for month in 1..13 {
            if week_day(year, month, 1) == day {
                count += 1;
            }
        }
    }
    return count;
}


fn week_day(year: i16, month: i16, day: i16) -> i16 {

    let mth: f64 = ((month - 3) % 12 + 1) as f64;
    let yr: i16 = if mth > 10 as f64 { (year - 1) } else { year };
    let y: i16 = yr % 100;
    let c: i16 = (yr - y) / 100;

    let w: i16 = (day + (2.6 * mth - 0.2).floor() as i16 + y + y / 4 + c / 4 - 2 * c) % 7;


    w
}
