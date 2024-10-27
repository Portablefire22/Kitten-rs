As a league player (disgusting, I know), I make avid use of statistic websites like [op.gg](https://op.gg),
[u.gg](https://u.gg), and [lolalytics](https://lolalytics.com) and I have always wondered how much effort would
be required to make one of these sites. The Riot API is well documented so it should be as simple as sending
HTTP requests for data right?

# The Journey

This isn't my first time at this project, nor will it be the last iteration of it, and that first iteration
can be found on [Github](https://github.com/Portablefire22/stats-website) with a video demonstration being on
[Youtube](https://www.youtube.com/watch?v=UFzqcmj7shI). My assumption of the project being easy was pretty spot on
for the state I achieved, the most difficult part of the entire project was the HTML & CSS. Both of which being
the most significant barrier in any web development project I pursue.

Most of my motivation to restart this project came from a combination of two factors: my university course has
a python Flask webdev assignment, and it isn't *enough* for me; and injecting estrogen has **massively** reignited
my passion for programming, clearly my body has understood the assignment of the "transfem programmer".

## Riot API
Thankfully, Riot provided both an API and documentation for the API on their [development website](https://developer.riotgames.com/).
It's just a bit unfortunate that I'm using Rust and therefore have to create a struct or enum for every possible
response for the api. Not great, but still not as bad as when I attempted to create a Discord client (this has caught my eye
again, so I might attempt this again) in Rust and had to use the Firefox network dev tools to determine
[Discord's Api](https://github.com/Portablefire22/discord-rs).

Riot's API was fairly easy to write an interface library for, with the following code snippet being how a Riot
account can be retrieved from a username and tagline.
```Rust
pub async fn by_riot_id(
    region: RoutingRegion,
    api_key: &String,
    game_name: &String,
    tag_line: &String,
) -> Option<Self> {
    let request_url = format!(
        "https://{}.api.riotgames.com/riot/account/v1/accounts/by-riot-id/{}/{}?api_key={}",
        region.to_string(),
        game_name,
        tag_line,
        api_key
    );
    let resp = reqwest::get(request_url).await.unwrap();
    let resp = resp.text().await.expect("Could not parse");
    match serde_json::from_str(&resp) {
        Ok(t) => {
            debug!("{}", serde_json::to_string_pretty(&resp).unwrap());
            Some(t)
        }
        Err(e) => {
            error!("{e:?}");
            None
        }
    }
}
```

## Player Searching
Fun fact, player searching was the reason I gave up on the last iteration of this project! Riot decided to
abandon summoner names, instead opting for the Riot ID system debuted in Valorant. Whilst this decision was met
with a negative reaction due to unique names no longer existing, I genuinely believe it was a great change since it
allows me to get names like "Lilith#Nyaa".

As with any change this caused breaking changes to the API, completely nuking the ability to search for players with
their summoner name. I probably could have fixed this, but the codebase was a mess due to server code being mixed with
API code causing the rewrite to be mentally draining to even think about.

Implementing player searching was fairly trivial, the only problem was due to me re-using the old HTML as it only had
a single text input when both the username and tagline need to be inputted. The solution to this was fairly trivial and
involved splitting the input at "#", then checking if the tag exists, and 404'ing if anything *freaky* occured. Not the
greatest solution, but it should be robust enough to ship.

## Summoner Profiles
Following with the theme of the previous section, my main problem whilst adding profiles was due to me re-using old HTML
that expected values I no longer had or had in a different format. Fixing these problems was fairly trivial, with the only
major change being that I have opted to use the ["CommunityDragon"](https://communitydragon.org) project for the game's
assets instead of hosting the DataDragon myself. This comes with the main advantages of: less network traffic for me, I'm
no longer hosting images and JSON files and therefore don't have to upload those with every request; better data, Riot
seemingly creates the DataDragon through a script that hasn't been updated in a while which causes the data to either be
in a strange format or just factually incorrect; and it's actually up-to-date, CommunityDragon provides a "latest" path and
doesn't need to be updated every league patch like self hosting the DataDragon would require.

Since I already have the summoner profile from searching, the main functionality to add now is showing the matches that
have been played and the player's stats. Player stats are as simple as a few API calls and inserting the data into the Tera
template, match history is a different beast though. Match history will require creating GET method that returns HTML,
this method was chosen as to allow for match history to be dynamic. The HTML will be streamed in as needed and inserted into
the match history section on the profile. Why don't I just generate these matches via JavaScript in the browser? Because I
fucking hate Javascript and Typescript with a ***passion***, it just does not vibe with me and I do not vibe with it.

## Match History
