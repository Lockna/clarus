# clarus
Music player written in Rust used for learning purpose

## About
Since my attention has been drawn to audio in general lately, I've always wondered how audio codecs actually work.
With this project I want to take a closer look at that and will wrap this up in a music player.

This is mainly written for the learning effect and therefore I won't program an elaborate GUI, feel free to do so yourself if you want.

Additionally I will switch in the later course (also because of the learning effect) from cpal to an own written audiolib, because also one of the goals is to achieve bit-perfect audio playback which some audiophiles ask for ;)

## Milestones
- [ ] Fully implement a wav decoder according to spec
- [ ] Write an id3-parser so a TUI can use this to show song metadata
- [ ] Implement the flac decoder for flac support
- [ ] Add mp3 support
- [ ] Implement and switch to own audio-playback library

## Components
 - [`flac`](./crates/flac): FLAC encoder/decoder
 - [`mp3`](./crates/mp3): MP3 encoder/decoder
 - [`utils`](./crates/utils): Util crate which has utility structs and functions useful for all other crates
 - [`wav`](./crates/wav): Own wav decoding library
 - [`playback`](./crates/playback): Audio playback which aims for bit-perfect playback

## Setup
Coming soon

## Contributing
The project is still at a very early stage of development.
Contributions are welcome everytime.

## Contact
Feel free to reach out to `Lockna#5599` on Discord

## Credits
Coming soon

## License
clarus is distributed under the terms of either the Apache License (Version 2.0) or the MIT license, at the user's choice.
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
Contributions to the clarus project must be made under the terms of both licenses.
