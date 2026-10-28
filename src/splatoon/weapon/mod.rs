mod filter;
mod query;
mod response;

use axum::response::Html;
pub use query::*;
pub use response::*;
use trng::Rng;
use trng_a::Sfc64;
use trng_sample::Choice;

use crate::{
    data::EmbedQuery,
    helper::{SITE_URL, error_html, error_text, escape_html},
    rgb::Rgb,
    state::AppState,
};

const URL: &str = "https://stat.ink/api/v3/weapon";

const HTML: &str = include_str!("../../../assets/html/weapon_opg.html");

fn build_html_weapon(info: &[&RawWeaponInfo], colour: Option<Rgb>) -> Html<String> {
    let title = escape_html("武器抽選");
    let colour = escape_html(colour.unwrap_or(Rgb([0xff, 0xff, 0x10])));
    let weapons: Vec<_> = info.iter().map(|&w| w.name.ja_JP.clone()).collect();

    let w_meta = {
        let weapons: Vec<_> = weapons
            .iter()
            .enumerate()
            .map(|(i, w)| format!("{}. {}", i + 1, escape_html(w)))
            .collect();
        weapons.join("\n")
    };
    let w_body = {
        let weapons: Vec<_> = weapons
            .iter()
            .map(|w| format!("<li>{}</li>", escape_html(w)))
            .collect();
        weapons.join(" ")
    };

    let vars = [
        (stringify!(SITE_URL), SITE_URL),
        (stringify!(title), &title),
        (stringify!(w_meta), &w_meta),
        (stringify!(w_body), &w_body),
        (stringify!(colour), &colour),
    ]
    .map(|(k, v)| (k.to_owned(), v));

    strfmt::strfmt(HTML, &vars.into())
        .map(Html)
        .unwrap_or(error_html())
}

pub async fn enquiry(client: reqwest::Client) -> anyhow::Result<self::RawResponse> {
    let res = crate::get!(client, URL.to_owned());
    Ok(serde_json::from_str(&res)?)
}

pub async fn get_info(
    AppState { client, cache }: AppState,
    EmbedQuery { n, colour, .. }: EmbedQuery,
) -> anyhow::Result<Html<String>> {
    let mut cache = cache.lock().await;

    let Some(raw) = cache.get_weapons(client).await else {
        return Ok(Html(error_text()));
    };

    let mut rng = {
        let ts = chrono::Local::now();
        Sfc64::from(ts.timestamp() as u64)
    };

    let n = n.map(|n| n.clamp(1, 8)).unwrap_or(1);
    let mut weapons = Vec::with_capacity(n as usize);

    for _ in 0..n {
        if let Some(w) = rng.choice(raw) {
            weapons.push(w);
        }
    }

    Ok(build_html_weapon(&weapons, colour))
}
