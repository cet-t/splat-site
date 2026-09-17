mod filter;
mod query;
mod response;

use axum::response::Html;
pub use query::*;
pub use response::*;
use urng::{Choice, SplitMix32};

use crate::{
    data::EmbedQuery,
    helper::{SITE_URL, error_text, escape_html},
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
        (stringify!(SITE_URL).to_owned(), SITE_URL),
        (stringify!(title).to_owned(), &title),
        (stringify!(w_meta).to_owned(), &w_meta),
        (stringify!(w_body).to_owned(), &w_body),
        (stringify!(colour).to_owned(), &colour),
    ];

    Html(::strfmt::strfmt(HTML, &vars.into()).unwrap_or(error_text()))
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

    let r = if let Some(raw) = cache.get_weapons(client).await {
        let n = n.unwrap_or(1).clamp(1, 8) as usize;
        let mut rng = SplitMix32::default();
        let mut weapons = Vec::with_capacity(n);

        for _ in 0..n {
            weapons.push(rng.choice(raw));
        }

        build_html_weapon(&weapons, colour)
    } else {
        Html(error_text())
    };

    Ok(r)
}
