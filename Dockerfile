FROM ubuntu:latest
LABEL authors="sharon"

ENTRYPOINT ["top", "-b"]