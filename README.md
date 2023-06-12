[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/pascal-chenevas/carbone_sdk_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/pascal-chenevas/carbone_sdk_rs/actions/workflows/rust.yml)
[![unstable](http://badges.github.io/stability-badges/dist/unstable.svg)](http://github.com/badges/stability-badges)

# carbone_sdk_rs

carbone_sdk_rs is a Library which supply functionalites to communicate with the [Carbone API](https://carbone.io/api-reference.html).

# Process to render a new report

```mermaid
sequenceDiagram
    Client->>Carbone API: send a template file to /template
    Carbone API-->>Client: send a template_id 
    Client->>Carbone API: send json data to be rendered to /render/{template_id}
    Carbone API-->>Client: send a render_id
    Carbone API-->>Client: get the rendered report from /render/{render_id}
```

# Installation

TODO

# Render a report

TODO

# References

[Carbone.io](https://carbone.io) a report generator.

## Useful links

- [How to build a template file](https://carbone.io/documentation.html#building-a-template)

- [Substitutions](https://carbone.io/documentation.html#substitutions)

- [Repetitions](https://carbone.io/documentation.html#repetitions)

- [Formatters](https://carbone.io/documentation.html#formatters)

- [Translations](https://carbone.io/documentation.html#translations)
