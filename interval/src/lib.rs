#![warn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

#[derive(Debug, PartialEq)]
struct Interval<T> {
    lower: T, // inclusive
    upper: T, // exclusive
}

use std::cmp::{Ordering, PartialOrd};

impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
    fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower >= other.upper {
            Some(Ordering::Greater)
        } else if self.upper <= other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[test]
fn test() {
    assert!(Interval { lower: 10, upper: 20 } <  Interval { lower: 20, upper: 40 });
    assert!(Interval { lower: 7,  upper: 8  } >= Interval { lower: 0,  upper: 1  });
    assert!(Interval { lower: 7,  upper: 8  } <= Interval { lower: 7,  upper: 8  });
    assert!(Interval { lower: 7,  upper: 8  }.le(&Interval { lower: 7,  upper: 8  }));

    // Overlapping intervals aren't ordered with respect to each other.
    let left  = Interval { lower: 10, upper: 30 };
    let right = Interval { lower: 20, upper: 40 };
    assert!(!(left < right));
    assert!(!(left >= right));
}
