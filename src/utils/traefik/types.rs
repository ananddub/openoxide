use crate::string_enum;

string_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CertificateType {
        default = None;
        LetsEncrypt => "LETSENCRYPT",
        Custom => "CUSTOM",
        None => "NONE",
    }
}
