# PartyArty

A pseudo-real-time ray tracer (prtrt... party arty) written for educational purposes, in Rust.

## Goals

* [x] Real-time preview
    * [x] Preview
    * [x] Real-time updates at >= 30Hz
        * The number of samples to process per frame is dynamically limited, targeting a specified preview frame rate.
* [x] Built using an Entity-Component System (ECS)
    * [x] Using specs
* [x] Progressive ray tracing
    * [x] One idea is to render at one ray sample per pixel per frame update
        * This is sufficiently-fast for simple scenes. The following point is a better choice.
    * [ ] Use blue noise to do a pseudorandom sampling of as many pixels as can be processed in one real-time frame duration, with a lower-bound of samples / frame
    * Clear the sample history when the camera or scene changes
* [ ] Bounded-Volume Hierarchy for efficiency of ray-object intersection
* [ ] Interactive camera
* [ ] Measure convergence
* [ ] Animation by rendering until convergence threshold is reached, storing and moving on to the next frame
* [ ] Denoising

## License

The Unlicense license which provides public domain rights. See the LICENSE file for details.

This license was chosen as @petershirley's Ray Tracing in One Weekend is using the CC0 public domain license and this work is based on that wonderful material. https://github.com/petershirley/raytracinginoneweekend
