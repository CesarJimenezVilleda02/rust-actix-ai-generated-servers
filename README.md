# Rust Actix AI-Generated Servers

This repository contains an AI tool that generates actix servers bassed on a template.
These templates provide a robust starting point for building efficient, scalable, and
secure web applications.

## Description

This tool uses AI functions and prompt engineering to generate efficient code, test it and
fix it to ensure production level quality is generated on each instance. The servers
created by this tool leverage the power of Rust and the efficiency of the Actix web
framework to offer high performance and safety. Each template has been designed to be
easily customizable, allowing developers to quickly implement features specific to their
needs.

## Features

-   **High Performance**: Utilize Rust's performance benefits with Actix for fast,
    non-blocking server responses.
-   **Security**: Pre-configured security features to safeguard your applications.
-   **AI-Optimized**: Templates optimized through AI analysis to ensure best practices and
    efficient code structure.

## Installation

To get started with these templates, you will need Rust installed on your machine. If you
don't have Rust, you can install it using `rustup`, which can be downloaded from
[https://rustup.rs/](https://rustup.rs/).

1. Clone the repository:
    ```bash
    git clone https://github.com/CesarJimenezVilleda02/rust-actix-ai-generated-servers.git
    Navigate into the repository:
    bash
    Copy code
    cd rust-actix-ai-generated-servers
    Usage
    To run any of the server templates:
    ```

Navigate to the specific server directory: bash Copy code cd <server_template_directory>
Run the server: bash Copy code cargo run This will start the server on the default port as
configured in the template. You can access it from your browser or API client at
http://localhost:<port>.

### Create .env

```shell
touch .env
```

Within the .env file created, paste the following:

```plaintext
OPEN_AI_ORG=YOUR_OPEN_AI_ORG_ID
OPEN_AI_KEY=YOUR_OPEN_AI_KEY
```

### Update Paths

Update constants in the src/helpers/general path.

These should match where you have your web_template project saved. Recommend to save your
web_template in the same folder as this project.

Web template project: https://github.com/CesarJimenezVilleda02/gippity_server_template

These should link to a code template which you want your web server to use and the main.rs
file where it will attempt to execute new code it writes.

### Build Project

```shell
cargo build
```

### Run Project

```shell
cargo run
```
