# Alex Mackay 2024

clean:
	@cargo clean

build:
	@cargo build -r

run: build
	@./target/release/psql-ledger-rst run

test: 
	@cargo test

# Must have Docker installed on the host machine

docker:
	@cd docker && ./build-docker.sh && cd ..

docker-compose: docker
	@cd docker-compose && docker-compose up


# For local stack testing
# Assummes that docker is installed on the host machine 

postgresup:
	docker run --name postgres -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:latest

postgresdown:
	docker kill postgres && docker remove postgres

createdb:
	docker exec -it postgres createdb --username=root --owner=root bank

dropdb: 
	docker exec -it postgres dropdb --username=root bank

# Requires migrate installation: https://github.com/golang-migrate/migrate/tree/master/cmd/migrate
migrateup:
	migrate -path sql/migrations -database "postgresql://root:secret@localhost:5432/bank?sslmode=disable" -verbose up

migratedown:
	migrate -path sql/migrations -database "postgresql://root:secret@localhost:5432/bank?sslmode=disable" -verbose down

benchmarks:
	cargo bench

.PHONY: build run test docker docker-compose postgresup postgresdown createdb dropdb migrateup migratedown 