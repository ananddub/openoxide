mod cloudflare;
mod hostinger;
mod infomaniak;
mod ovh;
mod porkbun;
mod route53;
mod spaceship;

#[cfg(test)]
pub(crate) mod harness;

pub use cloudflare::Cloudflare;
pub use hostinger::Hostinger;
pub use infomaniak::Infomaniak;
pub use ovh::Ovh;
pub use porkbun::Porkbun;
pub use route53::Route53;
pub use spaceship::Spaceship;
