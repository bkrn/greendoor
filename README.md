# greendoor
A web game that is fun


## Development Rules

### Client

Vanilla ES6 no NPM or other package managers. SPA with entrance in /index.html

window should work as really really small

### Service

Rust, no framework in memory only, minimize server side state

Does:
1. Serve room page ourhost.com/roomId (by room ID shared out of band)
3. shuffle websocket stuff around for exclusively for users, guesses, and score tracking (chat can happen out of band)
