FROM rust:alpine

WORKDIR /app

COPY . .

CMD ["cargo", "run"]
