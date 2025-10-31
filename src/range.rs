use crate::result::Result;
use core::ops::RangeInclusive;

pub fn map_value_in_range_inclusive(
    from: RangeInclusive<i64>,
    to: RangeInclusive<i64>,
    v: i64,
) -> Result<i64> {
    if !from.contains(&v) {
        Err("v is not in range from")
    } else {
        let from_left = (v - *from.start()) as i128;
        let from_width = (from.end() - from.start()) as i128;
        let to_width = (to.end() - to.start()) as i128;
        if from_width == 0 {
            Ok(*to.start())
        } else {
            let to_left = from_left * to_width / from_width;
            to_left
                .try_into()
                .or(Err("failed to convert to_left to the result type"))
                .map(|to_left: i64| to.start() + to_left)
        }
    }
}

#[test_case]
fn test_map_value_in_range_inclusive() {
    assert_eq!(
        map_value_in_range_inclusive(0..=100, 0..=200, 50).unwrap(),
        100
    );
    assert_eq!(
        map_value_in_range_inclusive(0..=100, 100..=200, 50).unwrap(),
        150
    );
    assert_eq!(
        map_value_in_range_inclusive(50..=150, 0..=100, 100).unwrap(),
        50
    );
    assert_eq!(
        map_value_in_range_inclusive(-50..=50, 0..=100, 0).unwrap(),
        50
    );
    assert_eq!(
        map_value_in_range_inclusive(-100..=0, -200..=-100, -50).unwrap(),
        -150
    );
}
