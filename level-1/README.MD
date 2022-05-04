### Overview

The goal of this level is to find typical bugs you would see in Solana programs (smart contracts) when working with basic CRUD(minus the D) type of apps. There are probably more bugs than the ones I intentionally created. If you find any additional bugs, publish your findings and tag me via twitter @wzrdk3lly.

Intended Functionality
- Only the authority/owner or person who initializes an account is able to increment the account and set the data for the account. 

### Instructions 
1. `cd` into the level-1 root directory and build the program with `anchor build`

2. Read through the code in lib.rs to see if you find any bugs and then experiment with a few test in level-1.ts. I included some boilerplate code to get your test started. 

3. Run the test with Anchor Test 
