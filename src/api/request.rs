use crate::utils::format_date;
use chrono::NaiveDate;
use reqwest::Client;

pub(crate) enum ApiOptions<'a> {
    WithYMD {
        year: i32,
        month: Option<u32>,
        day: Option<u32>,
    },
    WithStartEnd {
        start: &'a NaiveDate,
        end: &'a NaiveDate,
    },
}

pub(crate) async fn call_api<'a>(options: ApiOptions<'a>) -> Result<String, reqwest::Error> {
    let mut url = "https://isdayoff.ru/api".to_string();
    let client = Client::new();

    let mut path = "".to_string();

    if let ApiOptions::WithYMD { year, day, month } = options {
        path.push_str(format!("/getData?year={year}").as_str());

        if let Some(month) = month {
            path.push_str(format!("&month={month}").as_str());
        }

        if let Some(day) = day {
            path.push_str(format!("&day={day}").as_str());
        }
    }

    if let ApiOptions::WithStartEnd { start, end } = options {
        let str = format!(
            "/getData?date1={}&date2={}",
            format_date(start),
            format_date(end)
        );

        path.push_str(str.as_str());
    }

    url.push_str(&path);

    client
        .get(&url)
        .header(
            "User-Agent",
            format!(
                "isdayoff-rust-lib/{} (telegram: @tyominomi)",
                env!("CARGO_PKG_VERSION")
            ),
        )
        .send()
        .await?
        .text()
        .await
}

#[cfg(test)]
mod tests {
    use super::{call_api, ApiOptions};
    use chrono::NaiveDate;

    #[tokio::test]
    async fn test_call_api_full_date() {
        let str = call_api(ApiOptions::WithYMD {
            year: 2024,
            month: Some(09),
            day: Some(29),
        });

        assert_eq!("1", str.await.unwrap());
    }

    #[tokio::test]
    async fn test_call_api_only_year_month() {
        let str = call_api(ApiOptions::WithYMD {
            year: 2024,
            month: Some(09),
            day: None,
        });

        assert_eq!("100000110000011000001100000110", str.await.unwrap());
    }

    #[tokio::test]
    async fn test_call_api_only_year_or_empty() {
        // 2024 year val
        let true_str = "111111110000110000011000001100000110000011000001100001110000011000011100000110000011000001100000110000011000001100000011110011000111100000110000011000001100000110010011000001100000110000011000001100000110000011000001100000110000011000001100000110000011000001100000110000011000001100000110000011000001100000011000011000001100000110000011000001100000110000011000000111";

        let str = call_api(ApiOptions::WithYMD {
            year: 2024,
            month: None,
            day: None,
        });

        assert_eq!(true_str, str.await.unwrap());
    }

    #[tokio::test]
    async fn test_call_api_with_start_and_end() {
        let (start, end) = (
            &NaiveDate::parse_from_str("2024-09-20", "%Y-%m-%d").unwrap(),
            &NaiveDate::parse_from_str("2024-09-29", "%Y-%m-%d").unwrap(),
        );

        let str = call_api(ApiOptions::WithStartEnd { start, end });

        assert_eq!("0110000011", str.await.unwrap());
    }
}
