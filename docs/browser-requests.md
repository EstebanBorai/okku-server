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
  kind: 'message',
  data: (new TextEncoder()).encode('Hello World!'),
  client_id: '335f6d1d-fd0b-4fd2-9009-31be7de90701'
}));
```

<details>
  <summary>Helper Function to Send Message from Browser Console</summary>

```js
function sendMessage(receiver, text) {
  ws.send(JSON.stringify({
    kind: 'message',
    data: (new TextEncoder()).encode(text),
    receiver_id: receiver
  }))
}
```
</details>

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
