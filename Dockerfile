FROM rust:1.70.0

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

ENV PROD=1

CMD [ "fido" ]