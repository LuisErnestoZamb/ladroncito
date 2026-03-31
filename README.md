# Ladroncito

This repository find the routes given by transactions in a CSV file. You need to modify the file config.yaml to instruct what are the files and wallets to be reviewed.

```bash
# accounts_path is not being used currently but will have additional information that the transactions_path file don't have.
accounts_path: data/accounts_fake_test.csv
# transactions_path is the current file to be read under current code.
transactions_path: data/transacciones_complejas.csv
# Wallet that you need to track
initial_wallet: TL0_Node7_x92Jk
# Until what wallet you will scan
final_wallet: TL4_Node5_wQ11
```

```bash
cargo run
```

## Web interface

