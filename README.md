# jakt-playground

* `frontend` - is a single page application that connects to the backend url specified in .env
* `backend` - is a http server that handles the requests by passing the input to the one time docker container
* `sandbox` - contains the Dockerfile and the scripts to set up the image

## Building Instructions

### Windows/WSL2

TODO

### Prerequisites

* Rust
* Jakt
* Node.js (yarn)
* Docker


### Setting up Jakt

If you don't have jakt installed run those commands:
```bash
git clone https://github.com/SerenityOS/jakt
cd jakt
cargo install --path .
```

Make sure to set the enviorment `JAKT_HOME`:
```bash
export JAKT_HOME="/path/to/jakt"
```

### Starting the backend

If you don't have rust installed run this command:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

And you are set! To run it run:
```bash
cd backend
cargo run
```

This project can accept this arguments
* `ALLOW_ORIGIN` - sets `access-control-allow-origin` header (default: *)
* `PORT` - sets server's port (default: 8080)

You can pass those arguments like this:
```bash
ALLOW_ORIGIN="https://example.com" PORT="4242" cargo run
```

### Building the Docker image

Make sure you have Docker installed:
```bash
sudo apt update
sudo apt install docker.io
sudo systemctl start docker
```

Then you only need to run this shell. This will create an image called `jakt_sandbox`:
```bash
bash sandbox/setup.sh
```

You are free to run this command to clean up unused images:
```bash
docker image prune
```

### Building the UI

If you don't have node.js installed, run these commands:
```bash
# Install Node Version Manager
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash

# Install the latest version
nvm install node

# Install yarn
npm -g install yarn
```

Install the packages:
```
cd frontend
yarn install
```

Create `.env` file and change `VITE_SERVER_URL` to the backend url:
```bash
VITE_SERVER_URL=http://localhost:8080
```

and run it:
```bash
yarn dev
```

or build the project:
```bash
yarn build
```