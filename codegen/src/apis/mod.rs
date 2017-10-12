use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod community_content_api;
pub use self::community_content_api::{ CommunityContentApi, CommunityContentApiClient };
mod destiny2_api;
pub use self::destiny2_api::{ Destiny2Api, Destiny2ApiClient };
mod forum_api;
pub use self::forum_api::{ ForumApi, ForumApiClient };
mod group_v2_api;
pub use self::group_v2_api::{ GroupV2Api, GroupV2ApiClient };
mod preview_api;
pub use self::preview_api::{ PreviewApi, PreviewApiClient };
mod trending_api;
pub use self::trending_api::{ TrendingApi, TrendingApiClient };
mod user_api;
pub use self::user_api::{ UserApi, UserApiClient };

pub mod configuration;
pub mod client;
