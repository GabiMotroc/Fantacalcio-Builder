pub use self::{home::*, login::*, register::*};

pub mod login;
pub mod home;
pub mod register;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    Sighup,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Login => "/login",
            Self::Sighup => "/sighup",
        }
    }
}