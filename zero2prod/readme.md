## Learn Rust by following the book [Zero2Prod](https://www.zero2prod.com/)

### Steps for introducing changes to database schema
1. Create migration
    `sqlx migrate add add_user_table`

2. Edit the sql file create in the first step

3. Run migration
    `sqlx migrate run`

