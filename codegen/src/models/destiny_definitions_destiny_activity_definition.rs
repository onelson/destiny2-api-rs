/* 
 * Bungie.Net API
 *
 * These endpoints constitute the functionality exposed by Bungie.net, both for more traditional website functionality and for connectivity to Bungie video games and their related functionality.
 *
 * OpenAPI spec version: 2.0.0
 * Contact: support@bungie.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// DestinyDefinitionsDestinyActivityDefinition : The static data about Activities in Destiny 2.  Note that an Activity must be combined with an ActivityMode to know - from a Gameplay perspective - what the user is \"Playing\".  In most PvE activities, this is fairly straightforward. A Story Activity can only be played in the Story Activity Mode.  However, in PvP activities, the Activity alone only tells you the map being played, or the Playlist that the user chose to enter. You'll need to know the Activity Mode they're playing to know that they're playing Mode X on Map Y.  Activity Definitions tell a great deal of information about what *could* be relevant to a user: what rewards they can earn, what challenges could be performed, what modifiers could be applied. To figure out which of these properties is actually live, you'll need to combine the definition with \"Live\" data from one of the Destiny endpoints.  Activities also have Activity Types, but unfortunately in Destiny 2 these are even less reliable of a source of information than they were in Destiny 1. I will be looking into ways to provide more reliable sources for type information as time goes on, but for now we're going to have to deal with the limitations. See DestinyActivityTypeDefinition for more information.

#[derive(Debug, Serialize, Deserialize)]
pub struct DestinyDefinitionsDestinyActivityDefinition {
  /// The title, subtitle, and icon for the activity.
  #[serde(rename = "displayProperties")]
  display_properties: Option<Object>,
  /// If the activity has an icon associated with a specific release (such as a DLC), this is the path to that release's icon.
  #[serde(rename = "releaseIcon")]
  release_icon: Option<String>,
  /// If the activity will not be visible until a specific and known time, this will be the seconds since the Epoch when it will become visible.
  #[serde(rename = "releaseTime")]
  release_time: Option<i32>,
  /// The difficulty level of the activity.
  #[serde(rename = "activityLevel")]
  activity_level: Option<i32>,
  /// The recommended light level for this activity.
  #[serde(rename = "activityLightLevel")]
  activity_light_level: Option<i32>,
  /// The hash identifier for the Destination on which this Activity is played. Use it to look up the DestinyDestinationDefinition for human readable info about the destination. A Destination can be thought of as a more specific location than a \"Place\". For instance, if the \"Place\" is Earth, the \"Destination\" would be a specific city or region on Earth.
  #[serde(rename = "destinationHash")]
  destination_hash: Option<i32>,
  /// The hash identifier for the \"Place\" on which this Activity is played. Use it to look up the DestinyPlaceDefinition for human readable info about the Place. A Place is the largest-scoped concept for location information. For instance, if the \"Place\" is Earth, the \"Destination\" would be a specific city or region on Earth.
  #[serde(rename = "placeHash")]
  place_hash: Option<i32>,
  /// The hash identifier for the Activity Type of this Activity. You may use it to look up the DestinyActivityTypeDefinition for human readable info, but be forewarned: Playlists and many PVP Map Activities will map to generic Activity Types. You'll have to use your knowledge of the Activity Mode being played to get more specific information about what the user is playing.
  #[serde(rename = "activityTypeHash")]
  activity_type_hash: Option<i32>,
  /// The difficulty tier of the activity.
  #[serde(rename = "tier")]
  tier: Option<i32>,
  /// When Activities are completed, we generate a \"Post-Game Carnage Report\", or PGCR, with details about what happened in that activity (how many kills someone got, which team won, etc...) We use this image as the background when displaying PGCR information, and often use it when we refer to the Activity in general.
  #[serde(rename = "pgcrImage")]
  pgcr_image: Option<String>,
  /// The expected possible rewards for the activity. These rewards may or may not be accessible for an individual player based on their character state, the account state, and even the game's state overall. But it is a useful reference for possible rewards you can earn in the activity. These match up to rewards displayed when you hover over the Activity in the in-game Director, and often refer to Placeholder or \"Dummy\" items: items that tell you what you can earn in vague terms rather than what you'll specifically be earning (partly because the game doesn't even know what you'll earn specifically until you roll for it at the end)
  #[serde(rename = "rewards")]
  rewards: Option<Vec<::models::DestinyDefinitionsDestinyActivityRewardDefinition>>,
  /// Activities can have Modifiers, as defined in DestinyActivityModifierDefinition. These are references to the modifiers that *can* be applied to that activity, along with data that we use to determine if that modifier is actually active at any given point in time.
  #[serde(rename = "modifiers")]
  modifiers: Option<Vec<::models::DestinyDefinitionsDestinyActivityModifierReferenceDefinition>>,
  /// If True, this Activity is actually a Playlist that refers to multiple possible specific Activities and Activity Modes. For instance, a Crucible Playlist may have references to multiple Activities (Maps) with multiple Activity Modes (specific PvP gameplay modes). If this is true, refer to the playlistItems property for the specific entries in the playlist.
  #[serde(rename = "isPlaylist")]
  is_playlist: Option<bool>,
  /// An activity can have many Challenges, of which any subset of them may be active for play at any given period of time. This gives the information about the challenges and data that we use to understand when they're active and what rewards they provide. Sadly, at the moment there's no central definition for challenges: much like \"Skulls\" were in Destiny 1, these are defined on individual activities and there can be many duplicates/near duplicates across the Destiny 2 ecosystem. I have it in mind to centralize these in a future revision of the API, but we are out of time.
  #[serde(rename = "challenges")]
  challenges: Option<Vec<::models::DestinyDefinitionsDestinyActivityChallengeDefinition>>,
  /// If there are status strings related to the activity and based on internal state of the game, account, or character, then this will be the definition of those strings and the states needed in order for the strings to be shown.
  #[serde(rename = "optionalUnlockStrings")]
  optional_unlock_strings: Option<Vec<::models::DestinyDefinitionsDestinyActivityUnlockStringDefinition>>,
  /// Represents all of the possible activities that could be played in the Playlist, along with information that we can use to determine if they are active at the present time.
  #[serde(rename = "playlistItems")]
  playlist_items: Option<Vec<::models::DestinyDefinitionsDestinyActivityPlaylistItemDefinition>>,
  /// Unfortunately, in practice this is almost never populated. In theory, this is supposed to tell which Activity Graph to show if you bring up the director while in this activity.
  #[serde(rename = "activityGraphList")]
  activity_graph_list: Option<Vec<::models::DestinyDefinitionsDestinyActivityGraphListEntryDefinition>>,
  /// This block of data provides information about the Activity's matchmaking attributes: how many people can join and such.
  #[serde(rename = "matchmaking")]
  matchmaking: Option<Object>,
  /// This block of data, if it exists, provides information about the guided game experience and restrictions for this activity. If it doesn't exist, the game is not able to be played as a guided game.
  #[serde(rename = "guidedGame")]
  guided_game: Option<Object>,
  #[serde(rename = "directActivityModeHash")]
  direct_activity_mode_hash: Option<i32>,
  #[serde(rename = "directActivityModeType")]
  direct_activity_mode_type: Option<i32>,
  #[serde(rename = "activityModeHashes")]
  activity_mode_hashes: Option<Vec<i32>>,
  #[serde(rename = "activityModeTypes")]
  activity_mode_types: Option<Vec<::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType>>,
  /// If true, this activity is a PVP activity or playlist.
  #[serde(rename = "isPvP")]
  is_pv_p: Option<bool>,
  /// The unique identifier for this entity. Guaranteed to be unique for the type of entity, but not globally.  When entities refer to each other in Destiny content, it is this hash that they are referring to.
  #[serde(rename = "hash")]
  hash: Option<i32>,
  /// The index of the entity as it was found in the investment tables.
  #[serde(rename = "index")]
  index: Option<i32>,
  /// If this is true, then there is an entity with this identifier/type combination, but BNet is not yet allowed to show it. Sorry!
  #[serde(rename = "redacted")]
  redacted: Option<bool>
}

impl DestinyDefinitionsDestinyActivityDefinition {
  /// The static data about Activities in Destiny 2.  Note that an Activity must be combined with an ActivityMode to know - from a Gameplay perspective - what the user is \"Playing\".  In most PvE activities, this is fairly straightforward. A Story Activity can only be played in the Story Activity Mode.  However, in PvP activities, the Activity alone only tells you the map being played, or the Playlist that the user chose to enter. You'll need to know the Activity Mode they're playing to know that they're playing Mode X on Map Y.  Activity Definitions tell a great deal of information about what *could* be relevant to a user: what rewards they can earn, what challenges could be performed, what modifiers could be applied. To figure out which of these properties is actually live, you'll need to combine the definition with \"Live\" data from one of the Destiny endpoints.  Activities also have Activity Types, but unfortunately in Destiny 2 these are even less reliable of a source of information than they were in Destiny 1. I will be looking into ways to provide more reliable sources for type information as time goes on, but for now we're going to have to deal with the limitations. See DestinyActivityTypeDefinition for more information.
  pub fn new() -> DestinyDefinitionsDestinyActivityDefinition {
    DestinyDefinitionsDestinyActivityDefinition {
      display_properties: None,
      release_icon: None,
      release_time: None,
      activity_level: None,
      activity_light_level: None,
      destination_hash: None,
      place_hash: None,
      activity_type_hash: None,
      tier: None,
      pgcr_image: None,
      rewards: None,
      modifiers: None,
      is_playlist: None,
      challenges: None,
      optional_unlock_strings: None,
      playlist_items: None,
      activity_graph_list: None,
      matchmaking: None,
      guided_game: None,
      direct_activity_mode_hash: None,
      direct_activity_mode_type: None,
      activity_mode_hashes: None,
      activity_mode_types: None,
      is_pv_p: None,
      hash: None,
      index: None,
      redacted: None
    }
  }

  pub fn set_display_properties(&mut self, display_properties: Object) {
    self.display_properties = Some(display_properties);
  }

  pub fn with_display_properties(mut self, display_properties: Object) -> DestinyDefinitionsDestinyActivityDefinition {
    self.display_properties = Some(display_properties);
    self
  }

  pub fn display_properties(&self) -> Option<&Object> {
    self.display_properties.as_ref()
  }

  pub fn reset_display_properties(&mut self) {
    self.display_properties = None;
  }

  pub fn set_release_icon(&mut self, release_icon: String) {
    self.release_icon = Some(release_icon);
  }

  pub fn with_release_icon(mut self, release_icon: String) -> DestinyDefinitionsDestinyActivityDefinition {
    self.release_icon = Some(release_icon);
    self
  }

  pub fn release_icon(&self) -> Option<&String> {
    self.release_icon.as_ref()
  }

  pub fn reset_release_icon(&mut self) {
    self.release_icon = None;
  }

  pub fn set_release_time(&mut self, release_time: i32) {
    self.release_time = Some(release_time);
  }

  pub fn with_release_time(mut self, release_time: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.release_time = Some(release_time);
    self
  }

  pub fn release_time(&self) -> Option<&i32> {
    self.release_time.as_ref()
  }

  pub fn reset_release_time(&mut self) {
    self.release_time = None;
  }

  pub fn set_activity_level(&mut self, activity_level: i32) {
    self.activity_level = Some(activity_level);
  }

  pub fn with_activity_level(mut self, activity_level: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.activity_level = Some(activity_level);
    self
  }

  pub fn activity_level(&self) -> Option<&i32> {
    self.activity_level.as_ref()
  }

  pub fn reset_activity_level(&mut self) {
    self.activity_level = None;
  }

  pub fn set_activity_light_level(&mut self, activity_light_level: i32) {
    self.activity_light_level = Some(activity_light_level);
  }

  pub fn with_activity_light_level(mut self, activity_light_level: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.activity_light_level = Some(activity_light_level);
    self
  }

  pub fn activity_light_level(&self) -> Option<&i32> {
    self.activity_light_level.as_ref()
  }

  pub fn reset_activity_light_level(&mut self) {
    self.activity_light_level = None;
  }

  pub fn set_destination_hash(&mut self, destination_hash: i32) {
    self.destination_hash = Some(destination_hash);
  }

  pub fn with_destination_hash(mut self, destination_hash: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.destination_hash = Some(destination_hash);
    self
  }

  pub fn destination_hash(&self) -> Option<&i32> {
    self.destination_hash.as_ref()
  }

  pub fn reset_destination_hash(&mut self) {
    self.destination_hash = None;
  }

  pub fn set_place_hash(&mut self, place_hash: i32) {
    self.place_hash = Some(place_hash);
  }

  pub fn with_place_hash(mut self, place_hash: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.place_hash = Some(place_hash);
    self
  }

  pub fn place_hash(&self) -> Option<&i32> {
    self.place_hash.as_ref()
  }

  pub fn reset_place_hash(&mut self) {
    self.place_hash = None;
  }

  pub fn set_activity_type_hash(&mut self, activity_type_hash: i32) {
    self.activity_type_hash = Some(activity_type_hash);
  }

  pub fn with_activity_type_hash(mut self, activity_type_hash: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.activity_type_hash = Some(activity_type_hash);
    self
  }

  pub fn activity_type_hash(&self) -> Option<&i32> {
    self.activity_type_hash.as_ref()
  }

  pub fn reset_activity_type_hash(&mut self) {
    self.activity_type_hash = None;
  }

  pub fn set_tier(&mut self, tier: i32) {
    self.tier = Some(tier);
  }

  pub fn with_tier(mut self, tier: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.tier = Some(tier);
    self
  }

  pub fn tier(&self) -> Option<&i32> {
    self.tier.as_ref()
  }

  pub fn reset_tier(&mut self) {
    self.tier = None;
  }

  pub fn set_pgcr_image(&mut self, pgcr_image: String) {
    self.pgcr_image = Some(pgcr_image);
  }

  pub fn with_pgcr_image(mut self, pgcr_image: String) -> DestinyDefinitionsDestinyActivityDefinition {
    self.pgcr_image = Some(pgcr_image);
    self
  }

  pub fn pgcr_image(&self) -> Option<&String> {
    self.pgcr_image.as_ref()
  }

  pub fn reset_pgcr_image(&mut self) {
    self.pgcr_image = None;
  }

  pub fn set_rewards(&mut self, rewards: Vec<::models::DestinyDefinitionsDestinyActivityRewardDefinition>) {
    self.rewards = Some(rewards);
  }

  pub fn with_rewards(mut self, rewards: Vec<::models::DestinyDefinitionsDestinyActivityRewardDefinition>) -> DestinyDefinitionsDestinyActivityDefinition {
    self.rewards = Some(rewards);
    self
  }

  pub fn rewards(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyActivityRewardDefinition>> {
    self.rewards.as_ref()
  }

  pub fn reset_rewards(&mut self) {
    self.rewards = None;
  }

  pub fn set_modifiers(&mut self, modifiers: Vec<::models::DestinyDefinitionsDestinyActivityModifierReferenceDefinition>) {
    self.modifiers = Some(modifiers);
  }

  pub fn with_modifiers(mut self, modifiers: Vec<::models::DestinyDefinitionsDestinyActivityModifierReferenceDefinition>) -> DestinyDefinitionsDestinyActivityDefinition {
    self.modifiers = Some(modifiers);
    self
  }

  pub fn modifiers(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyActivityModifierReferenceDefinition>> {
    self.modifiers.as_ref()
  }

  pub fn reset_modifiers(&mut self) {
    self.modifiers = None;
  }

  pub fn set_is_playlist(&mut self, is_playlist: bool) {
    self.is_playlist = Some(is_playlist);
  }

  pub fn with_is_playlist(mut self, is_playlist: bool) -> DestinyDefinitionsDestinyActivityDefinition {
    self.is_playlist = Some(is_playlist);
    self
  }

  pub fn is_playlist(&self) -> Option<&bool> {
    self.is_playlist.as_ref()
  }

  pub fn reset_is_playlist(&mut self) {
    self.is_playlist = None;
  }

  pub fn set_challenges(&mut self, challenges: Vec<::models::DestinyDefinitionsDestinyActivityChallengeDefinition>) {
    self.challenges = Some(challenges);
  }

  pub fn with_challenges(mut self, challenges: Vec<::models::DestinyDefinitionsDestinyActivityChallengeDefinition>) -> DestinyDefinitionsDestinyActivityDefinition {
    self.challenges = Some(challenges);
    self
  }

  pub fn challenges(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyActivityChallengeDefinition>> {
    self.challenges.as_ref()
  }

  pub fn reset_challenges(&mut self) {
    self.challenges = None;
  }

  pub fn set_optional_unlock_strings(&mut self, optional_unlock_strings: Vec<::models::DestinyDefinitionsDestinyActivityUnlockStringDefinition>) {
    self.optional_unlock_strings = Some(optional_unlock_strings);
  }

  pub fn with_optional_unlock_strings(mut self, optional_unlock_strings: Vec<::models::DestinyDefinitionsDestinyActivityUnlockStringDefinition>) -> DestinyDefinitionsDestinyActivityDefinition {
    self.optional_unlock_strings = Some(optional_unlock_strings);
    self
  }

  pub fn optional_unlock_strings(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyActivityUnlockStringDefinition>> {
    self.optional_unlock_strings.as_ref()
  }

  pub fn reset_optional_unlock_strings(&mut self) {
    self.optional_unlock_strings = None;
  }

  pub fn set_playlist_items(&mut self, playlist_items: Vec<::models::DestinyDefinitionsDestinyActivityPlaylistItemDefinition>) {
    self.playlist_items = Some(playlist_items);
  }

  pub fn with_playlist_items(mut self, playlist_items: Vec<::models::DestinyDefinitionsDestinyActivityPlaylistItemDefinition>) -> DestinyDefinitionsDestinyActivityDefinition {
    self.playlist_items = Some(playlist_items);
    self
  }

  pub fn playlist_items(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyActivityPlaylistItemDefinition>> {
    self.playlist_items.as_ref()
  }

  pub fn reset_playlist_items(&mut self) {
    self.playlist_items = None;
  }

  pub fn set_activity_graph_list(&mut self, activity_graph_list: Vec<::models::DestinyDefinitionsDestinyActivityGraphListEntryDefinition>) {
    self.activity_graph_list = Some(activity_graph_list);
  }

  pub fn with_activity_graph_list(mut self, activity_graph_list: Vec<::models::DestinyDefinitionsDestinyActivityGraphListEntryDefinition>) -> DestinyDefinitionsDestinyActivityDefinition {
    self.activity_graph_list = Some(activity_graph_list);
    self
  }

  pub fn activity_graph_list(&self) -> Option<&Vec<::models::DestinyDefinitionsDestinyActivityGraphListEntryDefinition>> {
    self.activity_graph_list.as_ref()
  }

  pub fn reset_activity_graph_list(&mut self) {
    self.activity_graph_list = None;
  }

  pub fn set_matchmaking(&mut self, matchmaking: Object) {
    self.matchmaking = Some(matchmaking);
  }

  pub fn with_matchmaking(mut self, matchmaking: Object) -> DestinyDefinitionsDestinyActivityDefinition {
    self.matchmaking = Some(matchmaking);
    self
  }

  pub fn matchmaking(&self) -> Option<&Object> {
    self.matchmaking.as_ref()
  }

  pub fn reset_matchmaking(&mut self) {
    self.matchmaking = None;
  }

  pub fn set_guided_game(&mut self, guided_game: Object) {
    self.guided_game = Some(guided_game);
  }

  pub fn with_guided_game(mut self, guided_game: Object) -> DestinyDefinitionsDestinyActivityDefinition {
    self.guided_game = Some(guided_game);
    self
  }

  pub fn guided_game(&self) -> Option<&Object> {
    self.guided_game.as_ref()
  }

  pub fn reset_guided_game(&mut self) {
    self.guided_game = None;
  }

  pub fn set_direct_activity_mode_hash(&mut self, direct_activity_mode_hash: i32) {
    self.direct_activity_mode_hash = Some(direct_activity_mode_hash);
  }

  pub fn with_direct_activity_mode_hash(mut self, direct_activity_mode_hash: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.direct_activity_mode_hash = Some(direct_activity_mode_hash);
    self
  }

  pub fn direct_activity_mode_hash(&self) -> Option<&i32> {
    self.direct_activity_mode_hash.as_ref()
  }

  pub fn reset_direct_activity_mode_hash(&mut self) {
    self.direct_activity_mode_hash = None;
  }

  pub fn set_direct_activity_mode_type(&mut self, direct_activity_mode_type: i32) {
    self.direct_activity_mode_type = Some(direct_activity_mode_type);
  }

  pub fn with_direct_activity_mode_type(mut self, direct_activity_mode_type: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.direct_activity_mode_type = Some(direct_activity_mode_type);
    self
  }

  pub fn direct_activity_mode_type(&self) -> Option<&i32> {
    self.direct_activity_mode_type.as_ref()
  }

  pub fn reset_direct_activity_mode_type(&mut self) {
    self.direct_activity_mode_type = None;
  }

  pub fn set_activity_mode_hashes(&mut self, activity_mode_hashes: Vec<i32>) {
    self.activity_mode_hashes = Some(activity_mode_hashes);
  }

  pub fn with_activity_mode_hashes(mut self, activity_mode_hashes: Vec<i32>) -> DestinyDefinitionsDestinyActivityDefinition {
    self.activity_mode_hashes = Some(activity_mode_hashes);
    self
  }

  pub fn activity_mode_hashes(&self) -> Option<&Vec<i32>> {
    self.activity_mode_hashes.as_ref()
  }

  pub fn reset_activity_mode_hashes(&mut self) {
    self.activity_mode_hashes = None;
  }

  pub fn set_activity_mode_types(&mut self, activity_mode_types: Vec<::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType>) {
    self.activity_mode_types = Some(activity_mode_types);
  }

  pub fn with_activity_mode_types(mut self, activity_mode_types: Vec<::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType>) -> DestinyDefinitionsDestinyActivityDefinition {
    self.activity_mode_types = Some(activity_mode_types);
    self
  }

  pub fn activity_mode_types(&self) -> Option<&Vec<::models::DestinyHistoricalStatsDefinitionsDestinyActivityModeType>> {
    self.activity_mode_types.as_ref()
  }

  pub fn reset_activity_mode_types(&mut self) {
    self.activity_mode_types = None;
  }

  pub fn set_is_pv_p(&mut self, is_pv_p: bool) {
    self.is_pv_p = Some(is_pv_p);
  }

  pub fn with_is_pv_p(mut self, is_pv_p: bool) -> DestinyDefinitionsDestinyActivityDefinition {
    self.is_pv_p = Some(is_pv_p);
    self
  }

  pub fn is_pv_p(&self) -> Option<&bool> {
    self.is_pv_p.as_ref()
  }

  pub fn reset_is_pv_p(&mut self) {
    self.is_pv_p = None;
  }

  pub fn set_hash(&mut self, hash: i32) {
    self.hash = Some(hash);
  }

  pub fn with_hash(mut self, hash: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.hash = Some(hash);
    self
  }

  pub fn hash(&self) -> Option<&i32> {
    self.hash.as_ref()
  }

  pub fn reset_hash(&mut self) {
    self.hash = None;
  }

  pub fn set_index(&mut self, index: i32) {
    self.index = Some(index);
  }

  pub fn with_index(mut self, index: i32) -> DestinyDefinitionsDestinyActivityDefinition {
    self.index = Some(index);
    self
  }

  pub fn index(&self) -> Option<&i32> {
    self.index.as_ref()
  }

  pub fn reset_index(&mut self) {
    self.index = None;
  }

  pub fn set_redacted(&mut self, redacted: bool) {
    self.redacted = Some(redacted);
  }

  pub fn with_redacted(mut self, redacted: bool) -> DestinyDefinitionsDestinyActivityDefinition {
    self.redacted = Some(redacted);
    self
  }

  pub fn redacted(&self) -> Option<&bool> {
    self.redacted.as_ref()
  }

  pub fn reset_redacted(&mut self) {
    self.redacted = None;
  }

}


