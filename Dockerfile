FROM ubuntu:18.04
EXPOSE 8080
COPY target/release/chat .
COPY front ./front
RUN chmod +x ./chat
ENTRYPOINT [ "./chat" ]
