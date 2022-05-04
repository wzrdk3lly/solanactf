### Overview

The goal of this level is to find typical bugs you would see in Solana programs (smart contracts) when working with basic CRUD(minus the D) type of apps. There are probably more bugs than the ones I intentionally created. If you find any additional bugs, publish your findings and tag me via twitter @wzrdk3lly.

Intended Functionality
- Only the authority/owner or person who initializes an account is able to increment the account and set the data for the account. 

### Instructions 
1. `cd` into the level-1 root directory and build the program with `anchor build`

2. Replace the program id in lib.rs and in anchor.toml with the new program id. 

3. Read through the code in lib.rs to see if you find any bugs and then experiment with a few test in level-1.ts. I included some boilerplate code to get you started. 

4. Run the test with `anchor test`


### Resources 
1. [The Anchor Book](https://book.anchor-lang.com/)

2. [Sealevel attacks](https://github.com/project-serum/sealevel-attacks/tree/master/programs)

3. [Neyodyme's Smart Contract Pitfalls](https://blog.neodyme.io/posts/solana_common_pitfalls)


