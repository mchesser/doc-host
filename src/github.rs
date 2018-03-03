use std::marker::PhantomData;

use rocket::http::Status;
use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub tree_id: String,
    pub message: String,
    pub author: Author,
    pub url: String,
    pub distinct: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushData {
    #[serde(rename = "ref")]
    pub git_ref: String,
    pub before: String,
    pub after: String,
    pub commits: Vec<Commit>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingData {
    pub zen: String,
    pub hook_id: u64,
}

pub struct Event<T>(PhantomData<T>);

pub trait EventType {
    const NAME: &'static str;
}

pub struct Push;

impl EventType for Push {
    const NAME: &'static str = "push";
}

pub struct Ping;

impl EventType  for Ping {
    const NAME: &'static str = "ping";
}

pub type PushEvent = Event<Push>;
pub type PingEvent = Event<Ping>;

impl<'a, 'r, T: EventType> FromRequest<'a, 'r> for Event<T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Event<T>, ()> {
        let keys: Vec<_> = request.headers().get("X-GitHub-Event").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        if keys[0] != T::NAME {
            return Outcome::Forward(());
        }

        Outcome::Success(Event(PhantomData))
    }
}
