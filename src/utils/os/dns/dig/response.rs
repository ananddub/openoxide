use std::net::IpAddr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigResponse {
    pub raw: String,
    pub addresses: Vec<IpAddr>,
    pub values: Vec<String>,
}

impl DigResponse {
    pub(super) fn parse(raw: String) -> Self {
        let values = raw
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let addresses = values
            .iter()
            .filter_map(|value| value.trim_end_matches('.').parse().ok())
            .collect();
        Self {
            raw,
            addresses,
            values,
        }
    }
}
