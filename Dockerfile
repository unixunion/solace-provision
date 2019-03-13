FROM rust:1.33

ADD target/release/solace-provision /bin
#ADD target/release/ld-linux-x86-64.so.2 /lib/x86_64-linux-gnu/
#ADD target/release/libgcc_s.so.1 /lib/x86_64-linux-gnu/

ENTRYPOINT ["solace-provision"]