# Simple SMTP service for echor-server

## For what?
This service is needed to send a one-time code to confirm registration in echor.


## Basic command for build, run, or run tests

```
$ cargo build
$ cargo run 
$ cargo test
```

## .env structure

```
SMTP_USERNAME=<your email>
SMTP_PASSWORD=<your google app key>
FROM_EMAIL=<yout email>
```
