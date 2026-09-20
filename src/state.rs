use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Datelike, Timelike, Utc};
use tokio::sync::Mutex;

use crate::{
    helper::error_text,
    splatoon::{
        schedule::{self, Mode, RawScheduleInfo, build_url},
        weapon::{self, RawWeaponInfo},
    },
};

const UPD8_H: [u32; 12] = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];

#[derive(Clone)]
pub struct AppCache {
    pub schedule: HashMap<Mode, Vec<RawScheduleInfo>>,
    pub schedule_fetched: HashMap<Mode, Option<chrono::DateTime<chrono::Utc>>>,

    pub weapons: Option<Vec<RawWeaponInfo>>,
}

impl AppCache {
    pub fn new() -> Self {
        Self {
            schedule: HashMap::with_capacity(2),
            schedule_fetched: HashMap::with_capacity(2),
            weapons: None,
        }
    }

    pub(crate) fn calc_block(dt: DateTime<Utc>) -> i16 {
        let mut block = -1;

        for (i, hour) in UPD8_H.into_iter().enumerate().rev() {
            if dt.hour() <= hour {
                block = i as i16;
                break;
            }
        }

        block
    }

    pub(crate) fn is_diff_block(now: DateTime<Utc>, last: DateTime<Utc>) -> bool {
        let now_block = Self::calc_block(now);
        let last_block = Self::calc_block(last);
        now_block != last_block
    }

    pub async fn fetch_schedule(
        &mut self,
        client: reqwest::Client,
        mode: Mode,
    ) -> anyhow::Result<&Vec<RawScheduleInfo>> {
        let schedule_opt = self.schedule.get_mut(&mode);
        let last_dt_opt = self.schedule_fetched.get_mut(&mode);

        fn fetchable(last_dt: DateTime<Utc>) -> bool {
            let now = Utc::now();

            let is_diff_block = AppCache::is_diff_block(now, last_dt);
            let is_yesterday = last_dt.day() < now.day();

            is_diff_block || is_yesterday
        }

        async fn fetch(
            mode: Mode,
            client: reqwest::Client,
        ) -> anyhow::Result<(DateTime<Utc>, Vec<RawScheduleInfo>)> {
            let url = build_url(mode, schedule::Schedule::After);
            let res = schedule::enquiry(client, url).await?;
            Ok((Utc::now(), res.results))
        }

        if let Some(last_dt) = last_dt_opt.and_then(Option::as_mut)
            && let Some(schedule) = schedule_opt
        {
            if fetchable(*last_dt)
                && let Ok((dt, fetched)) = fetch(mode, client).await
            {
                *last_dt = dt;
                *schedule = fetched;
            }
        } else {
            let (dt, fetched) = fetch(mode, client).await?;

            self.schedule.insert(mode, fetched);
            self.schedule_fetched.insert(mode, Some(dt));
        }

        self.schedule
            .get(&mode)
            .ok_or(anyhow::anyhow!("{}", error_text()))
    }

    pub async fn get_weapons(&mut self, client: reqwest::Client) -> &Option<Vec<RawWeaponInfo>> {
        if self.weapons.is_none()
            && let Ok(raw) = weapon::enquiry(client).await
        {
            self.weapons = Some(raw);
        }

        &self.weapons
    }
}

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub cache: Arc<Mutex<AppCache>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            cache: Arc::new(Mutex::new(AppCache::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Utc};

    #[test]
    #[allow(deprecated)]
    fn calc_block_works() {
        let offset = *Utc::now().offset();

        let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::new(
                NaiveDate::from_ymd(2026, 9, 21),
                NaiveTime::from_hms(0, 0, 0),
            ),
            offset,
        );

        // [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
        assert_eq!(AppCache::calc_block(dt), 11);
    }
}
