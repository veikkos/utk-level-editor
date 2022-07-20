# WIP / "Not-Sure-If-Ever-Finished" level editor for TK321

Working but only partial level editor written in Rust for TK321 (https://github.com/suomipelit/ultimatetapankaikki).

Implementation is done using [SDL2](https://www.libsdl.org/) library for creating window, loading assets and providing user input interface.

![Cover image](./media/cover.png)

## Features

- :heavy_check_mark: Laying wall and floor tiles
- :heavy_check_mark: Creating shadows
- :heavy_check_mark: Setting player start positions
- :heavy_check_mark: Spotlights
- :heavy_check_mark::grey_exclamation: Saving level but with hard-coded file name
- :heavy_check_mark::grey_exclamation: Loading level but with hard-coded file name
- :heavy_check_mark::grey_exclamation: Level size support is dynamic but missing UI
- :heavy_check_mark::grey_exclamation: Enemies, weapons and bullets are read-only and hard-coded to 1 each
- :heavy_check_mark::grey_exclamation: Time limit and level comment are read-only and hard-coded
- :x: Setting crates
- :x: Setting steams
- :x: Automated shadow creation
- :x: Tile fill feature
