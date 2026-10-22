use os::string_enum;

string_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum CertificateType {
        default = None;
        LetsEncrypt => "LETSENCRYPT",
        Custom => "CUSTOM",
        None => "NONE",
    }
}
