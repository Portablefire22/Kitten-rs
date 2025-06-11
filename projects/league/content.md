Got someone cocky in Discord, do they always brag about being 'high-elo' when all they do is duo-queue with someone? 
Do you need to pull out stats to *verify* if they are high-elo? You probably answered "no" to both of those question 
but I didn't, so now I need a way to prove that people are boosted by others. 

So let's figure out how to do that :)

# Prerequisites
`Note: This section can be skipped if you are at all familiar with League of Legends and/or most video game ranked matchmaking`

League of Legends, like many other multiplayer games, provides a ranked matchmaking queue.
This queue provides a score after every game and allows the player to validly claim what their 
rank is and prove it with in-game data. Players may disagree with their rank, or decide to 
queue on a new account to go against easier opponents (smurfing), or they may engage 
in the main focus of this project, boosting. Boosting is the act of a higher skilled player 
improving the rank of the less skilled player. This may be performed by the higher elo player 
logging into the lower skilled player's account and directly winning games, or by joining 
the lower skilled player alongside them in game and winning with said low elo player. Both of 
these methods will result in the less skilled player to rankup and achieve ranks that they 
could not achieve solo.

League of Legends tracks this score with "League Points" (LP) in combination with a visible rank.
The visible rank can only be: Iron, Bronze, Silver, Gold, Platinum, Emerald, Diamond, Master, 
Grandmaster, or Challegner with the low skill ranks being Iron and the highest skilled players 
being situated in Challenger. Until one reaches master the visible rank is accompanied by a division 
from I-IV (IV being lowest), this division increases when LP reaches 100LP and decreases when going 
below 0LP, whereas master+ operates strictly on only showing LP since there is no cap to player 
skill. 

MMR (Matchmaking Rating) is a hidden value that determines your skill when compared to 
other players, the system behind MMR is completely hidden and provides no information to 
the player other than the amount of LP a player win/loses after a game. Due to this hidden 
nature, the playerbase has grown a hatred towards the MMR system as the system will 
inexplicably put you against "bad" players and reduce the LP gain of that game. Further 
products of the MMR system are the fabled "Winners Queue" and "Losers Queue", both 
are in the back of everyone's mind whilst queuing but they are never mentioned by the 
developers of the game. The details of these will be touched upon later when the analysis 
occurs.

