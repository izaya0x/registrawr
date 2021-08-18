# RegistRAWR

A destributed registry for DApp frontends built on Ethereum and IPFS.

The goal of registRAWR is to provide a distributed way of accessing OpenSource frontends for dapps. While the smart contracts themselves cannot be taken down or censored frontend and hosting is still vulnerable. RegistRAWR aims to solve that problem by ustilizing distributed technologies to guarantee access to dapp frontends.

RegistRAWR is still in very early stages so feel free to look around the GitHub and hack on the project.

RegistRAWR is not yet deployed on any testnets so if you would like to hack on the project please deploy it to a local hardhat network. This document will be updated when the project is deployed to a testnet.

## Architecture

registRAWR is made up of three main components, registrawr-core library that interfaces with the smart contracts, Registrawr the smart contract, and IPFS.

### Smart Contract

The registry's backbone is made up of an Ethereum smart contract that handles the tracking of what software has been published and IPFS that hosts the published files. Each published software is represented by an NFT that is minted to the publishing addresss. Ownership of a registry entry can be transferred between addresses just as normal NFTs.

At the time of writing properties of the NFT are pretty standard though there are ideas to expand the features before the service goes live. Check out the project plan on GitHub for more details.

### IFPS

IPFS is being used to host the compressed version of the publishded software. The miniumum amount of data is published to IPFS to save space and keep transfer times down. Each dapp is run through it's build process and only the built artifacts are published to IPFS. If full source is desired check out the project's GitHub page.

A transition to web3.storage is currently planned but on hold until there is a usable REST API.

### registrawr-core

Registrawr-core is the Rust library that interfaces with the :w
registrawr smart contract and IFPS. It is mainly interacted with via the CLI but could be embedded in other projects if so desired.

There are plans to have a Neon based NodeJS wrapper for registrawr-core so it can be used in other contexts such as an Electron desktop application.
