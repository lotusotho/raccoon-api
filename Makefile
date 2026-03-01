build:
	cargo build
run:
	cargo run
clean:
	cargo clean
docker-sh:
	docker exec -it raccoon-api sh
docker-build:
	docker build -t raccoon-api .
docker-run:
	docker compose up -d