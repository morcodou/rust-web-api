# Initial migration
docker-compose exec app diesel setup

docker-compose exec app diesel migration generate create_rustaceans

docker-compose exec app diesel migration generate create_crates

docker-compose exec app diesel migration run
