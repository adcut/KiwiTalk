use talk_loco_command::{request, response};

use crate::LocoCommandSession;

use super::async_client_method;

#[derive(Debug)]
pub struct BookingClient<'a>(pub &'a LocoCommandSession);

impl BookingClient<'_> {
    async_client_method!(get_conf, "GETCONF", request::booking::GetConfReq => response::booking::GetConfRes);
}
