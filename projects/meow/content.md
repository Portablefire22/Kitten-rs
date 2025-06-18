Discord, like most companies these days, have released an application to the public that uses the 
Electron framework to ship a website as a native application. Fair enough to be honest, I've done 
GUI projects before and the only one that didn't make want to kill myself was working with Electron 
to create a simple music player; it's just so strange that they decide to completely restrict the 
user in what they can do. They've completely disabled the developer tools in the standard client 
and explicitly forbid any modification to their client in the TOS - they act like third-party 
modifications could do anything that they don't allow when the data can only be accessed through 
their own API? Then they hide behind their TOS forbidding third-party clients whilst providing 
the documentation for how to access the same API that the first-party client uses all because 
their bot API is the exact same as the user API; at this point they're just asking for people to 
try and make a third-party client. So here we are, let's try and make a third-party client.

# Tech 

I have a semi-active Golang project going on right now and it has occupied the last 131 hours 
(as of writing this document) that I have dedicated to programming. I've come to enjoy the 
language and will probably use it whenever the thought of Python would usually cross my mind -
it has type-safety, error checking, and it just works but it's not Rust. Something about Rust 
just scratches an itch inside of me, I will abandon C++ projects just to go write something in 
Rust because I haven't had a hit of Rust in a while, it was the language that re-ignited my 
passion for programming and it has just been pulling me back in everytime I try something new. 
So I loaded up ["Are We Gui Yet"](https://areweguiyet.com) and narrowed down all of the options 
to: GTK, Tauri, or "just" writing a "small" Vulkan application that displays a GUI - with the 
reasoning being that I needed something that worked cross-platform, since many people aren't 
developing Linux only GUI libraries. I'm not a designer - nor do I wish to sit down and learn 
UI design - so I'd prefer if the GUI library also had some design library available to me, 
leaving GTK and Tauri. GTK kicked my arse on a different project so I chose 
[Tauri](https://tauri.app/) :) 

## Tauri 

If you don't know what Tauri is then just think of your average Electron application but without 
an instance of Google Chrome bundled within. For our purposes it's just Electron but with 
Rust instead of [Java/Type]Script as the backend language. Front-end is entirely my choice and 
since I'm not a masochist, I opted for React + Material UI with Typescript as the language of 
choice. I'd prefer if it was all Rust but my experience with Rust in GUI scenarios isn't the 
greatest, as the All-Seeing Borrowchecker loves to just eviscerate me over the slightest fuck-up, 
and I'm not away of any Rust web UI libraries that would fix my "I'm not a designer" problem.

# The API Library

I ***really*** shouldn't have done this.

Yes I probably could have used something like [Serenity-rs](https://github.com/serenity-rs/serenity), 
but sometimes you have to learn how something works to best re-create it. This is all just 
cope but at least I'm gaining some skills out of it - I imagine re-creating a client from 
just network requests and whatever minimal documentation is available online will probably 
help me in the long run. The resulting mess of a library produced by this adventure can be 
found on the [GitHub repo](https://github.com/Portablefire22/discord-rs); I would recommend 
you don't actually use this since other libaries are more developed, but it *might* be helpful
for following along or creating your own library.

# Writing style, or "Writing is painful" 

From this point onward, the discussion will be written in a manner where time has no meaning. 
Some things were written after, some written before, and some written during. Things written after 
will likely be ordered in the popular sorting method of "whichever I remembered doing first", 
whilst stuff written during - or yet to be even created - will be roughly in order. 

# API library

API libraries and GUIs should never require eachother to function, yeah the GUI might do some 
wizardry to display information and the API might be called by the GUI but I feel like 
they should stay in their own lane. Not only is this approach possible, it's the approach 
already taken by the application. Separating the front-end and back-end is just this approach 
but forced by Tauri's architecture, components on the front-end render with data they request 
and functions in the back-end deliver the data in the correct format. The backend could be 
entirely replaced but the program would still function as long as the data is formatted correctly; 
it might sound quite obvious but I've made the mistake of not separating the two on a 
web dev project and it was not fun. 

So to take it one step further the API library will be created with the intention that it could 
just be used as a standalone library for Discord bots/userbots. As per this requirement, the 
library will need to automatically deal with things like heartbeats or websocket connections 
that allow Discord clients to interface with the servers.

## REST

### Login 

First things first, this was not the first thing to be done - in fact it was added quite 
late into the library - but it's the best for simplicity since everything requires a 
user token. User tokens are just a method of authenticating when sending a username and 
password on every request is just not practical; tokens are just large unique alpha-numeric 
strings that are assigned to a user's session whenever they login with their credentials, 
and are attached to nearly every request to the Discord REST API. Web developer tools reveal 
that gathering the user token is as simple as sending username, password, and undelete (no clue 
what this does) in JSON to `discord.com/api/v9/auth/login` and - assuming that you don't get 
prompted for a Captcha or have multi-factor authentication linked to your account - then 
retrieve the token from the JSON response. 

Multi-factor Authentication is roughly the same, the login response JSON will instead prompt 
for MFA, the selected MFA is submitted to `https://discord.com/api/v9/auth/mfa/<MFA TYPE>` 
with a JSON body containing the relevant MFA code and a "ticket" field whose value can be 
retrieved from the previous login response. Submitting that should return a login response 
with the user's login token.

At the time of writing I haven't quite figured out Captchas yet so that'll be a problem for 
future me, and an explanation for later sections.

### Reading Messages 

*"If you wish to make an apple pie from scratch, you must first invent the universe." - Carl Sagan*

Reading messages turned out to be quite the investment, like twelve files of various structs 
produced by slogging through Discord documentation. Most of them made sense - messages require 
an author so I had to create a user structure - but messages needed channels to be defined, which 
ended up requiring: guilds, emojis, embeds, reactions, roles, stickers, threads, attachments, 
and applications just to receive a channel's messages from the API. Retrieving the messages 
from the API after defining the structure was as easy as submitting a get request to 
`https://discord.com/api/v9/channels/<channel id>` with the token attached to the request 
headers - more information when the 'Client' is discussed for the API - and parsing the 
response JSON to the defined message structure. Receiving messages when they've been sent 
isn't quite as simple - I mean I *could* just set a task to retrieve messages every few 
seconds - with the correct method being to setup a websocket connection to listen for 
message events and process the incoming stream. That's for later me and will be discussed 
when 'Gateway' is discussed in the API.

![Screenshot of messages in application](/assets/meow-messages.png)
*Screenshot of retrieved messages in a prototype UI*

### Sending Messages 

Likewise with reading messages, sending messages first required a significant amount of 
structures to hold the required data for sending messages. Thankfully though the 
structures required are also the structures for reading messages so we don't actually have 
to do anything special. Infact, all sending a message requires is defining the channel id to 
send to, defining the content to send, and then wrapping it in some JSON that gets sent 
via a POST request to `https://discord.com/api/v9/channels/<channel id>`, reading the 
response for a Message object or error message/status code to determine if it was 
sent successfully.

## Gateway 

Discord supplements their REST API with a WebSocket connection dubbed the 
"[Gateway API](https://discord.com/developers/docs/events/gateway)", these sockets 
are basically just there for the Discord servers to tell the client when something  
happened, e.g. Message Receive events.   

# React UI

