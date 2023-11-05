## About
Get people in line with a virtual line.

## Development setup
1. Tools and services
    - Rust
    - Docker
    - Postgres
    - AWS SES

2. Install [Rust](https://www.rust-lang.org/tools/install) if not installed

3. Install [Docker](https://www.docker.com/products/docker-desktop/) if not installed

4. (Optional) Run `docker pull postgres:alpine` to download a copy of Postgres image

5. Run `start_postgres` script to bring up the database, listening on default port: 5432


## User stories
0. Terms
```
App user: A person or business who want to use the app to create queues.
Customer: A person who gets in a queue to get services from app user.
```

1. Sign up.
```
As a new app user.
I want to sign up.
So that I can login to the app.

- sign up using email, required email confirmation.
- sign up using phone, required confirmation vis SMS message.
- provide a unique username during signing up, which will be used as part of the queue url
```

2. Sign in.
```
As an app user.
I want to sign in.
So that I can use the app.
```

3. Create a queue.
```
As a queue owner.
I want to create multiple queues.
So that customer can get in the queue.
```

4. Enqueue
```
As a customer.
I want to get in the queue.
So that I can wait without standing in a queue.
```

5. Dequeue.
```
As an app user.
I want to dequeue customer when he/she received my service.
So that the queue will not keep growing.
```

6. Notification
```
As a customer.
I want to get notified when there is fewer people ahead of me.
So that I know how long do I have to wait.
```

7. Skip customer.
```
As an app user.
I want to skip the customer if he/she doesn't show up on time.
So that I can serve the next on in line.
```

8. Quit the queue.
```
As a customer.
I want to quit the line whenever I want.
So that I can let the app user know I am not waiting any more.
```


