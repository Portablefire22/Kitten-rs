## Meeeeeeeeeeeeeeeeeeeeeeeeeee :3
First of all, sorry for the mess, I am terrible at doing any sort of design work.  
Secondly, welcome! :3

I still have optimism for the future and hate how corporate the internet has become, which is why I refuse to not act silly x3  
Despite having most of my socials linked on the bottom, I still feel like it's a large enough disconnect to act different :P  

I have absolutely 0 professional experience when it comes to programming, with the extent of my tested professionalism is 
nicely complaining directly at my group members in a university group project, and will 100% show in my code (and writing) quality  :3

## Website

As documented in the first project post, this quite a simple website written in Rust and the entire purpose of the site is to document any project I attempt.
I'm not great at writing so it'll mostly just be quick descriptions of features and what they do, occasionally deciding to add more technical information when
it's needed or when I love the topic :3

Most of my projects, including [this website](https://github.com/portablefire22/kitten-rs), are open-source and can be found on [my Github](https://github,com/portablefire22).
Don't expect anything to be commented since my projects rarely reach the point where I decide to comment, and dont expect details on how to build anything
unless it's group work.

Some projects may include snippets of code to allow for easier explaining of a feature, an example of a code snippet can be seen below: 
```Rust
fn about() -> maud::Markup {
    let contents = fs::read_to_string("projects/about.md");
    let mut plugins = comrak::Plugins::default();
    let builder = SyntectAdapterBuilder::new().theme("base16-ocean.dark");
    let adapter = builder.build();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    match contents {
        Ok(cont) => {
            html! {
                div."content-container" {
                    h1 {"About"}
                    div."project-text"{
                        (maud::PreEscaped(comrak::markdown_to_html_with_plugins(&cont,
                            &comrak::Options::default(),
                            &plugins)))
                    }    
                }
            }
        },
        Err(e) => error(404),
    }
}
```
