#!/usr/bin/env bash

set -e pipefail
anchor build --skip-lint
cp -v ./target/types/mango_v3_reimbursement.ts ./ts/client/src/mango_v3_reimbursement.ts
yarn tsc
