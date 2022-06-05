FROM rustlang/rust:nightly

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=staging

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ./target/release/rocket