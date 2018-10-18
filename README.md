# PartyArty

A pseudo-real-time ray tracer (prtrt... party arty) written for educational purposes, in Rust.

## Goals

* [ ] Real-time preview
* [ ] Built using an Entity-Component System (ECS)
    * [ ] Using specs
* [ ] Progressive ray tracing
    * [ ] One idea is to render at one ray sample per pixel per frame update
    * [ ] Another idea is to do a random sampling of as many pixels as can be processed in one real-time frame duration and just let it refine when you are still
* [ ] Measure convergence
* [ ] Interactive camera
* [ ] Bounded-Volume Hierarchy for efficiency of ray-object intersection
* [ ] Animation by rendering until convergence threshold is reached, storing and moving on to the next frame
* [ ] Denoising

## License

The Unlicense license which provides public domain rights. See the LICENSE file for details.

This license was chosen as @petershirley's Ray Tracing in One Weekend is using the CC0 public domain license and this work is based on that wonderful material. https://github.com/petershirley/raytracinginoneweekend
