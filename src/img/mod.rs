use std::{fs, path::PathBuf};

use image::ImageFormat;

async fn save_to_file(url: &str) -> anyhow::Result<PathBuf> {
    let resp = {
        // println!("{url}");
        reqwest::get(url).await?.error_for_status()?
    };

    let mut name = {
        let s = url
            .split("/")
            .last()
            .ok_or(anyhow::anyhow!("no file name in url"))?;
        let q = s.split("?").collect::<Vec<_>>();
        q.first()
            .ok_or(anyhow::anyhow!("no file name in url"))?
            .to_string()
    };

    if !name.contains('.') {
        let ext = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .and_then(|ct| ct.split('/').nth(1))
            .map(|sub| sub.split(';').next().unwrap_or(sub))
            .unwrap_or("png");
        name = format!("{name}.{ext}");
    }

    let dir = PathBuf::from("assets/imgs");
    fs::create_dir_all(&dir)?;
    let path = dir.join(name);

    let bytes = resp.bytes().await?;
    fs::write(&path, &bytes)?;

    Ok(path)
}

pub async fn combine(url0: &str, url1: &str) -> anyhow::Result<PathBuf> {
    let path0 = save_to_file(url0).await?;
    let img0 = combine_img::trim_img(path0.clone(), Some(|w| w / 2), None)?;

    let path1 = save_to_file(url1).await?;
    let img1 = combine_img::trim_img(path1.clone(), Some(|w| w / 2), None)?;

    let res = combine_img::combine2(&img0, &img1)?;

    let dir = PathBuf::from("assets/imgs/res");
    fs::create_dir_all(&dir)?;
    let out_path = dir.join(format!(
        "{}_{}.png",
        path0
            .file_name()
            .ok_or(anyhow::anyhow!(""))?
            .to_string_lossy(),
        path1
            .file_name()
            .ok_or(anyhow::anyhow!(""))?
            .to_string_lossy()
    ));
    res.save_with_format(&out_path, ImageFormat::Png)?;

    Ok(out_path)
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     const FILE0: &str = "https://api.lp1.av5ja.srv.nintendo.net/resources/prod/v3/stage_img/icon/low_resolution/692365fa7e56cf19cfa403a8546e69cf60fd9ca2171bde66cdaa53dc0e736ac9_1.png?Expires=1799539200&Signature=dJnztAlnkQzi5APX6j5RBQHYjdROrEdswpdgBkxaKjkSAZV8lTsBmBZw3OkoqK22OfbQILJ1lWgtvaQQEYBzS5xvrvktGAA2~xMZaBV1md~QVFbrBep0DqO25UIdZGufM~VLshi3Dhx3yEagi8HBZj10QvdcFcwrrMQpNAMOl6kZ9-jaau9Zm6SjY0PhHY7jRcF2xD8g3txq5EQUuMYK9ItIVLc4kjr69vX0tJo8Lxdyz5eRig7y9w2Ca2wfAx5I8dxQ-kKFDxUjRw0O2qhlFH0Bjk~1Rlo56BOxi9zX4Y3bULCybM4CH5I6vcsvlfQ7EidhlJIV6h~zCU~cWCvpSw__&Key-Pair-Id=KNBS2THMRC385";
    //     const FILE1: &str = "https://api.lp1.av5ja.srv.nintendo.net/resources/prod/v3/stage_img/icon/low_resolution/2ba481293efc554ac217f21b6d56dd08f9d66e72b286f20714abd5ef1520f47a_1.png?Expires=1799539200&Signature=mpOCcm0l-zHrQhdqOnd4ueStjZYTxoS8vmuwrFkwUF2ykfVn~J5t1D51gY0CvusUbVb-G5FtxTCR99w-cozPYBgqdBHMT8a0-NDeSv2UpYi8wNzwmmPOWN2k1uXFG8hOd6nPBRW0BwIKPNWeCYcoD27EzAINukq7b7p0WYR5W9-rjOA25upZXRdn3O00qQ13Ti8KVpQA38aEEUl0KXrUQa8Xf7o38aDDOf-jRm6tJKsyeZgBDpVdgDG6uHeZ2GWODbtfmsLqRUVrf5mtTAAJAVi4yZMWQ5Q6sO~v3rnOaSvzJ8SyVEVAVcI~t0NvhMM8P~lUmviXuHZtszzCxrTdaA__&Key-Pair-Id=KNBS2THMRC385";
    //     let f0 = save_to_file(FILE0);
    //     assert!(f0.is_ok(), "{}", f0.unwrap_err().to_string());
    //     let f1 = save_to_file(FILE1);
    //     assert!(f1.is_ok(), "{}", f1.unwrap_err().to_string());
    //     let r = combine(FILE0, FILE1);
    //     assert!(r.is_ok(), "{}", r.unwrap_err().to_string());
    // }
}
