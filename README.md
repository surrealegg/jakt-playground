# jakt-playground

* `sandbox` - A Docker image that is being used for running untrusted user code
* `backend` - A server to handle http requests
* `frontend` - UI of the playground

## NOTE 

This project is still in heavy development, missing features and bugs are to be expected!

## TODO

* [ ] Better documentation
* [ ] Refactoring to improve code quality.

## Building the Image

```sh
sh ./sandbox/setup.sh
```

## Starting the Server

```sh
cd backend && cargo run
```

## Starting the Frontend

```sh
cd frontend && yarn install && yarn dev
```

## Building the Frontend

```sh
cd frontend && yarn install && yarn build
```