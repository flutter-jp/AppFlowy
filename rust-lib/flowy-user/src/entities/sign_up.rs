use crate::entities::{UserEmail, UserName, UserPassword};
use flowy_derive::ProtoBuf;
use std::convert::TryInto;

#[derive(ProtoBuf, Default)]
pub struct SignUpRequest {
    #[pb(index = 1)]
    pub email: String,

    #[pb(index = 2)]
    pub name: String,

    #[pb(index = 3)]
    pub password: String,
}
impl TryInto<SignUpParams> for SignUpRequest {
    type Error = String;

    fn try_into(self) -> Result<SignUpParams, Self::Error> {
        let email = UserEmail::parse(self.email)?;
        let name = UserName::parse(self.name)?;
        let password = UserPassword::parse(self.password)?;
        Ok(SignUpParams {
            email: email.0,
            name: name.0,
            password: password.0,
        })
    }
}

#[derive(ProtoBuf, Default)]
pub struct SignUpParams {
    #[pb(index = 1)]
    pub email: String,

    #[pb(index = 2)]
    pub name: String,

    #[pb(index = 3)]
    pub password: String,
}

#[derive(ProtoBuf, Debug, Default)]
pub struct SignUpResponse {
    #[pb(index = 1)]
    pub is_success: bool,
}

impl SignUpResponse {
    pub fn new(is_success: bool) -> Self { Self { is_success } }
}