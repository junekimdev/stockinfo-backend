#!/bin/bash

# Set path for commands
PATH=/usr/local/bin:/usr/bin:/bin

# Clear prices data
curl -X DELETE https://stockinfo.junekim.xyz/api/v1/prices &> /dev/null
curl -X DELETE https://stockinfo.junekim.xyz/api/v1/prices_us &> /dev/null

# Add exec permission
# chmod +x clear_prices.sh
