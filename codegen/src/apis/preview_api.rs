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

pub struct PreviewApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PreviewApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PreviewApiClient<C> {
        PreviewApiClient {
            configuration: configuration,
        }
    }
}

pub trait PreviewApi {
    fn destiny2_activate_talent_node(&self, ) -> Box<Future<Item = ::models::InlineResponse20015, Error = Error>>;
    fn destiny2_get_activity_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32, count: i32, mode: i32, page: i32) -> Box<Future<Item = ::models::InlineResponse20045, Error = Error>>;
    fn destiny2_get_clan_aggregate_stats(&self, group_id: i64, modes: &str) -> Box<Future<Item = ::models::InlineResponse20041, Error = Error>>;
    fn destiny2_get_clan_leaderboards(&self, group_id: i64, maxtop: i32, modes: &str, statid: &str) -> Box<Future<Item = ::models::InlineResponse20040, Error = Error>>;
    fn destiny2_get_destiny_aggregate_activity_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32) -> Box<Future<Item = ::models::InlineResponse20047, Error = Error>>;
    fn destiny2_get_historical_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32, dayend: String, daystart: String, groups: Vec<::models::DestinyHistoricalStatsDefinitionsDestinyStatsGroupType>, modes: Vec<::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType>, period_type: i32) -> Box<Future<Item = ::models::InlineResponse20043, Error = Error>>;
    fn destiny2_get_historical_stats_for_account(&self, destiny_membership_id: i64, membership_type: i32, groups: Vec<::models::DestinyHistoricalStatsDefinitionsDestinyStatsGroupType>) -> Box<Future<Item = ::models::InlineResponse20044, Error = Error>>;
    fn destiny2_get_leaderboards(&self, destiny_membership_id: i64, membership_type: i32, maxtop: i32, modes: &str, statid: &str) -> Box<Future<Item = ::models::InlineResponse20040, Error = Error>>;
    fn destiny2_get_leaderboards_for_character(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32, maxtop: i32, modes: &str, statid: &str) -> Box<Future<Item = ::models::InlineResponse20040, Error = Error>>;
    fn destiny2_get_unique_weapon_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32) -> Box<Future<Item = ::models::InlineResponse20046, Error = Error>>;
    fn destiny2_get_vendor(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32, vendor_hash: i32, components: Vec<::models::DestinyDestinyComponentType>) -> Box<Future<Item = ::models::InlineResponse20036, Error = Error>>;
    fn destiny2_get_vendors(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32, components: Vec<::models::DestinyDestinyComponentType>) -> Box<Future<Item = ::models::InlineResponse20035, Error = Error>>;
    fn destiny2_insert_socket_plug(&self, ) -> Box<Future<Item = ::models::InlineResponse20015, Error = Error>>;
    fn destiny2_search_destiny_entities(&self, search_term: &str, _type: &str, page: i32) -> Box<Future<Item = ::models::InlineResponse20042, Error = Error>>;
}


