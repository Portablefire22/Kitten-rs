Before we get into my ever-increasingly deranged rant about LLMs we should look at 
some context. 

Tinkering with things has always been an interest to me, I still remember ripping 
open .jar files to mess with textures files on early Minecraft - sometimes even 
delving into the mess that was inserting Java .class files into the jar file 
to play with mods - and just sitting in awe of how a simple modification could 
change so much of a game. From this I had started to develop my ability to 
solve problems as my msPaint creations suddenly lacked transparency - plus the 
dread that came from thinking I had permanently make my doors and glass lack 
transparency - forcing me to figure out how to fix this problem I had created. 
Information on this topic was sparse back then - I don't even know if there is 
information on it now other than "redownload the game" - So I had to figure out 
how to completely replace the textures with a "vanilla" texture pack I found 
online.

It might not sound like much but that experience carried on for years. Problems 
with technology stopped being a problem when I had the lingering thought of how 
"I fixed Minecraft" to provide me with the motivation to power through obstacles. 
The previous problem of modding became an art, procurring mods whose .class files 
did not conflict, debugging problems that occured became second nature as 
looking through heaps of Java errors became a daily task. 

These days those problems are gone; texture packs are easily available online, and 
mods are more accessible than ever with the introduction of modpacks and launchers 
like Curseforge or Prism. It's not like thats a bad thing, improving accessibility 
allows those less capable to enjoy things they could not previously, but it has 
began to rear its ugly head in recent times with the introduction of LLMs. 

# The art of "Googling"

Googling - the act of using a search engine such as Google, Bing, or Duckduckgo 
to retrieve information - is an art form, the tools provided by Google allow 
anywhere from the broadest of searches to the most narrow 1 result search you 
could think of. Don't believe me? Google '\<MOVIE NAME\> intitle:"index of"' and 
you'll find people's public file servers that just contain rips of movies, 
explore a few of them and you'll probably find one that is viewable right in the 
browser. Researching something old and you only get garbage new pages discussing 
that topic in little detail? Add "before:<Year>" and you'll only get results 
indexed as being before that year. I have projects that dabble with reviving 
old systems and the ability to specify before 2016 is immensly useful as anything 
newer than 2016 is either incorrect, talking about the wrong thing, or is just 
an LLM rehashing a post from 2016. 

I'm certain I've done a terrible job at explaining all of that so it's probably 
best if you google "google dorking" for more information on how to manipulate 
google searches into giving you exactly what you want.

# Where am I going with this?

We've gone through enough of the context so here is the start of the rant. 
Large language models, LLMs, such as ChatGPT or DeepSeek have introduced an 
interresting problem to the world, finding information is no longer gated 
behind the "skill" of searching what you want to know and is instead 
now a "friend" who will converse with you to figure out what you want. Like 
all good friends, this one is not afraid to just straight up lie to you in an 
effort to appear smart. It will not tell you about it's own lying and will 
often attempt to gaslight you into thinking it is correct if you push back 
on its bullshit. Most people in tech understand this dynamic, ChatGPT tells 
you something and you basically flip a coin to determine if you should accept 
what it said as truth, but I've interracted with a lot of people who seem to 
think that ChatGPT can't be wrong or that it would never lie to you. 

I have witnessed people at university encounter problems that not only can be 
easily googled but are explained in depth in the actual lecture material. They 
would rather go back and forth with a chat bot than read the first result on Google.
They'll encounter a problem with hardware and instead of thinking for a second or 
asking the lab assistants they'll spend the rest of the session "talking" with 
ChatGPT just to never actually get the correct answer in the end. Programming 
assignments result in half of the class just using ChatGPT, Cursor, Copilot, or 
other LLMs to create their assignment and then dare to question when the feedback
states they haven't actually done half of the assignment. 

# Examples 

I can talk shit about LLMs but it takes examples to actually show how dogshit 
these things can be. I refuse to sign-up to any of these LLM sites so I will 
be using ChatGPT from this point onwards, most people I've encountered use ChatGPT 
for this sort of task so hopefully it will be good enough.

