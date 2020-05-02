# greendoor
A web game that is fun

## Development

Html pages should just work opened in browser for dumb dev cycle.

To start service `cd service && cargo run &` - should launch on 127.0.0.1:3030

To start evauator - TBD


## Development Rules

### Client

Vanilla ES6 no NPM or other package managers. Entrance in /index.html

window should work as really really small

### Service

Rust, no framework in memory only, minimize server side state to message replays for late joiners and storing the current session & rule. To that end all outbound messages should go to all clients

Does:
1. Serve room page ourhost.com/roomId (by room ID shared out of band)
2. shuffle websocket stuff around for exclusively for users, guesses, and score tracking (chat can happen out of band)

### Evaluator

Rust lambda that actually evaluates guess/rule combinations. Since we'll be evaluating client code server side a lambda with really minimal permissions is gonna be the way to go.