impl<C: hyper::client::Connect>PreviewApi for PreviewApiClient<C> {
    fn destiny2_activate_talent_node(&self, ) -> Box<Future<Item = ::models::InlineResponse20015, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri_str = format!("{}/Destiny2/Actions/Items/ActivateTalentNode/", configuration.base_path);

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
                let parsed: Result<::models::InlineResponse20015, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_activity_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32, count: i32, mode: i32, page: i32) -> Box<Future<Item = ::models::InlineResponse20045, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("count", &count.to_string())
            .append_pair("mode", &mode.to_string())
            .append_pair("page", &page.to_string())
            .finish();
        let uri_str = format!("{}/Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/Activities/{}", configuration.base_path, query, characterId=character_id, destinyMembershipId=destiny_membership_id, membershipType=membership_type);

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
                let parsed: Result<::models::InlineResponse20045, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_clan_aggregate_stats(&self, group_id: i64, modes: &str) -> Box<Future<Item = ::models::InlineResponse20041, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("modes", &modes.to_string())
            .finish();
        let uri_str = format!("{}/Destiny2/Stats/AggregateClanStats/{groupId}/{}", configuration.base_path, query, groupId=group_id);

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
                let parsed: Result<::models::InlineResponse20041, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_clan_leaderboards(&self, group_id: i64, maxtop: i32, modes: &str, statid: &str) -> Box<Future<Item = ::models::InlineResponse20040, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("maxtop", &maxtop.to_string())
            .append_pair("modes", &modes.to_string())
            .append_pair("statid", &statid.to_string())
            .finish();
        let uri_str = format!("{}/Destiny2/Stats/Leaderboards/Clans/{groupId}/{}", configuration.base_path, query, groupId=group_id);

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
                let parsed: Result<::models::InlineResponse20040, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_destiny_aggregate_activity_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32) -> Box<Future<Item = ::models::InlineResponse20047, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/AggregateActivityStats/", configuration.base_path, characterId=character_id, destinyMembershipId=destiny_membership_id, membershipType=membership_type);

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
                let parsed: Result<::models::InlineResponse20047, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_historical_stats(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32, dayend: String, daystart: String, groups: Vec<::models::DestinyHistoricalStatsDefinitionsDestinyStatsGroupType>, modes: Vec<::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType>, period_type: i32) -> Box<Future<Item = ::models::InlineResponse20043, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("dayend", &dayend.to_string())
            .append_pair("daystart", &daystart.to_string())
            .append_pair("groups", &groups.join(",").to_string())
            .append_pair("modes", &modes.join(",").to_string())
            .append_pair("periodType", &period_type.to_string())
            .finish();
        let uri_str = format!("{}/Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/{}", configuration.base_path, query, characterId=character_id, destinyMembershipId=destiny_membership_id, membershipType=membership_type);

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
                let parsed: Result<::models::InlineResponse20043, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_historical_stats_for_account(&self, destiny_membership_id: i64, membership_type: i32, groups: Vec<::models::DestinyHistoricalStatsDefinitionsDestinyStatsGroupType>) -> Box<Future<Item = ::models::InlineResponse20044, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("groups", &groups.join(",").to_string())
            .finish();
        let uri_str = format!("{}/Destiny2/{membershipType}/Account/{destinyMembershipId}/Stats/{}", configuration.base_path, query, destinyMembershipId=destiny_membership_id, membershipType=membership_type);

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
                let parsed: Result<::models::InlineResponse20044, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_leaderboards(&self, destiny_membership_id: i64, membership_type: i32, maxtop: i32, modes: &str, statid: &str) -> Box<Future<Item = ::models::InlineResponse20040, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("maxtop", &maxtop.to_string())
            .append_pair("modes", &modes.to_string())
            .append_pair("statid", &statid.to_string())
            .finish();
        let uri_str = format!("{}/Destiny2/{membershipType}/Account/{destinyMembershipId}/Stats/Leaderboards/{}", configuration.base_path, query, destinyMembershipId=destiny_membership_id, membershipType=membership_type);

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
                let parsed: Result<::models::InlineResponse20040, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_leaderboards_for_character(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32, maxtop: i32, modes: &str, statid: &str) -> Box<Future<Item = ::models::InlineResponse20040, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("maxtop", &maxtop.to_string())
            .append_pair("modes", &modes.to_string())
            .append_pair("statid", &statid.to_string())
            .finish();
        let uri_str = format!("{}/Destiny2/Stats/Leaderboards/{membershipType}/{destinyMembershipId}/{characterId}/{}", configuration.base_path, query, characterId=character_id, destinyMembershipId=destiny_membership_id, membershipType=membership_type);

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
                let parsed: Result<::models::InlineResponse20040, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_unique_weapon_history(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32) -> Box<Future<Item = ::models::InlineResponse20046, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let uri_str = format!("{}/Destiny2/{membershipType}/Account/{destinyMembershipId}/Character/{characterId}/Stats/UniqueWeapons/", configuration.base_path, characterId=character_id, destinyMembershipId=destiny_membership_id, membershipType=membership_type);

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
                let parsed: Result<::models::InlineResponse20046, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_vendor(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32, vendor_hash: i32, components: Vec<::models::DestinyDestinyComponentType>) -> Box<Future<Item = ::models::InlineResponse20036, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("components", &components.join(",").to_string())
            .finish();
        let uri_str = format!("{}/Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/Vendors/{vendorHash}/{}", configuration.base_path, query, characterId=character_id, destinyMembershipId=destiny_membership_id, membershipType=membership_type, vendorHash=vendor_hash);

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
                let parsed: Result<::models::InlineResponse20036, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_get_vendors(&self, character_id: i64, destiny_membership_id: i64, membership_type: i32, components: Vec<::models::DestinyDestinyComponentType>) -> Box<Future<Item = ::models::InlineResponse20035, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("components", &components.join(",").to_string())
            .finish();
        let uri_str = format!("{}/Destiny2/{membershipType}/Profile/{destinyMembershipId}/Character/{characterId}/Vendors/{}", configuration.base_path, query, characterId=character_id, destinyMembershipId=destiny_membership_id, membershipType=membership_type);

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
                let parsed: Result<::models::InlineResponse20035, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_insert_socket_plug(&self, ) -> Box<Future<Item = ::models::InlineResponse20015, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri_str = format!("{}/Destiny2/Actions/Items/InsertSocketPlug/", configuration.base_path);

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
                let parsed: Result<::models::InlineResponse20015, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn destiny2_search_destiny_entities(&self, search_term: &str, _type: &str, page: i32) -> Box<Future<Item = ::models::InlineResponse20042, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("page", &page.to_string())
            .finish();
        let uri_str = format!("{}/Destiny2/Armory/Search/{type}/{searchTerm}/{}", configuration.base_path, query, searchTerm=search_term, type=_type);

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
                let parsed: Result<::models::InlineResponse20042, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}