# AlmostDefault
A tool for upscaling Minecraft textures, making them "less edgy" in the process. Based off the [NearlyDefault](https://www.curseforge.com/minecraft/texture-packs/nearlydefault) texture pack, and the algorithm used to create it, by Qwertz19281.

## About
This project has been a learning project for writing rust, so structure / organisation / etc. may not be perfect  
It runs as a tool, called from the command-line.  
* `i` or `input` - a target directory. The program will recurse through anything here, processing any textures it finds.  
* `o` or `output` - the directory to place processed textures into. Any folders inside the target folder, should be replicated here  
* `x` or `scale` - how much to upscale the textures by. Currently only accepts 4, 8 or 16  

### Goals / Planned Features:
* [x] CLI Argument support for pointing to directories, setting the size to upscale to
* [x] Generate textures similar to the original
* [x] Modify code to be async / multi-threaded, to process several textures at once

## Acknowledgements
Thanks again to [Qwertz19281](https://github.com/qwertz19281) for making the original pack and coming up with the script.
* Original NearlyDefault texture pack can be found [here](https://www.curseforge.com/minecraft/texture-packs/nearlydefault), original code is in the description
* Qwertz' own Rust re-write can be found [here](https://github.com/qwertz19281/nearlydefault_attempt_2020)