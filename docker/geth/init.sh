#!/bin/sh

set -e

DEFAULT_GETH_ARGS=' --datadir "./l2geth-datadir"'
GENESIS_GENERATED='./docker/geth/templates/l2-testnet1.json'
GENESIS='./docker/geth/templates/l2-testnet.json'
GENESIS_TEMPLATE="$GENESIS"
MINER_PRIV_KEY=9fd4abb4a4e78804ae4b40fbab6d53355fffc701da2dbd9be567ce52bca22fca
MINER_ADDRESS=1c1860870a362c3B7eD3F278347c4F20B1eA4953

if [[ ! -e /root/.ethereum/geth ]]; then
  echo 'init chain'
  cat "$GENESIS_TEMPLATE" | sed "s/MINER_ADDRESS/$MINER_ADDRESS/g" > $GENESIS_GENERATED
  geth $DEFAULT_GETH_ARGS init $GENESIS_GENERATED
fi

if [[ ! -z $MINER_PRIV_KEY ]]; then
  geth $DEFAULT_GETH_ARGS --exec 'try { personal.importRawKey("'$MINER_PRIV_KEY'", null) } catch (e) { if (e.message !== "account already exists") { throw e; } }' console
fi

if [[ ! -z $BOOTNODE ]]; then
  cat > /geth.toml << EOF
[Node.P2P]
BootstrapNodes = ["$BOOTNODE"]
StaticNodes = ["$BOOTNODE"]
EOF

  DEFAULT_GETH_ARGS="$DEFAULT_GETH_ARGS --config /geth.toml"
fi

exec geth $DEFAULT_GETH_ARGS $@
