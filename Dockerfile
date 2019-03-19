FROM debian:stable-slim

ADD target/release/solace-provision /bin
ADD entrypoint.sh /

RUN apt-get update && apt-get -y install openssl

ENTRYPOINT ["/entrypoint.sh"]