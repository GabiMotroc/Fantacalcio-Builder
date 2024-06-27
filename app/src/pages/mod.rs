pub use self::{home::*, login::*, register::*};

pub mod login;
pub mod home;
pub mod register;
pub mod test;
pub mod hello_world;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    Register,
    Test,
    HelloWorld,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Login => "/login",
            Self::Register => "/register",
            Self::Test => "/test",
            Page::HelloWorld => "/hello-world"
        }
    }
}