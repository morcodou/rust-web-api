docker-compose exec app cargo run
docker-compose exec app cargo build

docker-compose exec app curl 127.0.0.1:8000/rustaceans
