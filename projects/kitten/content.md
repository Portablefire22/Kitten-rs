Haiiiiiiii :3

I'm not great at designing websites so you'll just have to deal with this plain looking site,
but at least I have a site to show off my projects now :D

## Tech
The website is currently hosted using an "Ampere A1 Compute" instance from Oracle with Nginx being used to reverse proxy into the web server.
The web server is built using Rust and uses a current total of 7 direct dependencies:
- tiny_http: handle http requests
- maud: generate html 
- ascii: http headers need to converted from utf-8 to ascii
- toml: reading tomls from file
- comrak: converting markdown to html
- serde: converting tomls to struct 
- chrono: converting unix timestamp 

Previous attempts at creating this website had all utilised the "Rocket" framework. Whilst Rocket was easy to use, it was too easy,
which killed my interest in the project. Rocket had simply removed the learning requirement for me, which is essential for me to 
even be remotely interested in a project.

### Why
The entire existence of this website essentially boils down to the combination of "I needed a project that had an actual function" and
having a server with an existing domain.

Previous attempts at creating a website just lead to me losing interest due to the creation process not being engaging. The process of creating 
this website was chosen after watching a [Youtube video](https://www.youtube.com/watch?v=s_oT5bggjrc) by [Destiny Hailstorm](https://www.youtube.com/@DestinyHailstorm).  
I doubt I'll actually stick with this website but at least I have something to occupy my domain now. ¯\\\_(ツ)_/¯

### Future
As mentioned, there is a high likelihood that this website will be completely abandonded. 

**BUT**, there is a small chance that I'll host web compatible projects here :3
