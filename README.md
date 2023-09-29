# map-randomgen

<img width="450" alt="screenshot" src="https://github.com/0xrhsmt/map-randomgen/blob/main/docs/assets/screenshot.png">

* This project is an application that generates fully-on-chain maps randomly using Secret VRF.
* Over the past year, there has been a growing interest in full-on-chain games among some developers, with Autonomous Worlds (AW) being a key keyword.
* Anticipating the development of on-chain games not only in the Ethereum ecosystem but also in the Cosmos ecosystem, I developed this project as a PoC (Proof of Concept).
* I think randomness is a crucial element in on-chain games, and I hope this project can provide some insights for future games that utilize randomness.

## Demo Video

https://map-randomgen.vercel.app/

## Demo Video

## Flow

<img width="800" alt="screenshot" src="https://github.com/0xrhsmt/map-randomgen/blob/main/docs/assets/flow.png">

1. User calls map-randomgen contract.
2. map-randomgen contract get random number from Secret VRF.
3. Next, map-randomgen contract generates a random map by using the random number as seed.
4. map-randomgen contract return generated map to user.

## Development

### Prerequisites

* [secret network setup]([https://book.getfoundry.sh/](https://docs.scrt.network/secret-network-documentation/development/getting-started/setting-up-your-environment))
* [node (>= 18.x.x)](https://nodejs.org/en)
* [pnpm (>= 8.x.x)](https://pnpm.io/)

### Local Development

```bash
$git clone https://github.com/0xrhsmt/map-randomgen.git

# Contract Deployment
$cd contracts/deploy
$cp .env.example .env
$vim .env
$pnpm install
$pnpm run deploy

# Frontend Serve
$cd ../../frontend
$pnpm run dev
$open http://localhost:5173
```

## Acknowledges

* [Secret Network](https://scrt.network/)
* [Akash Network](https://akash.network/)
* [secret-raffle](https://github.com/writersblockchain/secret-raffle)
