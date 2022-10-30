Simple rest api with tokio + axum + psqlx + postgres + jwt auth + tracing

* Before you start you have to create postgres database and set 2 env variables:
```
JWT_SECRET=
DATABASE_URL=
```
* Grab auth token:
```bash
curl --request POST 'http://localhost:3000/api/v1/auth' --data-raw '{"client_id": "client_id", "client_secret": "client_secret"}' --header 'Content-Type: application/json'
```
* Create social:
```bash
curl --request POST 'http://localhost:3000/api/v1/socials' --data-raw '{"social_type": "telegram"}' --header 'Content-Type: application/json' --header 'Authorization: Bearer {token}'
```

* Find social by id:
```bash
curl --request GET 'http://localhost:3000/api/v1/socials/{id}' --header 'Authorization: Bearer {auth_token}'
```

* You can find api tests in the [_tests folder](src/_tests)
* You can find postgres migrations in the [migrations folder](migrations)
* App errors structure can be found in [main.rs](src/main.rs)
* Request logs are printed to console:
```
2022-10-30T12:57:32.681117Z DEBUG request{method=POST uri=/api/v1/socials version=HTTP/1.1}: tower_http::trace::on_request: started processing request
2022-10-30T12:57:32.681489Z DEBUG request{method=POST uri=/api/v1/socials version=HTTP/1.1}: tower_http::trace::on_response: finished processing request latency=0 ms status=400
```
