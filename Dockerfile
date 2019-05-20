FROM rustlang/rust:nightly
MAINTAINER Matt Jack <matt@mattjack.dev>

WORKDIR /app
EXPOSE 8080

ADD ./ .

CMD ROCKET_ENV=production
CMD ROCKET_PORT=8080

RUN cargo build --release
RUN strip target/release/server

ENTRYPOINT ["cargo", "run", "--release"]
