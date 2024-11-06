# crossxmint

#Quickstart Instructions
1. Clone the repository
2. Build using cargo build
3. Run using:
    Phase 1: cargo run -- --phase Phase1
    Phase 2: cargo run -- --phase Phase2
    (aliases are also allowed like: phase1, p1, 1, phase2, p2, 2)

Please enjoy the program :D


#API Calls
1. Polyanet
Method: Post/Delete
Endpoint: /api/polyanets
Body Parameters: candidateId(string), row(integer), column(integer)

2. Soloon
Method: Post
Endpoint: /api/soloons
Body Parameters: candidateId(string), row(integer), column(integer), color(string): blue, red, purple, white

Method: Delete
Endpoint: /api/soloons
Body Parameters: candidateId(string), row(integer), column(integer)

3. Cometh
Method: Post
Endpoint: /api/comeths
Body Parameters: candidateId(string), row(integer), column(integer), direction(string): up, down, left, right

Method: Delete
Endpoint: /api/comeths
Body Parameters: candidateId(string), row(integer), column(integer)

