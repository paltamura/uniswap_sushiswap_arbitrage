# Sushiswap Arbitrage by Pablo Altamura

## Get started with gitpod (1 min)
[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/paltamura/uniswap_sushiswap_arbitrage.git)

## Get started with rust intalation (10 min)

https://www.rust-lang.org/tools/install

```Bash
cargo build
```

```Bash
cargo run
```

## Results

At the end of the execution, the result with the imbalances of each path can be found in the file `imbalances.json`

```json
[
  {
    "path": {
      "efective_pairs": [
        {
          "token_in": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
          "token_out": "0x476c5e26a75bd202a9683ffd34359c0cc15be0ff",
          "address": "0xcc3d1ecef1f9fd25599dbea2755019dc09db3c54"
        },
        {
          "token_in": "0x476c5e26a75bd202a9683ffd34359c0cc15be0ff",
          "token_out": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
          "address": "0x117d4288b3635021a3d612fe05a3cbf5c717fef2"
        }
      ]
    },
    "in_wei":        1000000000000000000,
    "out_wei":       1006372545746021531,
    "imbalance_in_wei": 6372545746021531
  }
]
```

## 4 initial trials

* imbalances_0_0_0.json > Minimum token reserves = 0 | Without symmetrics = false | Operation impact = false

* imbalances_1_0_0.json > Minimum token reserves = 1 000 000 | Without symmetrics = false | Operation impact = false

* imbalances_1_1_0.json > Minimum token reserves = 1 000 000 | Without symmetrics = true | Operation impact = false

* imbalances_1_1_1.json > Minimum token reserves = 1 000 000 | Without symmetrics = true | Operation impact = true

## Data
    .
    ├── ...
    ├── data
    │   ├── abis
    │   │   ├── UniswapPairAbi.json
    │   │   └── UniswapViewAbi.json
    │   └── working_files
    │       ├── all_pairs.json                 <- All pairs info
    │       ├── selected_pair_addresses.json   <- Addresses of pairs used in selected paths
    │       ├── selected_paths.json            <- Reconbinated and filtered paths
    │       ├── tokens.json                    <- All tokens info
    │       └── uni_sushi_paths.json           <- Original paths
    │   ├── imbalances.json                    <- Results
    └── ...
