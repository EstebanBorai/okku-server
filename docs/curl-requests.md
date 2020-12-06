# cURL Request Snippets for Endpoints

## Log In User

```bash
curl -i -u user:user http://127.0.0.1:3000/auth/login
```

## Sign Up User

```bash
curl -i \
     -H "Content-Type: application/json" \
     -d '{"name": "user", "password": "user"}' \
     http://127.0.0.1:3000/auth/signup
```

## Upload User Avatar

```bash
curl --verbose \
     -X POST \
     -H "Content-Type: multipart/form-data" \
     -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiOGNlNjJhZmUtMTNhNi00MDE5LWEyOWEtNGI3OTI1NzFhNTcwIn0.DHPtx021HIYD_-I6AjZ3A0LeorVc_B0KbCOd67skunY" \
     -F "image=@test.jpg" \
     -o response_file.png \
     http://127.0.0.1:3000/api/v1/users/avatar/8ce62afe-13a6-4019-a29a-4b792571a570
```

## Download User Avatar

```bash
curl --verbose \
     -X GET \
     -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiOGNlNjJhZmUtMTNhNi00MDE5LWEyOWEtNGI3OTI1NzFhNTcwIiwiZXhwIjoxNjA3MzE2NTE5NzM2fQ.6obI88M0FJ0wsjrFWM8wPNGjMcfxIKyJHBeQMW8jJCw" \
     -o response_file.png \
     http://127.0.0.1:3000/api/v1/users/avatar/8ce62afe-13a6-4019-a29a-4b792571a570
```

## Update User Avatar

```bash
curl -i \
     -X PUT \
     -H "Content-Type: multipart/form-data" \
     -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiOGNlNjJhZmUtMTNhNi00MDE5LWEyOWEtNGI3OTI1NzFhNTcwIn0.DHPtx021HIYD_-I6AjZ3A0LeorVc_B0KbCOd67skunY" \
     -F "image=@test.png" \
     http://127.0.0.1:3000/api/v1/users/avatar/7f855b52-4eb5-4e18-a235-faff0378b6e3
```
