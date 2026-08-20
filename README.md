# AlmostDefault
A tool for upscaling Minecraft textures, making them "less edgy" in the process. Based off the [NearlyDefault](https://www.curseforge.com/minecraft/texture-packs/nearlydefault) texture pack, and the algorithm used to create it, by Qwertz19281.

## About
This project is originally a learning project for Rust, but now also for Uiua. The intent is to produce a simple to use CLI tool that runs effectively.

### Usage
* `i` or `input` - a target directory. The program will recurse through anything here, processing any textures it finds.  
* `o` or `output` - the directory to place processed textures into. Any folders inside the target folder, should be replicated here  
* `x` or `scale` - how much to upscale the textures by. Currently only accepts 4, 8 or 16  

### Goals / Planned Features (2026):
#### Primary  
* [ ] Core image manipulation logic re-written to use Uiua inside Rust.
* [ ] Effectively multi-threading / parallel processing of textures.
#### Stretch  
* [ ] Improved usage documentation
* [ ] Improve handling of folders of resources so it's easier to sic on whole resource packs/ mod textures, which have varied structure
  
## Acknowledgements
Thanks to [Qwertz19281](https://github.com/qwertz19281) for making the original pack and coming up with the script.
* Original NearlyDefault texture pack can be found [here](https://www.curseforge.com/minecraft/texture-packs/nearlydefault), original code is in the description
* Qwertz' own Rust re-write can be found [here](https://github.com/qwertz19281/nearlydefault_attempt_2020)