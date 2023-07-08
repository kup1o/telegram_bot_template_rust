FROM rust:1.70 as builder
WORKDIR /usr/src/telegram_bot_template_rust
COPY . .
RUN cargo install --path .
CMD ["telegram_bot_template_rust"]
