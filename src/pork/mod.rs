pub(crate) mod authenticate;
mod create;
pub(crate) mod modify;

pub const AUTHENTICATE_PORKBUN_HOST: &str = "https://api-ipv4.porkbun.com";
pub const PORKBUN_HOST: &str = "https://api-ipv4.porkbun.com";


enum PorkbunError {
    PorkbunError //ToDo: improve error handling
}