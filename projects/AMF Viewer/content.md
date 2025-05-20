*As always, [Github Repo Link](https://github.com/Portablefire22/AMF-Viewer)*

Action Message Format at first glance just looks like your typical 
binary data format, objects go in and serialised data comes out, 
but it has actually been the source of my bruises for the past few 
months ever since I started a project involving the revival of an 
old Adobe Air program. Like most things related to Adobe Flash, 
AMF has basically zero presence in the modern age, and it's quite 
often you'll see web applications just opt for JSON instead of trying 
to roll out their own binary data format. 

So why am I doing this? Well it turns out that re-creating old Adobe 
servers requires re-creating their proprietary data format; who would 
have guessed? Wireshark provides excellent tools for reading RTMP 
and AMF, both of which are required and used heavily together in my 
main project, but I want the flexibility of creating my own solution 
that I can fine-tune to show exactly what is wrong with my AMF code 
instead of a generic "malformed packet" that provides no explanation 
of how or why it is malformed.

# Tech

It has been too long since I last touched a Rust codebase and I really
miss it. My current main project is currently written entirely in 
GoLang and after 130 hours (more than my entire time with Rust) I'm 
starting to miss Rust, so I've decided that all supplementary 
programs/servers will be written in Rust as to not make myself give up 
on the project when I'm close to a breakthrough.


- [Dioxus](https://dioxuslabs.com/) - Rust GUI framework.

# Actually Creating the Damn Thing

First things first, I need a GUI to display the data. Thankfully 
Dioxus has decent documentation, some of the information is outdated 
to the point of just completely not working at all, but it's enough 
to get something down. I followed the introduction sections and then 
started the side-quest of adding [TailwindCSS](https://tailwindcss.com/) 
to the project; TailwindCSS is a CSS framework that just improves 
general quality of life when it comes to the CSS of a page. I can't 
design to save my life, so I need as much help as I can get without 
just opting for some library with premade [React](https://react.dev/) 
components. Dioxus includes a setup command that creates projects 
with selected features, one of these features being Tailwind, but 
this seemingly hasn't been updated for Tailwind 4.0, and so I wasted  
roughly 2.5 hours banging my head against the wall wondering why the 
code that the documentation says "it just works" was in-fact not 
working. I installed TailwindCSS v3.0 and it began working 
immediately :)

Working GUI + CSS = time to actually work on the app, so a quick 
"file open" button was created that just uses 
[RFD](https://docs.rs/rfd/latest/rfd/) to open a file dialogue so 
select what file to read. Selected files are then stored in a simple 
struct that contains two fields: boolean is_open, and PathBuf path, 
it would be nice if the PathBuf could just be an Option<PathBuf> and 
remove the boolean but that got me in a fight with the borrow checker 
and the borrow checker obliterated my arse. Some simple RSX (seemingly
a Rust-ified version of JSX) plus a little bit of CSS later and I have 
three panels with a single "open file" button.

![Three blank panels](/assets/amf-blank-panels.png)
*Application without a loaded file*

Encapsulating the selected file struct in a GlobalSignal allows for 
the state of the file to be tracked, allowing us to only load the 
file data when a file is actually selected by the user. Sounds simple 
but this is where I spent my entire time fighting the borrow checker.
Creating a container div of width 27rem and using spans for individual 
bytes provides a hexadecimal byte-array that's 16 bytes in length. 
By creating a unique span for every byte we've opened ourselves to 
individually colouring each byte or even manipulating the page 
depending on which object a selected byte belongs to when clicked upon.
The code for the container and it's contents can be found below.

```rust  
rsx! {
    div {
        class: "max-w-[27rem]",
        span {
            class: "text-ctp-subtext0 hex",
            "00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F "
        }
        amf::amf_highlight::highlight_bytes {buffer, is_command: false}
    }
}
```

Works pretty well, gives a nice UI with the darkened text on top to 
allow for easy reading of the bytes. One small issue though, 
`amf::amf_highlight::highlight_bytes` is apparently my code and 
apparently I have to actually write it? 

Yeah I ain't gonna lie, I don't have the patience to explain *every* 
part of the highlighting so [here is the Github](https://github.com/Portablefire22/AMF-Viewer/blob/master/src/amf/amf_highlight.rs).
Read through the code? No? Good, because I wouldn't want to read that 
code either. Anyway lets go over the general idea of how the highlighting 
works.

Oh yea, you'll need this struct for the summary: 
```rust
#[derive(Clone)]
struct SyntaxByte {
    value: u8,
    object_id: usize,
    color: String,
}
```

1) Read the current byte.
2) Match the byte against AMF0/3 markers depending on the current encoding.
3) Perform the associated deserialisation (more info [here](https://web.archive.org/web/20190731015838/https://www.adobe.com/content/dam/acom/en/devnet/pdf/amf0-file-format-specification.pdf) and [here](https://web.archive.org/web/20210530224529/https://www.adobe.com/content/dam/acom/en/devnet/pdf/amf-file-format-spec.pdf)).
4) Append a SyntaxByte struct containing the current byte's value and assigned colour to an output buffer.
5) Iterate over the output buffer and convert the SyntaxBytes to a collection of RSX components.
6) Send that RSX collection to the hexadecimal display container.

Congrats, you've just read the summary of roughly 4 hours of work!!!
Currently, object_id is completely unused, colours are not finalised, 
and AMF3 is a thought that is yet to be implemented. The plans are 
there though, object_id is intended to be used for grouping data in 
the GUI so one can quickly visualise what data belongs to what object
(useful since objects can nest an infinite number of objects); colour 
is not my strong suit, so I'm currently deciding how to best visualise 
nested objects; and AMF3 is relatively easy to implement, with the 
only challenge being that pretty much any non-integer/floating point 
value can be re-used by an index which would require some caching.

![Version 0.0.0](/assets/amf-ver0.png)
*Version 0.0.0 of the Viewer*

# Next Steps

- Add AMF3 support.
- Add RTMP support.
- Options menu on left sidebar.
- Object inspector on right sidebar.
- Visual method of grouping nested items.