FROM alpine:latest AS nav-pkg
RUN apk add unzip wget curl
RUN mkdir /pkg/ ; \
  curl https://github.com/itacentury/HomepageRs/releases/download/pkg/pkg.zip | tar xvzf - -C /pkg  

FROM rust:1.86.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

WORKDIR /app
COPY . .
COPY  --from=nav-pkg /pkg/ /static/pkg/

RUN cargo build --release

CMD ["cargo", "run", "--release"]