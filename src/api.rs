use super::utils::to_vec;
use chrono::{Datelike, Local, NaiveDate};
use request::{call_api, ApiOptions};
use std::num::ParseIntError;

pub mod request;

#[derive(Debug)]
pub enum IsDayOffApiError {
    ParseIntError(ParseIntError),
    RequestError(reqwest::Error),
}

pub struct IsDayOffApi {}

impl IsDayOffApi {
    pub async fn today() -> Result<u32, IsDayOffApiError> {
        let now = Local::now();

        Ok(Self::request(ApiOptions::WithYMD {
            year: now.year(),
            month: Some(now.month()),
            day: Some(now.day()),
        })
        .await?
        .first()
        .unwrap()
        .clone())
    }

    pub async fn month(
        year: Option<i32>,
        month: Option<u32>,
    ) -> Result<Vec<u32>, IsDayOffApiError> {
        let now = Local::now();

        let year = year.unwrap_or(now.year());
        let month = month.unwrap_or(now.month());

        Self::request(ApiOptions::WithYMD {
            year,
            month: Some(month),
            day: None,
        })
        .await
    }

    pub async fn year(year: i32) -> Result<Vec<u32>, IsDayOffApiError> {
        Self::request(ApiOptions::WithYMD {
            year,
            month: None,
            day: None,
        })
        .await
    }

    pub async fn date(year: i32, month: u32, day: u32) -> Result<u32, IsDayOffApiError> {
        Ok(Self::request(ApiOptions::WithYMD {
            year,
            month: Some(month),
            day: Some(day),
        })
        .await?
        .first()
        .unwrap()
        .clone())
    }

    pub async fn period(start: &NaiveDate, end: &NaiveDate) -> Result<Vec<u32>, IsDayOffApiError> {
        Self::request(ApiOptions::WithStartEnd { start, end }).await
    }

    async fn request<'a>(options: ApiOptions<'a>) -> Result<Vec<u32>, IsDayOffApiError> {
        let result = call_api(options)
            .await
            .map_err(IsDayOffApiError::RequestError)?;

        Ok(to_vec(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn today() {
        let now = Local::now();
        let result = IsDayOffApi::today().await.unwrap();
        let test_result = call_api(ApiOptions::WithYMD {
            year: now.year(),
            month: Some(now.month()),
            day: Some(now.day()),
        })
        .await
        .unwrap();

        assert_eq!(test_result.parse::<u32>().unwrap(), result);
    }

    #[tokio::test]
    async fn month() {
        let now = Local::now();

        let options = ((), 08, 2023, (2023, 08));

        let test_results = (
            to_vec(
                call_api(ApiOptions::WithYMD {
                    year: now.year(),
                    month: Some(now.month()),
                    day: None,
                })
                .await
                .unwrap(),
            ),
            to_vec(
                call_api(ApiOptions::WithYMD {
                    year: now.year(),
                    month: Some(options.1),
                    day: None,
                })
                .await
                .unwrap(),
            ),
            to_vec(
                call_api(ApiOptions::WithYMD {
                    year: options.2,
                    month: Some(now.month()),
                    day: None,
                })
                .await
                .unwrap(),
            ),
            to_vec(
                call_api(ApiOptions::WithYMD {
                    year: options.3 .0,
                    month: Some(options.3 .1),
                    day: None,
                })
                .await
                .unwrap(),
            ),
        );
        let results = (
            IsDayOffApi::month(None, None).await.unwrap(),
            IsDayOffApi::month(None, Some(options.1)).await.unwrap(),
            IsDayOffApi::month(Some(options.2), None).await.unwrap(),
            IsDayOffApi::month(Some(options.3 .0), Some(options.3 .1))
                .await
                .unwrap(),
        );

        assert_eq!(test_results.0, results.0);
        assert_eq!(test_results.1, results.1);
        assert_eq!(test_results.2, results.2);
        assert_eq!(test_results.3, results.3);
    }

    #[tokio::test]
    async fn year() {
        let result = IsDayOffApi::year(2023).await.unwrap();
        let test_result = to_vec(
            call_api(ApiOptions::WithYMD {
                year: 2023,
                month: None,
                day: None,
            })
            .await
            .unwrap(),
        );

        assert_eq!(test_result, result);
    }

    #[tokio::test]
    async fn date() {
        let result = IsDayOffApi::date(2024, 09, 30).await.unwrap();
        let test_result = call_api(ApiOptions::WithYMD {
            year: 2024,
            month: Some(09),
            day: Some(30),
        })
        .await
        .unwrap();

        assert_eq!(test_result.parse::<u32>().unwrap(), result);
    }

    #[tokio::test]
    async fn period() {
        let options = (
            (
                NaiveDate::from_ymd_opt(2024, 09, 25).unwrap(),
                NaiveDate::from_ymd_opt(2024, 09, 28).unwrap(),
            ),
            NaiveDate::from_ymd_opt(2025, 09, 28).unwrap(),
        );

        let results = IsDayOffApi::period(&options.0 .0, &options.0 .1)
            .await
            .unwrap();
        let test_results = to_vec(
            call_api(ApiOptions::WithStartEnd {
                start: &options.0 .0,
                end: &options.0 .1,
            })
            .await
            .unwrap(),
        );

        println!("{test_results:?}");

        assert_eq!(test_results, results);
    }
}
