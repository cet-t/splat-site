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

#[derive(Clone)]
pub struct AppCache {
    pub schedule: HashMap<Mode, Vec<RawScheduleInfo>>,
    pub schedule_fetched: HashMap<Mode, Option<chrono::DateTime<chrono::Utc>>>,

    pub weapons: Option<Vec<RawWeaponInfo>>,
}

impl AppCache {
    pub(crate) const UPD8_H: [i16; 12] = const {
        let mut hours = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
        hours.reverse();
        hours
    };

    pub fn new() -> Self {
        Self {
            schedule: HashMap::with_capacity(2),
            schedule_fetched: HashMap::with_capacity(2),
            weapons: None,
        }
    }

    pub(crate) fn calc_block(dt: DateTime<Utc>) -> i16 {
        let mut block = -1;

        for (i, hour) in Self::UPD8_H.into_iter().enumerate() {
            if dt.hour() <= hour as u32 {
                block = i as i16;
            }
        }

        block
    }

    pub(crate) fn is_another_block(now: DateTime<Utc>, last: DateTime<Utc>) -> bool {
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

        let now_dt = Utc::now();

        let fetchable = |last_dt: DateTime<Utc>| -> bool {
            let is_diff_block = AppCache::is_another_block(now_dt, last_dt);
            let is_yesterday = last_dt.day() < now_dt.day();

            is_diff_block || is_yesterday
        };

        let fetch = async |mode: Mode,
                           client: reqwest::Client|
               -> anyhow::Result<(DateTime<Utc>, Vec<RawScheduleInfo>)> {
            let url = build_url(mode, schedule::Schedule::After);
            let res = schedule::enquiry(client, url).await?;
            Ok((now_dt, res.results))
        };

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
            if let Ok((dt, fetched)) = fetch(mode, client).await {
                self.schedule.insert(mode, fetched);
                self.schedule_fetched.insert(mode, Some(dt));
            }
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
        let day = NaiveDate::from_ymd(2026, 9, 21);
        let hours = AppCache::UPD8_H;

        let _test = |h: u32, m: u32, exp: usize| {
            let dt = DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::new(day, NaiveTime::from_hms(h, m, 0)),
                offset,
            );
            assert_eq!(hours[AppCache::calc_block(dt) as usize], hours[exp]);
        };

        // [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
        //test(0, 0, 0);
        //test(1, 59, 1);
        //test(23, 59, 11);
    }
}
