# mango-v3-reimbursement

This repository contains code that allows users of mango v3 to receive re-imbursement after the emergency shutdown in Slot 154,889,307. 

## Deployment

First calculate the re-imbursement from a recent solana snapshot, we used Slot 154,856,403:
- download the [solana snapshot](https://drive.google.com/file/d/1nYJjW0n2pSpAOwf7kUR_p-Cj2PpS3kcn/view?usp=sharing)
- download the [deposits & withdrawals](https://docs.google.com/spreadsheets/d/1DwtllQeCw3j9-DjNFgxSk_Gl_L8405W9ExjeqvKVjds/edit#gid=0) since block as tsv
- compile #208 on v3, run cargo --bin cli equity-from-snapshot snapshot-154856403-HJj8oytGGRnbUoFdojewzBWB3FrTBaTJdx1v4g63oUWc.sqlite snapshot-withdraws.tsv mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68 98pjRuQjK3qA6gXts96PqZT4Ze5QmnCmt3QYjhbUSPue
