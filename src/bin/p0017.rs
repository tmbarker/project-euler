use once_cell::sync::Lazy;
use std::collections::HashMap;

fn solve() -> String {
    number_letter_counts(1000).to_string()
}

fn number_letter_counts(n: usize) -> usize {
    (1..=n)
        .map(number_to_word_len)
        .sum()
}

fn number_to_word_len(n: usize) -> usize {
    let mut words = String::new();

    let thousands = n / 1000;
    let hundreds = (n % 1000) / 100;
    let remainder = n % 100;

    if thousands > 0 {
        words.push_str(&WORD_MAP.get(&thousands).unwrap());
        words.push_str("thousand");
    }

    if hundreds > 0 {
        words.push_str(&WORD_MAP.get(&hundreds).unwrap());
        words.push_str("hundred");
    }

    if (thousands > 0 || hundreds > 0) && remainder > 0 {
        words.push_str("and");
    }

    if let Some(&word) = &WORD_MAP.get(&remainder) {
        words.push_str(word);
    } else {
        let tens = remainder / 10 * 10;
        let ones = remainder % 10;

        if tens > 0 {
            words.push_str(&WORD_MAP.get(&tens).unwrap());
        }

        if ones > 0 {
            words.push_str(&WORD_MAP.get(&ones).unwrap());
        }
    }

    words.len()
}

euler::register_problem!("Number Letter Counts", solve, "21124");

static WORD_MAP: Lazy<HashMap<usize, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(   1, "one");
    m.insert(   2, "two");
    m.insert(   3, "three");
    m.insert(   4, "four");
    m.insert(   5, "five");
    m.insert(   6, "six");
    m.insert(   7, "seven");
    m.insert(   8, "eight");
    m.insert(   9, "nine");
    m.insert(  10, "ten");
    m.insert(  11, "eleven");
    m.insert(  12, "twelve");
    m.insert(  13, "thirteen");
    m.insert(  14, "fourteen");
    m.insert(  15, "fifteen");
    m.insert(  16, "sixteen");
    m.insert(  17, "seventeen");
    m.insert(  18, "eighteen");
    m.insert(  19, "nineteen");
    m.insert(  20, "twenty");
    m.insert(  30, "thirty");
    m.insert(  40, "forty");
    m.insert(  50, "fifty");
    m.insert(  60, "sixty");
    m.insert(  70, "seventy");
    m.insert(  80, "eighty");
    m.insert(  90, "ninety");
    m.insert( 100, "hundred");
    m.insert(1000, "thousand");
    m
});