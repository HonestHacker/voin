use std::cmp::Ordering;
use std::ops;

use shakmaty::Color;

use crate::utils::signum;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Score {
    Centipawn(i16),
    Mate(i8),
}

impl Score {
    pub const MAX: Self = Score::Centipawn(i16::MAX);
    pub const MIN: Self = Score::Centipawn(i16::MIN);
    pub const ZERO: Self = Score::Centipawn(0);

    #[inline]
    pub fn apply_color_factor(self, color: Color) -> Self {
        if matches!(color, Color::Black) {
            -self
        } else {
            self
        }
    }

    #[inline]
    pub fn is_min(&self) -> bool {
        matches!(self, Self::Centipawn(val) if *val == i16::MIN)
    }

    #[inline]
    pub fn is_max(&self) -> bool {
        matches!(self, Self::Centipawn(val) if *val == i16::MAX)
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        match self {
            Self::Centipawn(val) => val.is_negative(),
            Self::Mate(val) => val.is_negative(),
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ if self.is_max() => write!(f, "score upperbound"),
            _ if self.is_min() => write!(f, "score lowerbound"),
            Self::Centipawn(val) => write!(f, "score cp {}", val),
            Self::Mate(val) => write!(f, "score mate {}", (val - signum(*val as isize)) / 2),
        }
    }
}

impl ops::Neg for Score {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Self::Centipawn(val) => Self::Centipawn(-val),
            Self::Mate(val) => Self::Mate(-val),
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Mate(this), Self::Mate(other)) => {
                if *this < 0 && *other > 0 {
                    Ordering::Less
                } else if *this > 0 && *other < 0 {
                    Ordering::Greater
                } else {
                    other.cmp(this)
                }
            }
            (Self::Centipawn(this), Self::Centipawn(other)) => this.cmp(other),
            (Self::Mate(_), _) if other.is_min() => Ordering::Greater,
            (_, Self::Mate(_)) if self.is_min() => Ordering::Less,
            (Self::Mate(_), _) if self.is_negative() => Ordering::Less,
            (_, Self::Mate(_)) if other.is_negative() => Ordering::Greater,
            (Self::Mate(_), _) => Ordering::Greater,
            (_, Self::Mate(_)) => Ordering::Less,
        }
    }
}

impl ops::Add for Score {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Self::Mate(this), Self::Mate(other)) => Self::Mate(this + other),
            (Self::Mate(_), _) => self,
            (_, Self::Mate(_)) => rhs,
            (Self::Centipawn(this), Self::Centipawn(other)) => {
                Self::Centipawn(this.saturating_add(*other))
            }
        }
    }
}

impl ops::Add<isize> for Score {
    type Output = Self;
    fn add(self, rhs: isize) -> Self::Output {
        match self {
            Self::Centipawn(this) => Self::Centipawn(this + rhs as i16),
            Score::Mate(this) => Self::Mate(this + rhs as i8)
        }
    }
}

impl ops::Sub for Score {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl ops::Sub<isize> for Score {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: isize) -> Self::Output {
        self + -rhs
    }
}

impl ops::AddAssign for Score {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Div<i16> for Score {
    type Output = Self;
    fn div(self, rhs: i16) -> Self::Output {
        match self {
            Self::Mate(val) => Self::Mate(val / rhs as i8),
            Self::Centipawn(val) => Self::Centipawn(val / rhs),
        }
    }
}

impl ops::Mul<i16> for Score {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: i16) -> Self::Output {
        match self {
            Self::Mate(val) => Self::Mate(val * rhs as i8),
            Self::Centipawn(val) => Self::Centipawn(val * rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Score::*;

    macro_rules! assert_cmp {
        ($this:expr ;> $($other:expr$(,)?)+) => {
            $(
                assert!($this > $other);
                assert!($other < $this);
            )*
        };
        ($this:expr ;< $($other:expr$(,)?)+) => {
            $(
                assert!($this < $other);
                assert!($other > $this);
            )*
        };
        ($this:expr ;== $($other:expr$(,)?)+) => {
            $(
                assert!($this == $other);
                assert!($other == $this);
            )*
        };
        ($this:expr ;>= $($other:expr$(,)?)+) => {
            $(
                assert!($this >= $other);
                assert!($other <= $this);
            )*
        };
        ($this:expr ;<= $($other:expr$(,)?)+) => {
            $(
                assert!($this <= $other);
                assert!($other >= $this);
            )*
        };
    }

    #[test]
    fn test_mate_ord() {
        assert_cmp!(Mate(2) ;>= Mate(2), Mate(3), Mate(-3), Centipawn(10), Centipawn(-10), Score::MAX);
        assert_cmp!(Mate(-5) ;> Mate(-3), Score::MIN);
        assert_cmp!(Mate(-5) ;< Centipawn(10), Centipawn(-10));
    }

    #[test]
    fn test_centipawn_ord() {
        assert_cmp!(Centipawn(10) ;> Centipawn(9),Centipawn(-1));
        assert_cmp!(Centipawn(-1) ;< Centipawn(9),Centipawn(0));
    }

    macro_rules! assert_op_eq {
        ($lhs:expr ;+ $rhs:expr, $res:expr) => {
            assert_eq!($lhs + $rhs, $res);
            assert_eq!($rhs + $lhs, $res);
        };
        ($lhs:expr ;- $rhs:expr, $res:expr) => {
            assert_eq!($lhs - $rhs, $res);
            assert_eq!(-($rhs - $lhs), $res);
        };
    }

    #[test]
    fn test_add() {
        assert_op_eq!(Centipawn(1) ;+ Centipawn(2), Centipawn(3));
        assert_op_eq!(Centipawn(1) ;- Centipawn(2), Centipawn(-1));
        assert_op_eq!(Centipawn(1) ;+ Mate(3), Mate(3));
    }
}
