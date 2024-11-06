# Crossxmint

## Quickstart Instructions

1. **Clone the repository**.
2. **Build using** `cargo build`.
3. **Run using**:
   - **Phase 1**: `cargo run -- --phase Phase1`
   - **Phase 2**: `cargo run -- --phase Phase2`

   _(Aliases are also allowed, such as: phase1, p1, 1, phase2, p2, 2)_

Please enjoy the program! :smiley:

## API Calls

### 1. Polyanet
- **Method**: POST/DELETE
- **Endpoint**: `/api/polyanets`
- **Body Parameters**:
  - `candidateId` (string)
  - `row` (integer)
  - `column` (integer)

### 2. Soloon
- **Method**: POST
  - **Endpoint**: `/api/soloons`
  - **Body Parameters**:
    - `candidateId` (string)
    - `row` (integer)
    - `column` (integer)
    - `color` (string): _blue, red, purple, white_

- **Method**: DELETE
  - **Endpoint**: `/api/soloons`
  - **Body Parameters**:
    - `candidateId` (string)
    - `row` (integer)
    - `column` (integer)

### 3. Cometh
- **Method**: POST
  - **Endpoint**: `/api/comeths`
  - **Body Parameters**:
    - `candidateId` (string)
    - `row` (integer)
    - `column` (integer)
    - `direction` (string): _up, down, left, right_

- **Method**: DELETE
  - **Endpoint**: `/api/comeths`
  - **Body Parameters**:
    - `candidateId` (string)
    - `row` (integer)
    - `column` (integer)
