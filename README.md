# About
`haste.rs` is a CLI to interact with [hastebin servers](https://github.com/seejohnrun/haste-server), which I wrote to get more experience with Rust.

# Installation
`cargo install --git https://github.com/bks1b/haste.rs --branch master`<br>
`cargo build`<br>
`target\debug\haste-rs args`

# Commands
`help`<br>
Display a command list.<br><br>

`about`<br>
Display project metadata and README.<br><br>

`get <key> [server] [output-path]`<br>
Retrieve a hastebin document's content, and optionally write it to a file.<br>
`<key>` - The document's URL or key.<br>
`[server]` - The hastebin server to retrieve data from. Defaults to the key URL's server or https://hasteb.in<br>
`[output-path]` - The output file's path.<br><br>

`post <input-path> [server] [raw]`<br>
Create a hastebin document from a specified file's contents.<br>
`<input-path>` - The input file's path.<br>
`[server]` - The hastebin server to post to. Defaults to https://hasteb.in<br>
`[raw]` - Whether to output a raw URL. If the argument is specified, it's interpreted as positive. Defaults to negative.