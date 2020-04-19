FROM rust:1.24.1

COPY src/main.rs /src/Solution.rs
COPY src/*.txt /src/
WORKDIR /src

RUN rustc -C opt-level=3 -o Solution Solution.rs
RUN cat input.txt | ./Solution > result.txt && diff -u output.txt result.txt