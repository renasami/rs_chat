# how to start db
```
cp .env.local .env
docker-compose up -d
```

# what should be written .env.local
```
POSTGRES_USER=somethingULike
POSTGRES_PASSWORD=somethingULike
POSTGRES_DB=omethingULike
```


# how to run
```
cd frontend
bun run dev
```
```
cd backend/main
cargo run
```