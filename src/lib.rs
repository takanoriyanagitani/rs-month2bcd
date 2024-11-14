#[repr(C)]
pub enum Month {
    Jan = 0x01,
    Feb = 0x02,
    Mar = 0x03,
    Apr = 0x04,
    May = 0x05,
    Jun = 0x06,
    Jul = 0x07,
    Aug = 0x08,
    Sep = 0x09,
    Oct = 0x0a,
    Nov = 0x0b,
    Dec = 0x0c,
}

#[repr(C)]
pub enum BcdMonth {
    Jan = 0x01,
    Feb = 0x02,
    Mar = 0x03,
    Apr = 0x04,
    May = 0x05,
    Jun = 0x06,
    Jul = 0x07,
    Aug = 0x08,
    Sep = 0x09,
    Oct = 0x10,
    Nov = 0x11,
    Dec = 0x12,
}

static INVALID: u8 = 0x20;

static MONTH_TO_BCD: [u8; 16] = [
    INVALID,
    BcdMonth::Jan as u8,
    BcdMonth::Feb as u8,
    BcdMonth::Mar as u8,
    BcdMonth::Apr as u8,
    BcdMonth::May as u8,
    BcdMonth::Jun as u8,
    BcdMonth::Jul as u8,
    BcdMonth::Aug as u8,
    BcdMonth::Sep as u8,
    BcdMonth::Oct as u8,
    BcdMonth::Nov as u8,
    BcdMonth::Dec as u8,
    INVALID,
    INVALID,
    INVALID,
];

pub fn month_to_binary_coded_decimal(m: Month) -> u8 {
    let u: u8 = (m as u8) & 0x0f;
    MONTH_TO_BCD[u as usize]
}

#[allow(unsafe_code)]
#[no_mangle]
pub extern "C" fn mon2bcd(m: Month) -> u8 {
    month_to_binary_coded_decimal(m)
}

#[cfg(test)]
mod test {
    mod month_to_binary_coded_decimal {

        use crate::Month;

        macro_rules! create_test {
            ($fname: ident, $input: expr, $expected: expr) => {
                #[test]
                fn $fname() {
                    let got: u8 = crate::month_to_binary_coded_decimal($input);
                    assert_eq!(got, $expected);
                }
            };
        }

        create_test!(jan, Month::Jan, 0x01);
        create_test!(feb, Month::Feb, 0x02);
        create_test!(mar, Month::Mar, 0x03);
        create_test!(apr, Month::Apr, 0x04);
        create_test!(may, Month::May, 0x05);
        create_test!(jun, Month::Jun, 0x06);
        create_test!(jul, Month::Jul, 0x07);
        create_test!(aug, Month::Aug, 0x08);
        create_test!(sep, Month::Sep, 0x09);
        create_test!(oct, Month::Oct, 0x10);
        create_test!(nov, Month::Nov, 0x11);
        create_test!(dec, Month::Dec, 0x12);
    }
}
