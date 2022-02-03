# Remove local volume
rm -rf postgres_data ;
echo "Removed postgres data"

# Stop container
docker restart postgres_db &
echo "Stoping postgres container"

# Wait for container to start
sleep 15
echo "Postgres container stopped"

# Run diesel migrations
diesel migration run ;
echo "Running diesel migration"

# Run server
echo "Running project"
cargo run ;
