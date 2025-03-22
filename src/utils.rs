use shakmaty::Color;

#[inline]
pub fn get_color_factor(color: Color) -> i16 {
    match color {
        Color::Black => -1,
        Color::White => 1
    }
}

pub fn signum(number: isize) -> i8 {
    if number == 0 {
        return 0;
    } else if number > 0 {
        return 1;
    }

    -1
}

pub fn clamp<T: PartialOrd>(low: T, value: T, high: T) -> T {
    debug_assert!(low < high, "low is bigger than high!");
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}