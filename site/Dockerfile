FROM rust:1.86.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]