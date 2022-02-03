## Start the Postgres DB
Let's start with the postres container that will save all data on the postgres_data volume

```sh
docker compose up --build -d
```

If you want to check if the database is up and running do a `docker ps` and then display the container's logs by a `docker logs 3290f9616b1b`
```sh
$ docker ps

CONTAINER ID   IMAGE             COMMAND                  CREATED         STATUS         PORTS                    NAMES
3290f9616b1b   postgres:latest   "docker-entrypoint.s…"   8 seconds ago   Up 6 seconds   0.0.0.0:5432->5432/tcp   postgres_db


$ docker logs 3290f9616b1b

2021-12-23 10:00:52.062 UTC [1] LOG:  starting PostgreSQL 14.1 (Debian 14.1-1.pgdg110+1) on x86_64-pc-linux-gnu, compiled by gcc (Debian 10.2.1-6) 10.2.1 20210110, 64-bit
2021-12-23 10:00:52.062 UTC [1] LOG:  listening on IPv4 address "0.0.0.0", port 5432
2021-12-23 10:00:52.062 UTC [1] LOG:  listening on IPv6 address "::", port 5432
2021-12-23 10:00:52.069 UTC [1] LOG:  listening on Unix socket "/var/run/postgresql/.s.PGSQL.5432"
2021-12-23 10:00:52.118 UTC [1] LOG:  database system is ready to accept connections
```

## Set the Diesel schema
First thing, lets create an .env with your postgress data
```sh
echo DATABASE_URL=postgres://root:1234@localhost/chronos > .env
```

Once this is done we can create the migrations running
```sh
diesel migration run
```