[October 31st 2024 was when ChatGPT announced their upgrade that allows ChatGPT to 
actively search the web](https://openai.com/index/introducing-chatgpt-search/). 
One would think this would provide ChatGPT with accurate 
information and that the problems of ChatGPT bullshitting would be solved, those 
who think that would be wrong. 

## Kitten.rs 

Prompt: `What is on the website https://kitten.rs`

Response: 
```markdown
The website **[https://kitten.rs](https://kitten.rs)** is a simple, playful introduction to the **Kitten programming language**. 
It serves as both a showcase and a starting point for anyone interested in learning about Kitten.

Here’s a general breakdown of what you’ll find on the website:

1. **Homepage (Main Page):**

   * The page has a minimalistic design with a cute, kitten-themed aesthetic.
   * It introduces Kitten as a stack-based, minimalist programming language.
   * It has an embedded live REPL (Read-Eval-Print Loop), so you can try out Kitten code directly in the browser.

2. **Features:**

   * The language’s simplicity is emphasized, with a small set of commands that revolve around stack manipulation,
   which is typical for stack-based languages.
   * Kitten is designed to be compact and easy to experiment with, making it ideal for learning or just exploring 
   programming concepts.

3. **Interactive REPL:**

   * You can write and execute Kitten code directly on the website without needing to set up anything locally. 
   It’s perfect for quick experimentation.

4. **Syntax and Examples:**

   * The website provides examples of Kitten’s syntax and its basic structure. Since it’s stack-based, operations 
   like pushing and popping from the stack are central concepts.

5. **Links to Documentation:**

   * The site usually includes links to further documentation, guides, and other resources for people wanting to
   dig deeper into Kitten.

The general vibe is very approachable and fun, with a focus on simplicity. If you were expecting something more 
advanced, Kitten isn’t about being feature-heavy; it’s more about teaching the fundamentals of programming in a 
very clean and easy-to-digest format.

If you want, I can help you explore or try out some Kitten code directly!

```

You're on this site so you probably know that this website has nothing to do with 
the "Kitten Programming Language" - infact I knew nothing of it before this rant - 
and yet it has somehow confused my website with [Kittenlang.org](https://kittenlang.org/).
Better yet, it has seemingly just lied to me by saying it has an interactive REPL 
considering neither of the sites have one. Asking who the creator of this site has 
left me even more confused as a guy called "Nikita Lushchik" apparently made my website. 

```markdown
The creator of **[kitten.rs](https://kitten.rs)** is **Nikita Lushchik**. He is the developer and creator of the 
Kitten programming language itself, which is a fun, minimalist, stack-based language. The website serves as both
a showcase for the language and an interactive platform for users to try it out.

Nikita seems to have crafted Kitten as a playful yet educational way to explore programming concepts, particularly
for those interested in minimalism and low-level programming techniques like stack manipulation.

Would you like to explore any particular aspect of Kitten or have questions about how it works?
```

The worst fucking part is that [Jon Purdy](https://github.com/evincarofautumn) created 
the Kitten programming language and they weren't even mentioned??? 

Maybe it couldn't access the website - I don't know - but it could atleast inform me 
about the fact it couldn't access the website? Even then OpenAI have scanned my 
website enough to at least figure out that the website has nothing to do with the 
Programming language.

## Project Analysis 

Maybe searching websites isn't its strong suit despite these new developments, so clearly 
I should ask for something that was before the cut off. I don't know when the cut-off occured 
and seemingly OpenAI hasn't provided this information to the public, it's okay though since 
the information we're going to be using is dated around 2015. 

For context, I have been slowly working on recreating the old servers for the Adobe AIR 
League of Legends client and have gained knowledge of how a lot of it functions. Most 
of my knowledge has come from reading old GitHub repos, reading packets from the Client 
as it attempts to communicate with dead servers, and just plain old luck when I've guessed 
the functionality of random modules. Some libraries I'm using have been ripped directly 
from the Adobe AIR client itself and some information is from decompiling the included SWF files. 
Through this I've gained enough knowledge to at least know when something is wrong when it comes 
to how the Adobe Air client functions and how one would go about recreating the backend for it.

Prompt: `how would I Recreate the backend for the old league of legend adobe air client`

Response:
```markdown
Recreating the backend for the old League of Legends Adobe AIR client would involve several key steps. 
Given that Adobe AIR was used for the client side of the game in its earlier versions (prior to around 
2011-2013), you would need to reverse engineer the communication protocols and rebuild the necessary 
server architecture that the old client communicated with. Here's a broad approach to tackle this:

### 1. **Understand the Original Client Architecture**

* **Client-Side (Adobe AIR)**: The League of Legends client used Adobe AIR for cross-platform (Windows 
and macOS) support, making it easier to handle game updates and patches without relying on native code.
* **Backend Communication**: The client would connect to Riot’s backend servers to fetch data, authenticate
users, retrieve in-game information, and handle matchmaking.

### 2. **Identify the Key Components**

The old League of Legends client backend most likely had several components that you need to focus on:

* **Authentication Server**: User logins, validation, and session management.
* **Matchmaking Service**: Matchmaking algorithms and queueing systems for games.
* **Lobby Server**: Handling pre-game lobbies, player invitations, and team compositions.
* **Game Server**: Once the match starts, the game server takes over the real-time communication and 
game state management.
* **Data Persistence**: User profiles, stats, item purchases, and other user data.

### 3. **Reconstruct the APIs**

* **HTTP Requests**: The client would likely send HTTP requests to backend endpoints for authentication, 
retrieving account data, or matchmaking. Look into old network traffic logs, if available, from early 
League of Legends versions (before the transition to a new client architecture).
* **Real-time Communication**: For in-game communication (game state, player positions, etc.), the client
would use TCP or WebSockets (or something similar) to maintain a continuous connection with the game server.

To reconstruct the backend, you need to:

* **Understand old API endpoints** (request-response patterns) for login, matchmaking, account management, etc.
* **Recreate the same data structures** used in the old API responses (JSON/XML, etc.).

### 4. **Rebuild the Matchmaking System**

Matchmaking in the old client was likely driven by an Elo-like system (before the modern MMR system) and
would match players based on certain criteria like:

* **Rank**: Elo or similar rating system.
* **Region**: Server regions.
* **Latency**: Geographic location affecting latency.

You'll need to:

* Design a **matchmaking algorithm** that tries to balance skill levels and queues players accordingly.
* **Implement queue systems** (solo, duo, etc.) that match players into lobbies.

### 5. **Server-side Architecture**

You’d need to set up:

* **Backend servers** to handle all the matchmaking logic.
* **Databases** for storing player information (like user profiles, game history, etc.).
* **Game instance servers**: These are real-time servers that handle the game state, syncing data 
between clients during the game.

Popular technologies could include:

* **Node.js** or **Python** for real-time APIs.
* **Redis** or **Kafka** for fast message passing in a real-time environment.
* **PostgreSQL** or **MySQL** for structured data like user profiles and game statistics.

### 6. **Client-Side Adaptation**

Since Adobe AIR is deprecated, the client would have to either:

* **Run on older machines** that support Adobe AIR (not a scalable solution).
* **Rebuild the client in modern technologies** (HTML5, JavaScript, etc.) while maintaining the same 
backend communication structure.

### 7. **Rebuild Data Persistence Systems**

* You'll need a **persistent storage** system to keep track of player progress, stats, skins, etc. In 
the early versions, this was likely backed by SQL databases.
* **Implementing database migrations** to ensure the data is compatible with the old format.

### 8. **Reverse Engineer Old Protocols**

* **Network Traffic Analysis**: If you can find old network traffic (for example, pcap files, or old 
client logs), you can reverse-engineer how the client communicated with the server. Tools like 
**Wireshark** would help inspect HTTP/WebSocket traffic.
* **Inspect Client Code**: If you can get access to old Adobe AIR client code, you can find the 
network endpoints and see how the client interacted with the backend.

### 9. **Testing & Validation**

After rebuilding the backend:

* Test client-server interactions.
* Simulate match queuing, authentication, and live game states.
* Ensure the server performance can handle the expected load (many thousands of concurrent players).

### 10. **Security Considerations**

* Ensure the server has basic security measures such as **rate limiting**, **SSL/TLS encryption**, 
and **input sanitization**.
* Implement **OAuth** or **JWT** authentication to secure user sessions.

### Tools & Technologies to Consider:

* **Game Engine**: If you're rebuilding the client, Unity or Unreal Engine might be a good choice 
for modern game development.
* **WebSockets**: For real-time communication between the client and the game server.
* **Microservices**: Consider breaking down the backend into microservices for scalability and 
ease of maintenance.
* **Game Server Hosting**: Platforms like **Google Cloud**, **AWS**, or **Azure** offer services
that can host real-time game servers.

### Potential Challenges:

1. **Reverse engineering the old client and protocol** may not be straightforward.
2. **Legal issues**: Rebuilding something based on a proprietary game like League of Legends 
could infringe on copyright laws, so this might be best done for educational purposes unless Riot Games gives permission.

### Conclusion

Recreating the backend for the old League of Legends Adobe AIR client would require careful 
analysis of old communication protocols, rebuilding game servers for matchmaking and in-game
functionality, and implementing a backend infrastructure to support data persistence. You
would likely need to reverse-engineer both the client and server interactions, which can be a complex but rewarding challenge.

```

Let's go over what is wrong here.

- League of Legends used Adobe Air from [2009](https://wiki.leagueoflegends.com/en-us/Beta_Test) - 
[2017](https://na.leagueoflegends.com/en/featured/preseason-2017/client) not 2011-2013.
- Game Servers are mentioned despite the game and client being two separate entities and not in 
scope of the question asked.
- ChatGPT guesses the Client uses HTTP for all networking when HTTP is used only for getting a login 
token, with the rest of networking being performed over RTMP or XMPP. 
- ChatGPT guesses that API responses will be JSON/XML when all RTMP responses are AMF (Action Message 
Format).
- ChatGPT suggests just rebuilding the Client in HTML5 when the question was to create the 
backend, not the client.
- ChatGPT suggests the use of Websockets for communication when, in reality, standard TCP sockets 
are used to send RTMP information.

It might seem like ChatGPT wasn't ***that*** wrong until you consider the journey required for me 
to not even get past the login screen. For testing I had to patch the client into using unsecured 
connections so that I could read them via wireshark, then I had to download SDKs for Adobe AIR & Flex 
to get access to the debugging information to even hold an established connection. Establishing 
these connection required implementing a significant chunk of the RTMP standard along with 
(de)serialisation of AMF to create a stream so I could send and receive data from a Client. Now I have 
to essentially re-create the functionality of [BlazeDS](https://github.com/apache/flex-blazeds) 
so my backend can correctly communicate with Adobe Flex clients. None of this was even mentioned 
in ChatGPT's guide despite the fact that RTMP and AMF are how Adobe Flash/Air programs would 
interact with Flash Media Servers or Flex servers like BlazeDS.

The other information is decent I guess but it's just cookie cutter advice for creating any 
backend for a game client, I thought the entire point of these LLMs was to adapt and provide 
information specific to the request but these things can't even figure out what protocol or 
data format Adobe Flash used to communicate with remote servers when that information is 
literally on the [wikipedia page for Adobe flash](https://en.wikipedia.org/wiki/Adobe_Flash).

# More Rambling 

So You have these LLMs just straight up lying to people, providing incorrect information, or 
providing the most generic programming advice ever and junior developers are treating these 
responses as Gospel. Those who need correct and concise information most are being given 
terrible advice the second they deviate from the most common path just because these algorithms 
would rather lie than admit they're wrong.

## Tutorial Hell and Its Evolution 
