# Haml-rs

![Travis CI Build Status](https://travis-ci.org/jhartwell/haml-rs.svg?branch=master)

This is a library and CLI for parsing [Haml](http://haml.info/) templates. You are able to get Haml-rs on [Crates.io](https://crates.io/crates/hamlrs). The aim for this is to produce identical HTML to what the Ruby [Haml gem](https://rubygems.org/gems/haml) produces.

## Usage

To include haml-rs in your project add the following to your Cargo.toml:

```
[dependencies]
hamlrs = "0.1.2"
```
Then add the following to your code:

```rust
extern crate haml;
```
## Example

#### Library
```rust
extern crate haml;

use haml::Haml;

fn main() {
    let test_haml = "%span";
    let html = Haml::to_html(&test_haml);
}
```

#### CLI

`hamlrs input.haml output.html`


### Stability

This software is in its early stages and as such there may be issues with stability. While the public interface is unlikely to change from the library perspective (Haml::to_html(&str) is pretty straightforward) there will be changes behind the scenes.

If you find any bugs please don't hesitate to open an [issue](https://github.com/jhartwell/haml-rs/issues) on github or, if you want, you can reach out directly to me at jon@dontbreakthebuild.com


## Integration tests

There are currently a few integration tests (and more to come). If you are going to contribute to an integration test please make sure that the HTML file that is generated by the Ruby [Haml gem](https://rubygems.org/gems/haml) so that we can ensure that we are producing the same output as the reference implementation.

## Current limitations

There are a few limitations in this version of haml-rs. 

  * No variable support
  * Must use HTML syntax for attributes rather than Ruby syntax (so "()" instead of "{}")


## Roadmap

These are things that are on the roadmap for this project:

  * Documentation
  * Add variable support
  * Support ruby attributes
  * Add more tests
  * Improve performance (there are areas that are copying when they may not need to)


## License

This project is licensed under the [MIT license](https://github.com/jhartwell/haml-rs/blob/master/LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Haml-rs by you, shall be licensed as MIT, without any additional terms or conditions.


If you have any questions you can reach me via email at jon@dontbreakthebuild.com