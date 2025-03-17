FROM debian:latest

RUN apt-get update && apt-get install -y sqlite3 curl

WORKDIR /app

COPY ./target/dx/todo/release/web/server /app/server
COPY ./target/dx/todo/release/web/public /app/public

COPY ./migration/init.sql /app/init.sql

RUN sqlite3 /app/todo.db < /app/init.sql

RUN chmod +x /app/server

EXPOSE 8080

CMD ["/app/server"]
