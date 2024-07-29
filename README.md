# Paladin Lockup Program

![lockup program](docs/lockup_program.jpg)

This program allows for the creation of lockups that can be used to restrict
the transfer of tokens.

Lockups are created with a duration and will not allow withdrawal of the locked
tokens until the duration has passed.

### Creating a Lockup

The Lockup program uses an escrow token account owned by a
Program-Derived Address (PDA) escrow authority signer.

Users deposit funds into the escrow when they create a new lockup. They submit
the number of tokens and the lockup period as inputs to the program's
`Lockup` instruction, which will transfer the tokens into the vault and issue a
lockup - which contains information about when the tokens can be accessed.

### Withdrawing from a Lockup

The program will only issue a withdrawal when the the lockup period has ended.
This information is stored in the lockup account's state, which includes the
start and end timestamp of the lockup period, the amount of tokens locked up,
and the mint.

If a lockup period has ended, the lockup's creator (`authority`) can withdraw
the tokens using `Withdraw`.

