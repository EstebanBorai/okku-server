# Browser Request Snippets

## Log In User

```javascript
fetch('http://127.0.0.1:3000/auth/login', {
  method: 'GET',
  headers: {
    'authorization': `Basic ${btoa('stevejobs:root')}`,
    'content-type': 'application/json'
  },
  mode: 'cors'
});
```

## Sign Up User

```javascript
fetch('http://127.0.0.1:3000/auth/signup', {
  method: 'POST',
  body: JSON.stringify({ name: 'stevejobs', password: 'root' }),
  headers: {
    'content-type': 'application/json'
  },
  mode: 'cors'
});
```

## Open WebSocket Connection

```javascript
const ws = new WebSocket(`ws://127.0.0.1:3000/chat?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiZjlhYjU3ODEtYTJkNC00ZWY4LWIwMWEtYjhjZjI1MGFiMzg0IiwiZXhwIjoxNjA4MTMyMTc4MjYxfQ.Tre0-f_NOrx1I6RgzahiSVMUwVDHTlq2XKRTMmFCL8M`);
```

## Join

```javascript
ws.send(JSON.stringify({
  type: 'join',
  payload: {
    name: 'stevejobs'
  }
}));
```

## Send Message

```javascript
ws.send(JSON.stringify({
  type: 'post',
  payload: {
    body: 'Hello from msend!'
  }
}));
```

## Receive Messages

```javascript
ws.onmessage = function(event) {
  console.log(event.data);
};
```

## Disconnect

```javascript
ws.close();
```
