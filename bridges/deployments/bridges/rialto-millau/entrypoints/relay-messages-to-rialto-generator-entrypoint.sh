#!/bin/bash

# THIS SCRIPT IS NOT INTENDED FOR USE IN PRODUCTION ENVIRONMENT
#
# This scripts periodically calls the Substrate relay binary to generate messages. These messages
# are sent from the Millau network to the Rialto network.

set -eu

# Max delay before submitting transactions (s)
MAX_SUBMIT_DELAY_S=${MSG_EXCHANGE_GEN_MAX_SUBMIT_DELAY_S:-30}
MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE=128

SHARED_CMD=" /home/user/substrate-relay send-message millau-to-rialto"
SHARED_HOST="--source-host millau-node-bob --source-port 9944"
SOURCE_SIGNER="--source-signer //Rialto.MessagesSender"

SEND_MESSAGE="$SHARED_CMD $SHARED_HOST $SOURCE_SIGNER"

SOURCE_CHAIN="Millau"
TARGET_CHAIN="Rialto"
EXTRA_ARGS=""
REGULAR_PAYLOAD="020419ac"
BATCH_PAYLOAD="020419ac"

source /common/generate_messages.sh
