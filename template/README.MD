# Rust ToDo App Server

This is a simple ToDo application server built using Actix web in Rust. It provides
RESTful APIs for managing tasks and user registration/login.

## Getting Started

### Prerequisites

-   Rust (>= 1.54.0)
-   Cargo (Rust's package manager)

### Installation

1. Clone this repository:

    ```
    git clone https://github.com/yourusername/todo-app-server.git
    ```

2. Navigate to the project directory:

```
cd todo-app-server
```

3. Build the project:

```
cargo build --release
```

### Usage

Start the server:

```
cargo run --release
```

The server will start running on http://localhost:8080.

### Features

Task Management: Create, read, update, and delete tasks. User Registration/Login: Register
new users and authenticate existing users.

## Dependencies

-   `actix-web`: A powerful web framework for Rust.
-   `actix-cors`: Middleware for enabling CORS in Actix web applications.
-   `serde`: A framework for serializing and deserializing Rust data structures.
-   `reqwest`: An HTTP client for Rust.
-   `async-trait`: A utility for defining asynchronous functions in traits.

## API Endpoints

### Tasks

-   **Create Task:**

    -   Method: `POST`
    -   URL: `/task`
    -   Body: JSON object containing task details (`id`, `name`, `completed`)

-   **Read All Tasks:**

    -   Method: `GET`
    -   URL: `/task`

-   **Read Task by ID:**

    -   Method: `GET`
    -   URL: `/task/{id}`

-   **Update Task:**

    -   Method: `PUT`
    -   URL: `/task`
    -   Body: JSON object containing updated task details (`id`, `name`, `completed`)

-   **Delete Task:**
    -   Method: `DELETE`
    -   URL: `/task/{id}`

### Users

-   **Register User:**

    -   Method: `POST`
    -   URL: `/register`
    -   Body: JSON object containing user details (`id`, `username`, `password`)

-   **Login:**
    -   Method: `POST`
    -   URL: `/login`
    -   Body: JSON object containing user credentials (`username`, `password`)
