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
