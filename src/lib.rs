use std::collections::{HashMap, HashSet};

// TODO: Decide how to implement versions!

pub struct Account {
    pub id: String,
    pub username: String,
    /// Content: URI
    pub acct: String,
    /// Content: URL
    ///     - Scheme: HTTPS
    pub url: String,
    pub display_name: String,
    /// Content: HTML
    pub note: String,
    /// Content: URL
    pub avatar: String,
    /// Content: URL
    pub avatar_static: String,
    /// Content: URL
    pub header: String,
    /// Content: URL
    pub header_static: String,
    pub locked: bool,
    pub emojis: Vec<Emoji>,
    pub discoverable: bool,
    /// Content: ISO 8601 datetime
    pub created_at: String,
    /// Content: ISO 8601 datetime
    pub last_status_at: String,
    pub statuses_count: u32,
    pub followers_count: u32,
    pub following_count: u32,
    pub moved: Option<Box<Account>>,
    pub fields: Option<Vec<Field>>,
    pub bot: Option<bool>,
    pub source: Option<Source>,
    pub suspended: Option<bool>,
    /// Content: ISO 8601 datetime
    pub mute_expires_at: Option<String>,
}

pub struct Activity {
    /// Content: UNIX timestamp
    pub week: String,
    /// Content: Integer
    pub statuses: String,
    /// Content: Integer
    pub logins: String,
    /// Content: Integer
    pub registrations: String,
}

pub struct Announcement {
    pub id: String,
    pub text: String,
    pub published: bool,
    pub all_day: bool,
    /// Content: ISO 8601 datetime
    pub created_at: String,
    /// Content: ISO 8601 datetime
    pub updated_at: String,
    pub read: bool,
    pub reactions: Vec<AnnouncementReaction>,
    /// Content: ISO 8601 datetime
    pub scheduled_at: Option<String>,
    /// Content: ISO 8601 datetime
    pub starts_at: Option<String>,
    /// Content: ISO 8601 datetime
    pub ends_at: Option<String>,
}

pub struct AnnouncementReaction {
    pub name: String,
    pub count: u32,
    pub me: bool,
    /// Content: URL
    pub url: String,
    /// Content: URL
    pub static_url: String,
}

pub struct Application {
    pub name: String,
    /// Content: URL
    pub website: Option<String>,
    pub vapid_key: Option<String>,
    pub client_id: String,
    pub client_secret: String,
}

#[non_exhaustive]
pub enum AttachementType {
    Unknown,
    Image,
    Gifv,
    Video,
    Audio,
}

pub struct Attachment {
    pub id: String,
    pub r#type: AttachementType,
    /// Content: URL
    pub url: String,
    /// Content: URL
    pub preview_url: String,
    /// Content: URL
    pub remote_url: Option<String>,
    // TODO: Decide on type.
    pub meta: Option<HashMap<String, String>>,
    pub description: Option<String>,
    /// Content: BlurHash hash
    pub blurhash: Option<String>,
    // TODO: Decide what to do with deprecated attribute: text_url.
}

#[non_exhaustive]
pub enum CardType {
    Link,
    Photo,
    Video,
    Rich,
}

pub struct Card {
    /// Content: URL
    pub url: String,
    pub title: String,
    pub description: String,
    pub r#type: CardType,
    pub author_name: Option<String>,
    /// Content: URL
    pub author_url: Option<String>,
    pub provider_name: Option<String>,
    /// Content: URL
    pub provider_url: Option<String>,
    /// Content: HTML
    pub html: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    /// Content: URL
    pub image: Option<String>,
    /// Content: URL
    pub embed_url: Option<String>,
    /// Content: BlurHash hash
    pub blurhash: Option<String>,
}

pub struct Context {
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}

pub struct Conversation {
    pub id: String,
    pub accounts: Vec<Account>,
    pub unread: bool,
    pub last_status: Option<Status>,
}

pub struct Emoji {
    pub shortcode: String,
    /// Content: URL
    pub url: String,
    /// Content: URL
    pub static_url: String,
    pub visible_in_picker: bool,
    pub category: Option<String>,
}

pub struct Error {
    pub error: String,
    pub error_description: Option<String>,
}

pub struct FeaturedTag {
    pub id: String,
    pub name: String,
    /// Content: URL
    pub url: String,
    pub statuses_count: u32,
    /// Content: ISO 8601 datetime
    pub last_status_at: String,
}

pub struct Field {
    pub name: String,
    // TODO: Verify if this content type is true.
    /// Content: HTML
    pub value: String,
    /// Content: ISO 8601 datetime
    pub verified_at: Option<String>,
}

#[non_exhaustive]
pub enum ContextFilter {
    Home,
    Notifications,
    Public,
    Thread,
}

