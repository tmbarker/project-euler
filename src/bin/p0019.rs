fn solve() -> String {
    count_sundays().to_string()
}

fn count_sundays() -> usize {
    SundayIter::new(7, 1, 1900)
        .take_while(|sunday| sunday.year <= 2000)
        .filter(|sunday| sunday.year >= 1901 && sunday.day == 1)
        .count()
}

#[inline]
fn get_month_len(month: u64, year: u64) -> u64 {
    match month {
        2 if is_leap_year(year) => 29, // Feb (non-leap years)
        2 => 28,                       // Feb (leap years)
        4 | 6 | 9 | 11 => 30,          // Apr, Jun, Sep, Nov
        _ => 31,                       // All other months
    }
}

#[inline]
fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[derive(Copy, Clone)]
struct DayOfWeek {
    day: u64,
    month: u64,
    year: u64,
}

struct SundayIter {
    day: DayOfWeek,
}

impl SundayIter {
    fn new(day_of_month: u64, month: u64, year: u64) -> Self {
        SundayIter {
            day: DayOfWeek {
                day: day_of_month,
                month,
                year,
            },
        }
    }
}

impl Iterator for SundayIter {
    type Item = DayOfWeek;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.day;
        let month_len = get_month_len(cur.month, cur.year);

        let roll_month = cur.day + 7 > month_len;
        let roll_year = roll_month && cur.month == 12;
        let new_month = cur.month + if roll_month { 1 } else { 0 };

        self.day = DayOfWeek {
            day: cur.day + 7 - if roll_month { month_len } else { 0 },
            month: if new_month <= 12 { new_month } else { 1 },
            year: cur.year + if roll_year { 1 } else { 0 },
        };
        Some(self.day)
    }
}

euler::register_problem!("Counting Sundays", solve, "171");
