/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct CommunityContentApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> CommunityContentApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> CommunityContentApiClient<C> {
        CommunityContentApiClient {
            configuration: configuration,
        }
    }
}

pub trait CommunityContentApi {
    fn community_content_get_community_content(&self, media_filter: i32, page: i32, sort: i32) -> Box<Future<Item = ::models::InlineResponse2006, Error = Error>>;
    fn community_content_get_community_live_statuses(&self, page: i32, partnership_type: i32, sort: i32, mode_hash: i32, stream_locale: &str) -> Box<Future<Item = ::models::InlineResponse20050, Error = Error>>;
    fn community_content_get_community_live_statuses_for_clanmates(&self, page: i32, partnership_type: i32, sort: i32) -> Box<Future<Item = ::models::InlineResponse20050, Error = Error>>;
    fn community_content_get_community_live_statuses_for_friends(&self, page: i32, partnership_type: i32, sort: i32) -> Box<Future<Item = ::models::InlineResponse20050, Error = Error>>;
    fn community_content_get_featured_community_live_statuses(&self, page: i32, partnership_type: i32, sort: i32, stream_locale: &str) -> Box<Future<Item = ::models::InlineResponse20050, Error = Error>>;
    fn community_content_get_streaming_status_for_member(&self, membership_id: i64, membership_type: i32, partnership_type: i32) -> Box<Future<Item = ::models::InlineResponse20051, Error = Error>>;
}


impl<C: hyper::client::Connect>CommunityContentApi for CommunityContentApiClient<C> {
    fn community_content_get_community_content(&self, media_filter: i32, page: i32, sort: i32) -> Box<Future<Item = ::models::InlineResponse2006, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/CommunityContent/Get/{sort}/{mediaFilter}/{page}/", configuration.base_path, mediaFilter=media_filter, page=page, sort=sort);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::InlineResponse2006, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn community_content_get_community_live_statuses(&self, page: i32, partnership_type: i32, sort: i32, mode_hash: i32, stream_locale: &str) -> Box<Future<Item = ::models::InlineResponse20050, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("modeHash", &mode_hash.to_string())
            .append_pair("streamLocale", &stream_locale.to_string())
            .finish();
        let uri_str = format!("{}/CommunityContent/Live/All/{partnershipType}/{sort}/{page}/{}", configuration.base_path, query, page=page, partnershipType=partnership_type, sort=sort);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::InlineResponse20050, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn community_content_get_community_live_statuses_for_clanmates(&self, page: i32, partnership_type: i32, sort: i32) -> Box<Future<Item = ::models::InlineResponse20050, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/CommunityContent/Live/Clan/{partnershipType}/{sort}/{page}/", configuration.base_path, page=page, partnershipType=partnership_type, sort=sort);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::InlineResponse20050, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn community_content_get_community_live_statuses_for_friends(&self, page: i32, partnership_type: i32, sort: i32) -> Box<Future<Item = ::models::InlineResponse20050, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/CommunityContent/Live/Friends/{partnershipType}/{sort}/{page}/", configuration.base_path, page=page, partnershipType=partnership_type, sort=sort);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::InlineResponse20050, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn community_content_get_featured_community_live_statuses(&self, page: i32, partnership_type: i32, sort: i32, stream_locale: &str) -> Box<Future<Item = ::models::InlineResponse20050, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("streamLocale", &stream_locale.to_string())
            .finish();
        let uri_str = format!("{}/CommunityContent/Live/Featured/{partnershipType}/{sort}/{page}/{}", configuration.base_path, query, page=page, partnershipType=partnership_type, sort=sort);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::InlineResponse20050, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn community_content_get_streaming_status_for_member(&self, membership_id: i64, membership_type: i32, partnership_type: i32) -> Box<Future<Item = ::models::InlineResponse20051, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/CommunityContent/Live/Users/{partnershipType}/{membershipType}/{membershipId}/", configuration.base_path, membershipId=membership_id, membershipType=membership_type, partnershipType=partnership_type);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());



        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::InlineResponse20051, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}