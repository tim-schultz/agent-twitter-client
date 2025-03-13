use crate::api::client::TwitterClient;
use crate::api::requests::request_api;
use crate::error::Result;
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use urlencoding;

#[derive(Debug, Deserialize)]
pub struct HomeTimelineResponse {
    pub data: Option<HomeData>,
}

#[derive(Debug, Deserialize)]
pub struct HomeData {
    pub home: Home,
}

#[derive(Debug, Deserialize)]
pub struct Home {
    #[serde(rename = "home_timeline_urt")]
    pub home_timeline: HomeTimeline,
}

#[derive(Debug, Deserialize)]
pub struct HomeTimeline {
    pub instructions: Vec<TimelineInstruction>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TimelineInstruction {
    #[serde(rename = "TimelineAddEntries")]
    AddEntries { entries: Vec<TimelineEntry> },
    // Add other variants as needed
}

#[derive(Debug, Deserialize)]
pub struct TimelineEntry {
    pub content: EntryContent,
}

#[derive(Debug, Deserialize)]
pub struct EntryContent {
    #[serde(rename = "itemContent")]
    pub item_content: Option<ItemContent>,
}

#[derive(Debug, Deserialize)]
pub struct ItemContent {
    pub tweet_results: Option<TweetResults>,
}

#[derive(Debug, Deserialize)]
pub struct TweetResults {
    pub result: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterResponse {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub core: Option<Core>,
    pub edit_control: Option<EditControl>,
    pub is_translatable: Option<bool>,
    pub legacy: Legacy,
    pub quoted_status_result: Option<QuotedStatusResult>,
    pub rest_id: Option<String>,
    pub source: Option<String>,
    pub unmention_data: Option<HashMap<String, serde_json::Value>>,
    pub views: Option<Views>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Core {
    pub user_results: Option<UserResults>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResults {
    pub result: Option<UserResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResult {
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub affiliates_highlighted_label: Option<AffiliatesHighlightedLabel>,
    pub has_graduated_access: Option<bool>,
    pub id: Option<String>,
    pub is_blue_verified: Option<bool>,
    pub legacy: Option<UserLegacy>,
    pub professional: Option<Professional>,
    pub profile_image_shape: Option<String>,
    pub rest_id: Option<String>,
    pub tipjar_settings: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffiliatesHighlightedLabel {
    pub label: Option<Label>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub badge: Option<Badge>,
    pub description: Option<String>,
    pub url: Option<LabelUrl>,
    pub user_label_display_type: Option<String>,
    pub user_label_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Badge {
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelUrl {
    pub url: Option<String>,
    pub url_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLegacy {
    pub can_dm: Option<bool>,
    pub can_media_tag: Option<bool>,
    pub created_at: Option<String>,
    pub default_profile: Option<bool>,
    pub default_profile_image: Option<bool>,
    pub description: Option<String>,
    pub entities: Option<UserEntities>,
    pub fast_followers_count: Option<i64>,
    pub favourites_count: Option<i64>,
    pub followers_count: Option<i64>,
    pub following: Option<bool>,
    pub friends_count: Option<i64>,
    pub has_custom_timelines: Option<bool>,
    pub is_translator: Option<bool>,
    pub listed_count: Option<i64>,
    pub location: Option<String>,
    pub media_count: Option<i64>,
    pub name: Option<String>,
    pub normal_followers_count: Option<i64>,
    pub pinned_tweet_ids_str: Option<Vec<String>>,
    pub possibly_sensitive: Option<bool>,
    pub profile_banner_url: Option<String>,
    pub profile_image_url_https: Option<String>,
    pub profile_interstitial_type: Option<String>,
    pub screen_name: Option<String>,
    pub statuses_count: Option<i64>,
    pub translator_type: Option<String>,
    pub url: Option<String>,
    pub verified: Option<bool>,
    pub want_retweets: Option<bool>,
    pub withheld_in_countries: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntities {
    pub description: Option<Description>,
    pub url: Option<UrlEntity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    pub urls: Option<Vec<UrlInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlEntity {
    pub urls: Option<Vec<UrlInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlInfo {
    pub display_url: Option<String>,
    pub expanded_url: Option<String>,
    pub indices: Option<Vec<i64>>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Professional {
    pub category: Option<Vec<Category>>,
    pub professional_type: Option<String>,
    pub rest_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub icon_name: Option<String>,
    pub id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditControl {
    pub edit_tweet_ids: Option<Vec<String>>,
    pub editable_until_msecs: Option<String>,
    pub edits_remaining: Option<String>,
    pub is_edit_eligible: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Legacy {
    pub bookmark_count: Option<i64>,
    pub bookmarked: Option<bool>,
    pub conversation_id_str: Option<String>,
    pub created_at: Option<String>,
    pub display_text_range: Option<Vec<i64>>,
    pub entities: Option<Entities>,
    pub favorite_count: Option<i64>,
    pub favorited: Option<bool>,
    pub full_text: Option<String>,
    pub id_str: Option<String>,
    pub is_quote_status: Option<bool>,
    pub lang: Option<String>,
    pub quote_count: Option<i64>,
    pub quoted_status_id_str: Option<String>,
    pub quoted_status_permalink: Option<QuotedStatusPermalink>,
    pub reply_count: Option<i64>,
    pub retweet_count: Option<i64>,
    pub retweeted: Option<bool>,
    pub user_id_str: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entities {
    pub hashtags: Option<Vec<serde_json::Value>>,
    pub symbols: Option<Vec<serde_json::Value>>,
    pub timestamps: Option<Vec<serde_json::Value>>,
    pub urls: Option<Vec<UrlInfo>>,
    pub user_mentions: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotedStatusPermalink {
    pub display: Option<String>,
    pub expanded: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotedStatusResult {
    pub result: Option<Box<TwitterResponse>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Views {
    pub count: Option<String>,
    pub state: Option<String>,
}

pub async fn fetch_home_timeline(
    client: &TwitterClient,
    count: i32,
    seen_tweet_ids: Vec<String>,
) -> Result<Vec<TwitterResponse>> {
    let variables = serde_json::json!({
        "count": count,
        "includePromotedContent": false,
        "latestControlAvailable": true,
        "requestContext": "launch",
        "withCommunity": false,
        "seenTweetIds": seen_tweet_ids,
    });
    let features = serde_json::json!({
        "rweb_tipjar_consumption_enabled": true,
        "responsive_web_graphql_exclude_directive_enabled": true,
        "verified_phone_label_enabled": false,
        "creator_subscriptions_tweet_preview_api_enabled": true,
        "responsive_web_graphql_timeline_navigation_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "communities_web_enable_tweet_community_results_fetch": true,
        "c9s_tweet_anatomy_moderator_badge_enabled": true,
        "articles_preview_enabled": true,
        "responsive_web_edit_tweet_api_enabled": true,
        "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
        "view_counts_everywhere_api_enabled": true,
        "longform_notetweets_consumption_enabled": true,
        "responsive_web_twitter_article_tweet_consumption_enabled": true,
        "tweet_awards_web_tipping_enabled": false,
        "creator_subscriptions_quote_tweet_preview_enabled": false,
        "freedom_of_speech_not_reach_fetch_enabled": true,
        "standardized_nudges_misinfo": true,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
        "rweb_video_timestamps_enabled": true,
        "longform_notetweets_rich_text_read_enabled": true,
        "longform_notetweets_inline_media_enabled": true,
        "responsive_web_enhance_cards_enabled": false,
    });
    let url = format!(
        "https://x.com/i/api/graphql/HJFjzBgCs16TqxewQOeLNg/HomeTimeline?variables={}&features={}",
        urlencoding::encode(&variables.to_string()),
        urlencoding::encode(&features.to_string())
    );
    let mut headers = HeaderMap::new();
    client.auth.install_headers(&mut headers).await?;
    let (response, _) =
        request_api::<HomeTimelineResponse>(&client.client, &url, headers, Method::GET, None)
            .await?;
    let home = response
        .data
        .map(|data| data.home.home_timeline.instructions);
    let mut entries = Vec::new();
    if let Some(instructions) = home {
        for instruction in instructions {
            match instruction {
                TimelineInstruction::AddEntries {
                    entries: new_entries,
                } => {
                    for entry in new_entries {
                        if let Some(item_content) = entry.content.item_content {
                            if let Some(tweet_results) = item_content.tweet_results {
                                if let Some(result) = tweet_results.result {
                                    // Convert the result to TwitterResponse
                                    if let Ok(tweet) =
                                        serde_json::from_value::<TwitterResponse>(result)
                                    {
                                        entries.push(tweet);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(entries)
}
