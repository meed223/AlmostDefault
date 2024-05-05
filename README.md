# AlmostDefault
A tool for upscaling Minecraft textures, making them "less edgy" in the process. Based off the [NearlyDefault](https://www.curseforge.com/minecraft/texture-packs/nearlydefault) texture pack, and the algorithm used to create it, by Qwertz19281.

## About
This project has been a learning project for writing rust, so structure / organisation / etc. may not be perfect.  

### ToDo:
* [ ] Try and clear up any build-warnings
* [ ] Update fs & resource-code so it can be pointed at a root Minecraft directory (from an unpacked version jar)
  * [ ] Do Simple-copy colormap folder
  * [ ] Ignore / Simple-Copy background folder (gui > title > background)
* [ ] Try and resolve any to-dos
* [ ] Push initial version to master branch

### Goals / Planned Features:
* [x] CLI Argument support for pointing to directories, setting the size to upscale to
* [x] Generate textures similar to the original
* [ ] Extend to produce a usable pack - i.e. generate the expected pack structure, pack-info files, etc.
* [ ] Modify code to be async / multi-threaded, to process several textures at once (*stretch goal*)

## Acknowledgements
Thanks again to [Qwertz19281](https://github.com/qwertz19281) for making the original pack and coming up with the script.
* Original NearlyDefault texture pack can be found [here](https://www.curseforge.com/minecraft/texture-packs/nearlydefault), original code is in the description
* Qwertz' own Rust re-write can be found [here](https://github.com/qwertz19281/nearlydefault_attempt_2020)