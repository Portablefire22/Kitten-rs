Companies are soooooooo restrictive. You use a web
browser to run your website based application but 
don't allow people to use their own custom CSS? 
Your application comes with built in support for JS 
but you make it against TOS to use custom JS scripts? 
Then you create an API for your application to use. You 
don't use advertisements for your main source of income, 
yet you insist on people using only your app? 
Despite all of this, saying you can't make your own client,
 you provide the documentation required to implement half 
of your client just because your bot API is the same as 
your client api? XDDDD

I'm not a company, I have exactly one functional project 
to my name, but I can try right? Probably not, but what 
if I create something better than an entire corp?

# Tech
- Leptos: Rust based web framework
- Tauri: Rust based framework for creating 
cross-platform applications.

# Denial
Discord publicaly provides an [API](https://discord.com/developers/docs/reference) 
that seems to roughly do what I want, surely this should be 
easy right? Right?
```Rust
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub channel_type: usize,
    pub guild_id: Option<String>,
    pub position: Option<usize>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<usize>,
    pub user_limit: Option<usize>,
    pub rate_limit_per_user: Option<usize>,
    pub recipients: Option<Vec<users::User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub managed: Option<bool>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQuality>,
    pub message_count: Option<usize>,
    pub member_count: Option<usize>,
    pub thread_metadata: Option<threads::ThreadMetadata>,
    pub member: Option<threads::ThreadMember>,
    pub default_auto_archive_duration: Option<usize>,
    pub permissions: Option<String>,
    pub flags: Option<usize>,
    pub total_message_sent: Option<usize>,
    pub available_tags: Option<Vec<ForumTag>>,
    pub applied_tags: Option<String>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<usize>,
    pub default_sort_order: Option<usize>,
    pub default_forum_layout: Option<usize>,
}
```
Turns out Discord like to save bandwidth and they do that 
by making ***EVERYTHING*** optional. If this was JS then 
that'd be fine but I'm running Rust, I need to specify 
every possible value that could ever appear or it will 
break. Some values are even more fun as bools have are a 
50/50 on if they actually appear whilst false :)

# Anger
DON'T BELIVE ITS LIES. DONT BELIEVE THE DOCUMENTATION WHEN 
IT SAYS SOMETHING IS REQUIRED. THEY NEVER ARE. EVERYTHING 
IS AN OPTION. NOTHING IS GUARRANTEED. THEY LIE.

Denial 
Anger
Bargaining
Depression
Acceptance
