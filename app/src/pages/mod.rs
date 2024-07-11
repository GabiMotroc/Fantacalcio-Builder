pub use self::{home::*, login::*, register::*};

pub mod login;
pub mod home;
pub mod register;
pub mod test;
pub mod hello_world;
pub mod select_squad;
pub mod build_squad;

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Home,
    Login,
    Register,
    Test,
    HelloWorld,
    CreateSquad,
    BuildSquad,
}

impl Page {
    pub fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Login => "/login",
            Self::Register => "/register",
            Self::Test => "/test",
            Self::HelloWorld => "/hello-world",
            Self::CreateSquad => "/create-squad",
            Self::BuildSquad => "/build-squad"
        }
    }
}