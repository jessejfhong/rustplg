docker build `
    --tag zero2prod `
    --network=host `
    --build-arg DATABASE_URL=${env:DATABASE_URL} `
    .
