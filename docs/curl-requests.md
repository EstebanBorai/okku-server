# cURL Request Snippets for Endpoints

## Log In User

```bash
curl -i  -u user:user http://127.0.0.1:3000/auth/login
```

## Sign Up User

```bash
curl -i -H "Content-Type: application/json" -d '{"name": "user", "password": "user"}' http://127.0.0.1:3000/auth/signup
```

## Upload User Avatar

```bash
curl -i -X POST -H "Content-Type: multipart/form-data" -F "image=@test.png" http://127.0.0.1:3000/api/v1/users/avatar/89d5de0e-4108-447d-aff1-0f8d0dfa0284
```

## Download User Avatar

```bash
curl -i http://127.0.0.1:3000/api/v1/users/avatar/7f855b52-4eb5-4e18-a235-faff0378b6e3
```

## Update User Avatar

```bash
curl -i -X PUT -H "Content-Type: multipart/form-data" -F "image=@test.png" http://127.0.0.1:3000/api/v1/users/avatar/7f855b52-4eb5-4e18-a235-faff0378b6e3
```
