use chrono::Datelike;

pub(crate) fn format_date(date: &impl Datelike) -> String {
    let y_str = format!("0000{}", date.year());
    let m_str = format!("00{}", date.month());
    let d_str = format!("00{}", date.day());

    format!(
        "{}{}{}",
        y_str.chars().skip(y_str.len() - 4).collect::<String>(),
        m_str.chars().skip(m_str.len() - 2).collect::<String>(),
        d_str.chars().skip(d_str.len() - 2).collect::<String>()
    )
}

pub(crate) fn to_vec(result: String) -> Vec<u32> {
    result
        .chars()
        .into_iter()
        .map(|char| char.to_digit(10).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDate};

    #[test]
    fn test_format_date() {
        let correct_str = "20240919";

        let formatted_date1 = format_date(&NaiveDate::from_ymd_opt(2024, 09, 19).unwrap());
        let formatted_date2 =
            format_date(&NaiveDate::parse_from_str("2024-09-19", "%Y-%m-%d").unwrap());
        let formatted_date3 =
            format_date(&DateTime::parse_from_rfc3339("2024-09-19T00:00:00Z").unwrap());

        assert_eq!(formatted_date1, correct_str);
        assert_eq!(formatted_date2, correct_str);
        assert_eq!(formatted_date3, correct_str);
    }
}
