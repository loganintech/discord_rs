#![allow(dead_code)]

//External includes
use hyper::Uri;
use lazy_static::lazy_static;

//Modules
mod scopes;

//Internal includes
use crate::scopes::Scope;

lazy_static! {
    static ref API_ENDPOINT: Uri = "https://discordapp.com/api".parse::<Uri>().unwrap();
}

pub struct DiscordAPI {
    id: String,
    token: String,
    secret: String,
    redirect_uri: String,
    scopes: Vec<Scope>,
}

impl DiscordAPI {
    fn new(id: String, token: String, secret: String, redirect_uri: String, scopes: Vec<Scope>) -> Self {
        DiscordAPI {
            id,
            token,
            secret,
            redirect_uri,
            scopes,
        }
    }
}