pub struct Filter {
    pub id: String,
    pub phrase: String,
    pub context: HashSet<ContextFilter>,
    /// Content: ISO 8601 datetime
    pub expires_at: Option<String>,
    pub irreversible: bool,
    pub whole_word: bool,
}

pub struct History {
    /// Content: UNIX timestamp
    pub day: String,
    /// Content: Integer
    pub uses: String,
    /// Content: Integer
    pub accounts: String,
}

pub struct IdentityProof {
    pub provider: String,
    pub provider_username: String,
    /// Content: URL
    pub profile_url: String,
    /// Content: URL
    pub proof_url: String,
    /// Content: ISO 8601 datetime
    pub updated_at: String,
}

pub struct Instance {
    // TODO: Verify if this really isn't guaranteed to be a uri?
    pub uri: String,
    pub title: String,
    pub description: String,
    pub short_description: String,
    pub email: String,
    pub version: String,
    /// Content: ISO 639 part 1-5 language codes
    pub languages: Vec<String>,
    pub registrations: bool,
    pub approval_required: bool,
    pub invites_enabled: bool,
    pub urls: HashMap<String, String>,
    pub stats: HashMap<String, String>,
    /// Content: URL
    pub thumbnail: Option<String>,
    pub contact_account: Option<Account>,
}

pub struct Source {
    pub note: String,
    pub fields: Vec<Field>,
    pub privacy: Option<PostPrivacy>,
    pub sensitive: Option<bool>,
    pub language: Option<String>,
    pub follow_requests_count: Option<u32>,
}

#[non_exhaustive]
pub enum PostPrivacy {
    Public,
    Unlisted,
    Private,
    Direct,
}

#[non_exhaustive]
pub enum StatusVisibility {
    Public,
    Unlisted,
    Private,
    Direct,
}

pub struct Status {
    pub id: String,
    pub uri: String,
    pub created_at: String,
    pub account: Account,
    pub content: String,
    pub visibility: StatusVisibility,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub media_attachments: Vec<Attachment>,
    pub application: Application,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub emojis: Vec<Emoji>,
    pub reblogs_count: u32,
    pub favourites_count: u32,
    pub replies_count: u32,
    pub url: Option<String>,
    pub in_remoy_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub reblog: Option<Box<Status>>,
    pub poll: Option<Poll>,
    pub card: Option<Card>,
    pub language: Option<String>,
    pub text: Option<String>,
    pub favourited: Option<bool>,
    pub reblogged: Option<bool>,
    pub muted: Option<bool>,
    pub bookmarked: Option<bool>,
    pub pinned: Option<bool>,
}

pub enum RepliesPolicy {
    Followed,
    List,
    None,
}

pub struct List {
    pub id: String,
    pub title: String,
    pub replies_policy: RepliesPolicy,
}

pub struct Marker {
    pub home: HashMap<String, String>,
    pub notifications: HashMap<String, String>,
}

pub struct Mention {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub url: String,
}

pub enum NotificationType {
    Follow,
    FollowRequest,
    Mention,
    Reblog,
    Favourite,
    Poll,
    Status,
}

pub struct Notification {
    pub id: String,
    pub r#type: NotificationType,
    pub created_at: String,
    pub account: Account,
    pub status: Option<Status>,
}

pub struct Poll {
    pub id: String,
    pub expires_at: Option<String>,
    pub expired: bool,
    pub multiple: bool,
    pub votes_count: u32,
    pub voters_count: Option<u32>,
    pub voted: Option<bool>,
    pub own_votes: Vec<HashMap<String, String>>,
    pub emojis: Vec<Emoji>,
}

pub enum PostVisibility {
    Public,
    Unlisted,
    Private,
    Direct,
}

pub enum ExpandMediaSetting {
    Default,
    ShowAll,
    HideAll,
}

pub struct Preferences {
    pub posting_default_visibility: PostVisibility,
    pub posting_default_sensitive: bool,
    pub posting_default_language: Option<String>,
    pub reading_expand_media: ExpandMediaSetting,
    pub reading_expand_spoilers: bool,
}

pub struct PushSubscription {
    pub id: String,
    pub endpoint: String,
    pub server_key: String,
    pub alerts: HashMap<String, String>,
}

pub struct RelationShip {
    pub id: String,
    pub following: bool,
    pub requested: bool,
    pub endorsed: bool,
    pub followed_by: bool,
    pub muting: bool,
    pub muting_notifications: bool,
    pub showing_reblogs: bool,
    pub notifying: bool,
    pub blocking: bool,
    pub domain_blocking: bool,
    pub blocked_by: bool,
    pub note: String,
}

pub struct Report {}

pub struct Results {
    pub accounts: Vec<Account>,
    pub statuses: Vec<Status>,
    pub hashtags: Vec<Tag>,
}

pub struct ScheduledStatus {}

pub struct Tag {
    pub id: String,
    pub url: String,
    pub history: Vec<History>,
}

pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub created_at: u32,
}
