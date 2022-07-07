# lustre-rs: Toy RT Renderer

[![Rust CI](https://github.com/nbarrios1337/lustre-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/nbarrios1337/lustre-rs/actions/workflows/rust.yml)

Learning Rust via [Peter Shirley's Ray Tracing in One Weekend](https://raytracing.github.io/) Book series and possibly other sources.

## Usage

1. If you don't have Rust installed, take a look at [Rust's Getting Started page](https://www.rust-lang.org/learn/get-started).
2. Clone this repository:

    ```shell
    git clone git@github.com:nbarrios1337/lustre-rs.git
    ```

3. Build `lustre` by running:

    ```shell
    cargo build --release
    ```

4. Run `lustre` by specifying an output image file:

   ```shell
   ./target/release/lustre -o image.png
   ```

   See `lustre --help` for more options.

## Progress

- [x] Implementing Book 1: [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) - 100%
- [x] Documenting Book 1 - 100%
- [ ] Implementing Book 2: [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html) - 12%
- [ ] Documenting Book 2 implementation
- [ ] Implementing Book 3: [Ray Tracing: The Rest of Your Life](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)
- [ ] Documenting Book 3 implementation
- [ ] Look into other ways to expand this renderer. Possibilties:
  - Integration with shaders
  - Integration with graphics APIs
  - Rendering in realtime
  - ...

## Additional Sources

- [Peter Shirley's "In One Weekend" Blog](https://in1weekend.blogspot.com/), serving as addendums to his aforementioned book series.
