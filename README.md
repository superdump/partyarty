# PartyArty

A pseudo-real-time ray tracer (prtrt... party arty) written for educational purposes, in Rust.

## Goals

* [ ] Real-time preview
    * [x] Preview
    * [ ] Real-time updates at >= 30Hz
        * Depends on scene complexity and resolution - separate frame update from processing
* [x] Built using an Entity-Component System (ECS)
    * [x] Using specs
* [x] Progressive ray tracing
    * [x] One idea is to render at one ray sample per pixel per frame update
        * As this is sufficiently-fast, I will stop here.
    * ~~[ ] Another idea is to do a random sampling of as many pixels as can be processed in one real-time frame duration and just let it refine when you are still~~
* [ ] Bounded-Volume Hierarchy for efficiency of ray-object intersection
* [ ] Interactive camera
* [ ] Measure convergence
* [ ] Animation by rendering until convergence threshold is reached, storing and moving on to the next frame
* [ ] Denoising

## License

The Unlicense license which provides public domain rights. See the LICENSE file for details.

This license was chosen as @petershirley's Ray Tracing in One Weekend is using the CC0 public domain license and this work is based on that wonderful material. https://github.com/petershirley/raytracinginoneweekend
