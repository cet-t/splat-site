mod mode;
mod query;
mod response;
mod rule;
mod sche;

use axum::response::Html;
use chrono::DateTime;
use chrono::FixedOffset;

pub use self::mode::*;
pub use self::query::*;
pub use self::response::*;
pub use self::rule::*;
pub use self::sche::*;

use crate::data::{EmbedQuery, ScheduleInput};
use crate::helper::{SITE_URL, error_text, escape_html};
use crate::state::AppState;

const HTML: &str = include_str!("../../../assets/html/stage_ogp.html");

pub fn build_url(mode: self::Mode, sche: self::Schedule) -> String {
    format!("https://spla3.yuu26.com/api/{mode}/{sche}")
}

pub async fn enquiry(client: reqwest::Client, url: String) -> anyhow::Result<self::RawResponse> {
    let res = crate::get!(client, url);
    Ok(serde_json::from_str(&res)?)
}

fn build_html_sche(info: &RawScheduleInfo, query: EmbedQuery) -> Html<String> {
    let format_dt = |dt: &DateTime<FixedOffset>| {
        escape_html(dt.naive_local().format("%m/%d %H:%M").to_string())
    };

    let desc = {
        let stage_names: Vec<_> = info
            .stages
            .iter()
            .map(|s| format!("- {}", s.name))
            .collect();
        let time = format!(
            "{} - {}",
            format_dt(&info.start_time),
            format_dt(&info.end_time)
        );
        format!("{time}\n{}", stage_names.join("\n"))
    };

    let imgs_meta = {
        let metas: Vec<_> = info
            .stages
            .iter()
            .map(|s| {
                format!(
                    "<meta property=\"og:image\" content=\"{}\">",
                    escape_html(&s.image_url)
                )
            })
            .collect();
        metas.join("\n")
    };

    let imgs_src = {
        let srcs: Vec<_> = info
            .stages
            .iter()
            .map(|s| {
                format!(
                    "<img src=\"{}\" alt=\"stage\" style=\"max-width:100%\">",
                    escape_html(&s.image_url)
                )
            })
            .collect();
        srcs.join("\n")
    };

    let title = escape_html(info.rule);
    let desc = escape_html(desc);
    let colour = escape_html(query.colour.unwrap_or(info.rule.to_rgb()));

    let vars = [
        (stringify!(SITE_URL).to_owned(), SITE_URL),
        (stringify!(title).to_owned(), &title),
        (stringify!(desc).to_owned(), &desc),
        (stringify!(imgs_meta).to_owned(), &imgs_meta),
        (stringify!(colour).to_owned(), &colour),
        (stringify!(imgs_src).to_owned(), &imgs_src),
    ];

    Html(::strfmt::strfmt(HTML, &vars.into()).unwrap_or(error_text()))
}

// --- core ---

async fn get_info(
    AppState { client, cache }: AppState,
    schedule: ScheduleInput,
    mode: Mode,
    query: EmbedQuery,
) -> anyhow::Result<Html<String>> {
    let mut cache = cache.lock().await;
    let info = cache.fetch_schedule(client, mode).await?;

    let info = match schedule {
        ScheduleInput::Now => info.first(),
        ScheduleInput::Next => info.get(query.n.unwrap_or(1) as usize),
    };

    Ok(build_html_sche(info.ok_or(anyhow::anyhow!(""))?, query))
}