# Tech
Various small libraries probably got included to save time so here are the main ones:
- [Rocket.rs](https://rocket.rs/): Web framework for Rust, akin to Python's [Flask](https://flask.palletsprojects.com/en/stable/)
- [Diesel.rs](https://github.com/diesel-rs/diesel): Library for accessing databases in Rust
- [Reqwest](https://github.com/seanmonstar/reqwest): Simple HTTP client for making web requests in Rust
- [Serde](https://serde.rs/): Framework for serialising & deserialising Rust data structures

# Starting the Project 
I'm going to be honest here, this part of the write-up was written 2 years after the intial 
creation of this project. Don't worry too much though, basically only the HTML & CSS transfered 
to the current version.

## Iteration One 
Not much to say about the development of this iteration, especially since I forgot about most 
of it, but I can show off a [video of the first working feature](https://www.youtube.com/watch?v=3xy4gUPteqM) 
and a [video of match history partially working](https://www.youtube.com/watch?v=UFzqcmj7shI). 

There is not much to show other than that, but I can go into detail about the main pitfalls 
of the first iteration.

### I can't do CSS or design work 
Turns out there is a reason "programmer art" is often simple. I tried to make a good design that 
was unique but I'm pretty sure I just recreated [op.gg](https://op.gg), whilst somehow managing 
to make it so that the website just doesn't scale dynamically at all. Unfortunate if you're on 
mobile or a high resolution I suppose, especially since I did all of the CSS on my 1440p monitor. 

### What the fuck is a library?
To sum it up, I'm entirely self taught. I moved too quick for any sort of formal education to 
help me, whilst this meant I excelled in school it also means that I'm just missing knowledge 
on some fundamental parts of programming. I could learn them if I needed but there is still 
a lot that I haven't encountered yet. I'm completely unaware of how little I know when it 
comes to programming.

Anyway, this basically meant that the first iteration of the project had ***ALL*** of the 
Riot (developers of League of Legends) API baked into the website, I couldn't change 
how the website interacted with the API without completely nuking functionality of the site. 
Combine this with the fact that Riot changed their api to remove unique names and you might 
now see why I abandonded this project 2 years ago. Further issues started to develop with the 
fact that I was terrible at structuring my project and would morph the data as needed into 
these Frankenstein structs that didn't follow the API or their documentation. 
An example of this Frankenstein creation would probably be the items of players, I decided 
to alter the data into a struct and therefore had a new function to run everytime I searched: 
```Rust
pub async fn get_match_items(mut local_match: MatchInformation) -> MatchInformation {

    for participant in &local_match.info.participants {
        let mut items: Vec<Datum> = Vec::new();
        // Hehe >:3
        items.push(get_item_by_id(participant.item0).await);
        items.push(get_item_by_id(participant.item1).await);
        items.push(get_item_by_id(participant.item2).await);
        items.push(get_item_by_id(participant.item3).await);
        items.push(get_item_by_id(participant.item4).await);
        items.push(get_item_by_id(participant.item5).await);
        items.push(get_item_by_id(participant.item6).await);
        local_match.participant_info.participant_items.push(items);
    }
    return local_match
}
```
Not the worst tbf, but what if I forgot to run this function? That's basically how most of 
the code ended up looking, bandaid solutions everywhere and disorder when you forgot to 
apply the fucked up fix.

So let's not do that in the next iteration.

# Iteration Two

## Creating the API Library
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
Currently I'm just creating the API as I go and only adding features that I need, after I finish I'll probably 
decide to include the entire API. No clue if I'd try to get it onto [crates.io](https://crates.io/) since I'd 
probably have to actually maintain something, write documentation, and make things actually consistent, which 
all sounds like a pain in the fucking arse.

### Structure
The API structure currently follows the Riot documentation and should therefore provide as little resistance as 
possible for use in future projects. Some of the library does depart from Riot's implementation however, cases 
of this are mostly where Riot make use of a string to represent a value from a selection and therefore could be 
swapped for an enum. The usage of enums will require more dev work whenever the API changes but can prevent 
any dev errors from mistyped comparisons or mistyped requests. I've tried as hard as possible to separate any 
site specific functions from the library as to prevent bloat and inconsistencies with functionality.

## Frontend
God I fucking hate front-end work.

### Player Searching
Fun fact, player searching was the reason I gave up on the last iteration of this project! Riot decided to
abandon summoner names, instead opting for the Riot ID system debuted in Valorant. Whilst this decision was met
with a negative reaction due to unique names no longer existing, I genuinely believe it was a great change since it
allows me to get names like "Lilith#Nyaa".

As with any change this caused breaking changes to the API, completely nuking the ability to search for players with
their summoner name. I probably could have fixed this, but the codebase was a mess due to server code being mixed with
API code causing the rewrite to be mentally draining to even think about.

Implementing player searching was fairly trivial, the only problem was due to me re-using the old HTML as it only had
a single text input when both the username and tagline need to be inputted. The solution to this was quite simple and
involved splitting the input at "#" then checking if the tag exists, looking at [u.gg](https://u,gg) 
and [op.gg](https://op.gg) makes it seem like that is the approach other people take.

![Search](/assets/league-index.jpg)

As mentioned earlier the HTML & CSS from the previous iteration was transfered to iteration #2 with 
only a minor modification to the search box, as seen in the above image. Functionality is quite 
simple, only requiring the selection of the region from the dropdown box and inputting in the 
form of \<username\>#\<tagline\>.

### Player Profile
The profile, the place of reflection, is where you can expect the most fun to occur. Nothing 
beats looking at your profile and seeing the red carpet be laid on your match history as 
you prepare the elo-slingshot up to master. This page is expected to show the most simple stats, 
providing information on the player as a whole, whilst providing a comprehensive history 
of the player's match history. Below is an example of how one's profile currently looks 
in the profile.

![Profile](/assets/league-profile.jpg)

Now we are in the present, most of the write-up from this point is written before or immediately 
after implementation, so let's talk about the plan. The current implementation relies too much 
on making API calls to Riot, which runs into problems when the rate limit is 20/s or 50/min, 
so we should start thinking about caching. 

### Implementing the Database
***FUCK***

I had been dreading this part of the project, infact it's why I'm currently writing this part, 
I just don't want to start work on the database. On paper it should be fine, nothing more than 
taking data and storing it in a way that I can use it later but it's just not that simple

```Rust
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    all_in_pings: i64,
    assist_me_pings: i64,
    assists: i64,
    baron_kills: i64,
    bounty_level: i64,
    champ_experience: i64,
    champ_level: i64,
    champion_id: i64,
    champion_name: String,
    command_pings: i64,
    /// Only used for kayn
    champion_transform: Option<i64>,
    consumables_purchased: i64,
    challenges: Challenges,
    damage_dealt_to_buildings: i64,
    damage_dealt_to_objectives: i64,
    damage_dealt_to_turrets: i64,
    damage_self_mitigated: i64,
    deaths: i64,
    detector_wards_placed: i64,
    double_kills: i64,
    dragon_kills: i64,
    eligible_for_progression: bool,
    enemy_missing_pings: i64,
    enemy_vision_pings: i64,
    first_blood_assist: bool,
    first_blood_kill: bool,
    first_tower_assist: bool,
    game_ended_in_early_surrender: bool,
    game_ended_in_surrender: bool,
    hold_pings: i64,
    get_back_pings: i64,
    gold_earned: i64,
    gold_spent: i64,
    /// This is a guess, typically best to use team_position
    individual_position: String,
    inhibitor_kills: i64,
    inhibitor_takedowns: i64,
    inhibitors_lost: i64,
    item_0: i64,
    item_1: i64,
    item_2: i64,
    item_3: i64,
    item_4: i64,
    item_5: i64,
    item_6: i64,
    items_purchased: i64,
    killing_sprees: i64,
    kills: i64,
    lane: String,
    largest_critical_strike: i64,
    largest_killing_spree: i64,
    largest_multi_kill: i64,
    longest_time_spent_living: i64,
    magic_damage_dealt: i64,
    magic_damage_dealt_to_champions: i64,
    magic_damage_taken: i64,
    missions: Missions,
    /// Jungle camps & pet kills
    neutral_minions_killed: i64,
    need_vision_pings: i64,
    nexus_kills: i64,
    nexus_takedowns: i64,
    nexus_lost: i64,
    objectives_stolen: i64,
    objectives_stolen_assists: i64,
    on_my_way_pings: i64,
    participant_id: i64,
    #[serde(rename = "playerScore0")]
    player_score0: Option<i64>,
    #[serde(rename = "playerScore1")]
    player_score1: Option<i64>,
    #[serde(rename = "playerScore2")]
    player_score2: Option<i64>,
    #[serde(rename = "playerScore3")]
    player_score3: Option<i64>,
    #[serde(rename = "playerScore4")]
    player_score4: Option<i64>,
    #[serde(rename = "playerScore5")]
    player_score5: Option<i64>,
    #[serde(rename = "playerScore6")]
    player_score6: Option<i64>,
    #[serde(rename = "playerScore7")]
    player_score7: Option<i64>,
    #[serde(rename = "playerScore8")]
    player_score8: Option<i64>,
    #[serde(rename = "playerScore9")]
    player_score9: Option<i64>,
    #[serde(rename = "playerScore10")]
    player_score10: Option<i64>,
    #[serde(rename = "playerScore11")]
    player_score11: Option<i64>,
    penta_kills: i64,
    perks: Perks,
    physical_damage_dealt: i64,
    physical_damage_dealt_to_champions: i64,
    physical_damage_taken: i64,
    placement: i64,
    player_augment_1: i64,
    player_augment_2: i64,
    player_augment_3: i64,
    player_augment_4: i64,
    player_subteam_id: i64,
    push_pings: i64,
    profile_icon: i64,
    puuid: String,
    quadra_kills: i64,
    riot_id_game_name: String,
    riot_id_tagline: String,
    role: String,
    sight_wards_bought_in_game: i64,
    spell1_casts: i64,
    spell2_casts: i64,
    spell3_casts: i64,
    spell4_casts: i64,
    subteam_placement: i64,
    summoner1_casts: i64,
    summoner1_id: i64,
    summoner2_casts: i64,
    summoner2_id: i64,
    summoner_id: String,
    summoner_level: i64,
    summoner_name: String,
    team_early_surrendered: bool,
    team_id: i64,
    team_position: String,
    #[serde(rename = "timeCCingOthers")]
    time_ccing_others: i64,
    time_played: i64,
    total_ally_jungle_minions_killed: i64,
    total_damage_dealt: i64,
    total_damage_dealt_to_champions: i64,
    total_damage_shielded_on_teammates: i64,
    total_damage_taken: i64,
    total_enemy_jungle_minions_killed: i64,
    /// Only direct heals, not regeneration
    total_heal: i64,
    total_heals_on_teammates: i64,
    /// Does not include jungle or pets
    total_minions_killed: i64,
    #[serde(rename = "totalTimeCCDealt")]
    total_time_cc_dealt: i64,
    total_time_spent_dead: i64,
    total_units_healed: i64,
    triple_kills: i64,
    true_damage_dealt: i64,
    true_damage_dealt_to_champions: i64,
    true_damage_taken: i64,
    turret_kills: i64,
    turret_takedowns: i64,
    turrets_lost: i64,
    /// Hexakill?
    unreal_kills: i64,
    vision_score: i64,
    vision_cleared_pings: i64,
    vision_wards_bought_in_game: i64,
    wards_killed: i64,
    wards_placed: i64,
    win: bool,
}
```

That was the struct used to contain a player's data within a match. I now have the 
fun part of figuring out how to condense this information into a DB by removing unneeded 
information whilst also separating data in a way that allows for analysis later on. I'm not 
good at doing this, especially since this'll be my first time attempting anything like this :D

#### Basic Account Information
Basic account information should hold all of the data required for creating a simple profile 
page, it doesn't need any match history or analysis, and therefore only requires data 
such as their name, tag, id, or icon but also current ranked information. The ranked information 
stored only corresponds to their tier, division, and LP, anything like their current ranked 
league name (most people don't even know this exists) or promotion status (removed 1.5
years ago) will be ignored.

I'm not an SQL pro, I'm basically just googling anything that isn't VARCHAR or INT, pls 
no flame :3
```SQL
create table summoners {
  id SERIAL PRIMARY KEY,
  accountId NVARCHAR(32),
  gameName NVARCHAR(24) NOT NULL,
  tagLine NVARCHAR(8) NOT NULL,
  puuid NVARCHAR(64) NOT NULL,
  [region] NVARCHAR(8) NOT NULL CHECK ([region] IN ('BR1', 'EUN1', 'EUW1', 'JP1', 'KR', 'LA1', 'LA2', 'ME1', 'NA1', 'OC1', 'PH2', 'RU', 'SG2', 'TH2', 'TR1', 'TW2', 'VN2')) DEFAULT 'EUW1',
  profileIcon INT DEFAULT 0,
  level INT DEFAULT 0,

  soloTier NVARCHAR[12] NOT NULL DEFAULT 'UNRANKED', 
  CONSTRAINT solo_tier_check CHECK (soloTier IN ('UNRANKED', 'IRON', 'BRONZE', 'SILVER', 'GOLD', 'PLATINUM', 'EMERALD', 'DIAMOND', 'MASTER', 'GRANDMASTER', 'CHALLENGER')),
  soloDivision TINYINT NOT NULL DEFAULT 0,
  CONSTRAINT solo_division_check CHECK (soloDivision >= 0 AND soloDivision <= 4),
  soloLp TINYINT NOT NULL DEFAULT 0,
  CONSTRAINT solo_lp_check CHECK (soloLp <= 100),

  flexTier NVARCHAR[12] NOT NULL DEFAULT 'UNRANKED', 
  CONSTRAINT flex_tier_check CHECK (flexTier IN ('UNRANKED', 'IRON', 'BRONZE', 'SILVER', 'GOLD', 'PLATINUM', 'EMERALD', 'DIAMOND', 'MASTER', 'GRANDMASTER', 'CHALLENGER')),
  flexDivision TINYINT NOT NULL DEFAULT 0,
  CONSTRAINT flex_division_check CHECK (flexDivision >= 0 AND flexDivision <= 4),
  flexLp TINYINT NOT NULL DEFAULT 0,
  CONSTRAINT flex_lp_check CHECK (flexLp <= 100),
}
```
### League Point History 
Statistic websites for League seem to all have LP history as an option, if I hope to have this 
as a feature then I'll need to start early to allow for the history to build. Facilitating the 
creation of this history feature requires the creation of a lpHistory table that'll contain 
all records of a player's LP, adding a new record whenever their profile is updated. This table 
will contain the queue, rankedTier, rankedDivision, lp, wins, losses, and the timestamp of the 
record's creation. Hopefully this data will be enough to build a comprehensive history of a 
player's LP history, maybe the updating of the LP could be automated for VIP's such as my own 
accounts or a friends so that the LP graph can be as detailed as possible.

```SQL
create table lpHistory {
  id SERIAL PRIMARY KEY,
  queue NVCHAR(32) NOT NULL DEFAULT 'RANKED_SOLO_5X5'
  CONSTRAINT queue_check CHECK (queue IN ('RANKED_SOLO_5X5', "RANKED_FLEX_SR", "RANKED_FLEX_TT"))
  tier NVARCHAR[12] NOT NULL DEFAULT 'UNRANKED', 
  CONSTRAINT tier_check CHECK (tier IN ('UNRANKED', 'IRON', 'BRONZE', 'SILVER', 'GOLD', 'PLATINUM', 'EMERALD', 'DIAMOND', 'MASTER', 'GRANDMASTER', 'CHALLENGER')),
  division TINYINT NOT NULL DEFAULT 0,
  CONSTRAINT division_check CHECK (division >= 0 AND division <= 4),
  lp TINYINT NOT NULL DEFAULT 0,
  CONSTRAINT lp_check CHECK (lp <= 100),
  win INT NOT NULL DEFAULT 0,
  CONSTRAINT win_check CHECK (win >= 0),
  loss INT NOT NULL DEFAULT 0,
  CONSTRAINT loss_check CHECK (loss >= 0),
  time TIMESTAMP,
}
```
### Match History 
Match history will be stored as a simple table that contains only the simple data, basically:
matchId, gameResult, gameCreation timestamp, gameDuation, gameEnd timestamp, gameId, gameMode, 
mapId, platformId, queueId, team_n_0-9 by summoner ID, tournamentCode (if 
present).

It could probably be condensed but I'm just going near 1:1 from the API data to the 
table.
```SQL
create table matches {
  id SERIAL PRIMARY KEY,
  matchId VARCHAR(64) NOT NULL,
  platformId VARCHAR(64) NOT NULL,
  queueId VARCHAR(64) NOT NULL,
  gameId VARCHAR(64) NOT NULL,
  endOfGameResult NVCHAR(32) NOT NULL,
  gameCreation DATETIME NOT NULL,
  gameDuration TIME NOT NULL,
  gameEndTimestamp DATETIME NOT NULL,
  gameMode NVARCHAR(64) NOT NULL,
  mapId NVARCHAR(64) NOT NULL,
  team0Participant0 NVARCHAR(64) NOT NULL,
  team0Participant1 NVARCHAR(64) NOT NULL,
  team0Participant2 NVARCHAR(64) NOT NULL,
  team0Participant3 NVARCHAR(64) NOT NULL,
  team0Participant4 NVARCHAR(64) NOT NULL,

  team1Participant0 NVARCHAR(64) NOT NULL,
  team1Participant1 NVARCHAR(64) NOT NULL,
  team1Participant2 NVARCHAR(64) NOT NULL,
  team1Participant3 NVARCHAR(64) NOT NULL,
  team1Participant4 NVARCHAR(64) NOT NULL,
  tournamentCode NVARCHAR(64),
  CONSTRAINT `match_participant_0`
    FOREIGN KEY (team0Participant0_id) REFERENCES summoners (puuid),

  CONSTRAINT `match_participant_1`
    FOREIGN KEY (team0Participant1_id) REFERENCES summoners (puuid),

  CONSTRAINT `match_participant_2`
    FOREIGN KEY (team0Participant2_id) REFERENCES summoners (puuid),

  CONSTRAINT `match_participant_3`
    FOREIGN KEY (team0Participant3_id) REFERENCES summoners (puuid),

  CONSTRAINT `match_participant_4`
    FOREIGN KEY (team0Participant4_id) REFERENCES summoners (puuid),

  CONSTRAINT `match_participant_5`
    FOREIGN KEY (team1Participant0_id) REFERENCES summoners (puuid),

  CONSTRAINT `match_participant_6`
    FOREIGN KEY (team1Participant1_id) REFERENCES summoners (puuid),

  CONSTRAINT `match_participant_7`
    FOREIGN KEY (team1Participant2_id) REFERENCES summoners (puuid),

  CONSTRAINT `match_participant_8`
    FOREIGN KEY (team1Participant3_id) REFERENCES summoners (puuid),

  CONSTRAINT `match_participant_9`
    FOREIGN KEY (team1Participant4_id) REFERENCES summoners (puuid),


} ENGINE = InnoDB;
```

### Match Participants 
I'm trying my best to make the tables as small as possible but that's quite hard when you're 
working with structs like the one showed earlier in **Implementing the Database**. To combat 
this I'm just going to create a participants table and populate that, linking particpants to 
their matches by the matchId column. It would take too long for me to list out the columns So 
I'll just include the SQL below.
