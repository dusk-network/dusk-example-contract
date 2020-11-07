# Dusk Example Contract

Merkle tree implementation for a constrained execution environment

To test: `$ make test`

## Query methods

##### `read_value_squared(pos) -> Scalar`

Fetch the leaf on the provided position and return its scalar squared

##### `state() -> Scalar`

Return the state of the contract

## Transaction methods

##### `set_state_neg(pos) -> Scalar`

Fetch the leaf on the provided position and set the state of the contract to the negative scalar value of the leaf
