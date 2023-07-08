# Template for a Telegram bot written in Rust with Teloxide library
make sure to setup an environment variable `TELOXIDE_TOKEN`

## HOW TO RUN
### Cargo
```
TELOXIDE_TOKEN=000:AAA cargo run
```
### Docker
```bash
# Build a docker image with the name `bot`
docker build -t bot .

# Start a container with the image `bot`
docker run -d -e TELOXIDE_TOKEN=000:AAA bot
```
