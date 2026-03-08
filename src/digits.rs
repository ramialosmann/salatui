/// tty-clock style ASCII digit font data.
///
/// Each digit (0-9) and colon (index 10) is a 5-row x 6-column boolean grid.
/// `true` = filled block (U+2588), `false` = space.
pub const DIGIT_WIDTH: u16 = 6;
pub const DIGIT_HEIGHT: u16 = 5;

pub const DIGITS: [[[bool; 6]; 5]; 11] = [
    // 0
    [
        [false, true, true, true, true, false],
        [true, true, false, false, true, true],
        [true, true, false, false, true, true],
        [true, true, false, false, true, true],
        [false, true, true, true, true, false],
    ],
    // 1
    [
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
    ],
    // 2
    [
        [false, true, true, true, true, false],
        [false, false, false, false, true, true],
        [false, true, true, true, true, false],
        [true, true, false, false, false, false],
        [false, true, true, true, true, false],
    ],
    // 3
    [
        [false, true, true, true, true, false],
        [false, false, false, false, true, true],
        [false, true, true, true, true, false],
        [false, false, false, false, true, true],
        [false, true, true, true, true, false],
    ],
    // 4
    [
        [true, true, false, false, true, true],
        [true, true, false, false, true, true],
        [false, true, true, true, true, false],
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
    ],
    // 5
    [
        [false, true, true, true, true, false],
        [true, true, false, false, false, false],
        [false, true, true, true, true, false],
        [false, false, false, false, true, true],
        [false, true, true, true, true, false],
    ],
    // 6
    [
        [false, true, true, true, true, false],
        [true, true, false, false, false, false],
        [false, true, true, true, true, false],
        [true, true, false, false, true, true],
        [false, true, true, true, true, false],
    ],
    // 7
    [
        [false, true, true, true, true, false],
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
        [false, false, false, false, true, true],
    ],
    // 8
    [
        [false, true, true, true, true, false],
        [true, true, false, false, true, true],
        [false, true, true, true, true, false],
        [true, true, false, false, true, true],
        [false, true, true, true, true, false],
    ],
    // 9
    [
        [false, true, true, true, true, false],
        [true, true, false, false, true, true],
        [false, true, true, true, true, false],
        [false, false, false, false, true, true],
        [false, true, true, true, true, false],
    ],
    // 10 = colon
    [
        [false, false, false, false, false, false],
        [false, false, true, true, false, false],
        [false, false, false, false, false, false],
        [false, false, true, true, false, false],
        [false, false, false, false, false, false],
    ],
];

/// Maps a character to its index in the DIGITS array.
/// '0'-'9' -> 0-9, ':' -> 10, anything else -> None.
pub fn digit_index(c: char) -> Option<usize> {
    match c {
        '0'..='9' => Some((c as u8 - b'0') as usize),
        ':' => Some(10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_digits_have_correct_dimensions() {
        for (i, digit) in DIGITS.iter().enumerate() {
            assert_eq!(digit.len(), 5, "Digit {} should have 5 rows", i);
            for (row_idx, row) in digit.iter().enumerate() {
                assert_eq!(
                    row.len(),
                    6,
                    "Digit {} row {} should have 6 columns",
                    i,
                    row_idx
                );
            }
        }
    }

    #[test]
    fn test_colon_dots_at_correct_positions() {
        let colon = &DIGITS[10];
        // Row 1 (index 1): dots at columns 2,3
        assert!(colon[1][2], "Colon should have dot at row 1, col 2");
        assert!(colon[1][3], "Colon should have dot at row 1, col 3");
        // Row 3 (index 3): dots at columns 2,3
        assert!(colon[3][2], "Colon should have dot at row 3, col 2");
        assert!(colon[3][3], "Colon should have dot at row 3, col 3");
        // Row 0, 2, 4 should be all false
        for col in 0..6 {
            assert!(!colon[0][col], "Colon row 0 should be empty");
            assert!(!colon[2][col], "Colon row 2 should be empty");
            assert!(!colon[4][col], "Colon row 4 should be empty");
        }
    }

    #[test]
    fn test_digit_index_valid_digits() {
        for i in 0..=9 {
            let c = char::from(b'0' + i as u8);
            assert_eq!(digit_index(c), Some(i));
        }
    }

    #[test]
    fn test_digit_index_colon() {
        assert_eq!(digit_index(':'), Some(10));
    }

    #[test]
    fn test_digit_index_invalid() {
        assert_eq!(digit_index('x'), None);
        assert_eq!(digit_index(' '), None);
        assert_eq!(digit_index('a'), None);
    }

    #[test]
    fn test_digit_1_only_right_columns() {
        let one = &DIGITS[1];
        for row in one {
            // Only columns 4 and 5 should be true
            for col in 0..4 {
                assert!(!row[col], "Digit 1 should only have right columns filled");
            }
            assert!(row[4], "Digit 1 col 4 should be filled");
            assert!(row[5], "Digit 1 col 5 should be filled");
        }
    }

    #[test]
    fn test_digit_0_has_hollow_center() {
        let zero = &DIGITS[0];
        // Middle rows (1,2,3) should have sides filled, center empty
        for row_idx in 1..=3 {
            assert!(zero[row_idx][0], "Digit 0 left side should be filled");
            assert!(zero[row_idx][1], "Digit 0 left side should be filled");
            assert!(!zero[row_idx][2], "Digit 0 center should be empty");
            assert!(!zero[row_idx][3], "Digit 0 center should be empty");
            assert!(zero[row_idx][4], "Digit 0 right side should be filled");
            assert!(zero[row_idx][5], "Digit 0 right side should be filled");
        }
    }
}
