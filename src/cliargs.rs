#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[clap(long, default_value("false"))]
    release: bool,
}

const DEV_PORT: u16 = 3000;
const REL_PORT: u16 = DEV_PORT + 1;

impl Cli {
    pub fn port(&self) -> anyhow::Result<u16> {
        let port = if self.release { REL_PORT } else { DEV_PORT };
        if port_check::is_port_reachable(format!("127.0.0.1:{}", port)) {
            anyhow::bail!("`{}` is invalid port.", port)
        } else {
            Ok(port)
        }
    }
}
