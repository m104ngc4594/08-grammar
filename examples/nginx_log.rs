// parse:
// 93.184.216.34 - - [07/Mar/2014:16:05:49 +0800] "GET /api/v1/user/login HTTP/1.1" 200 2 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/35.0.1916.153 Safari/537.36"

use anyhow::{anyhow, Result};
use regex::Regex;

#[allow(unused)]
#[derive(Debug)]
struct NginxLog {
    addr: String,
    datetime: String,
    method: String,
    url: String,
    protocol: String,
    status: u16,
    body_bytes: u64,
    referer: String,
    user_agent: String,
}

fn main() -> Result<()> {
    let s = r#"93.184.216.34 - - [07/Mar/2014:16:05:49 +0800] "GET /api/v1/user/login HTTP/1.1" 200 2 "-" "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/35.0.1916.153 Safari/537.36""#;
    let log = parse_nginx_log(s)?;
    println!("{:?}", log);
    Ok(())
}

fn parse_nginx_log(s: &str) -> Result<NginxLog> {
    let re = Regex::new(
        r#"^(?<ip>\S+)\s+\S+\s+\S+\s+\[(?<date>[^\]]+)\]\s+"(?<method>\S+)\s+(?<url>\S+)\s+(?<proto>[^"]+)"\s+(?<status>\d+)\s+(?<bytes>\d+)\s+"(?<referer>[^"]+)"\s+"(?<ua>[^"]+)"$"#,
    )?;
    let cap = re.captures(s).ok_or(anyhow!("parse error"))?;

    let addr = cap
        .name("ip")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse ip error"))?;
    let datetime = cap
        .name("date")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse datetime error"))?;
    let method = cap
        .name("method")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse method error"))?;
    let url = cap
        .name("url")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse url error"))?;
    let protocol = cap
        .name("proto")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse protocol error"))?;
    let status = cap
        .name("status")
        .map(|m| m.as_str().parse::<u16>())
        .ok_or(anyhow!("parse status error"))??;
    let body_bytes = cap
        .name("bytes")
        .map(|m| m.as_str().parse::<u64>())
        .ok_or(anyhow!("parse body_bytes error"))??;
    let referer = cap
        .name("referer")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse referer error"))?;
    let user_agent = cap
        .name("ua")
        .map(|m| m.as_str().to_string())
        .ok_or(anyhow!("parse user_agent error"))?;

    Ok(NginxLog {
        addr,
        datetime,
        method,
        url,
        protocol,
        status,
        body_bytes,
        referer,
        user_agent,
    })
}
