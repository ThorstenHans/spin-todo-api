# Spin ToDo API

This [Spin](https://github.com/spinframework/spin) application exposes a super simple RESTful API for managing ToDo items.

The app uses key-value store for persisting items.

## API Endpoints

The following endpoints are exposed:

- `GET /api/todos` to retrieve the list of ToDo items
- `GET /api/todos/:id` to retrieve a single ToDo item using its identifier
- `POST /api/todos` to add a new ToDo item
- `POST /api/todos/:id/toggle` to toggle the state of a particular ToDo item using its identifier
- `DELETE /api/todos/:id` to delete a ToDo item using its identifier

## Building & Running the Spin application

To build and run the Spin application on your local machine, you must have the following installed:

- Spin CLI (`spin`) in version `3.3.1` or newer
- Rust in version `1.86.0` or newer including the `wasm32-wasip1` target

Run `spin build` to compile the application.

Run `spin up` to launch the application on your local machine (you can overwrite the default listener `localhost:3000` by providing the `--listen` argument).
