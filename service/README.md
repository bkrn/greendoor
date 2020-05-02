## ENV

ASSETS_PATH= path to directory containing index.html and the assets directory. Absolute or relative to executable.


## Services I Service

### HTTP

**GET** `/`

200 and home page which allows you to create a game

**POST** `/new`

Create a new game and redirect the user to that resource

303
Location: `/game/{id}`


**GET** `/game/{id}`

Respond with a game page (includes userId and Secret) - this opens a websocket which has the following messages


### WS

#### Server -> Clients

`{userId: string, userName: string}`

notify change of userName

`{userId: string, endSession: bool}`

notify vote of end session

`{session: id, userId: id|null}`

create a new session and identify the rule setter where null is against the "AI"

`{sessionFailure: string, userId: id}`

Explain that someone tried to create a session but their rule wasn't valid and why

`{guess: string, userId: id, session: id, result: BORK|FAIL|PASS}`

send a guess result back, bork means not a word

#### Client -> Server

`{userId: id, userSecret: id, userName: string}`

change a userName

`{userId: id, userSecret: id, endSession: bool}`

vote to end session

`{userId: id, userSecret: id, guess: string}`

send a guess

`{userId: id, userSecret: id, rule: string}`

send a rule to create a new session

