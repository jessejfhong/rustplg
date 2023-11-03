$user = "postgres"
$password = "postgres"
$db_name = "newsletter"
$port = 5432
$pgdata = "/var/lib/postgresql/data/pgdata"

# setup environment variable for cargo tool: sqlx
$env:DATABASE_URL = "postgres://${user}:${password}@localhost:${port}/${db_name}"

docker run -d --rm `
    -e POSTGRES_USER=$user `
    -e POSTGRES_PASSWORD=$password `
    -e POSTGRES_DB=$db_name `
    -e PGDATA=$pgdata `
    -p ${port}:5432 `
    --name postgres `
    postgres:alpine `
    postgres -N 1000
