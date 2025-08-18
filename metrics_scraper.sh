#!/bin/bash

# Simple script to scrape Aptos node metrics and push to push gateway
# No code changes needed!

PUSH_GATEWAY_URL="http://localhost:9091"
METRICS_URL="http://localhost:9101/metrics"
JOB_NAME="aptos_validator"
INTERVAL=15

echo "🚀 Starting Aptos Metrics Scraper"
echo "📊 Scraping from: $METRICS_URL"
echo "📤 Pushing to: $PUSH_GATEWAY_URL/metrics/job/$JOB_NAME"
echo "⏱️  Interval: ${INTERVAL}s"
echo "=" * 50

while true; do
    echo "$(date): Scraping metrics..."
    
    # Scrape metrics from Aptos node
    if curl -s "$METRICS_URL" > /tmp/aptos_metrics.txt; then
        # Push to push gateway
        if curl -s -X POST "$PUSH_GATEWAY_URL/metrics/job/$JOB_NAME" \
           --data-binary @/tmp/aptos_metrics.txt \
           -H "Content-Type: text/plain"; then
            echo "✅ Successfully pushed metrics to push gateway"
        else
            echo "❌ Failed to push to push gateway"
        fi
    else
        echo "❌ Failed to scrape metrics from Aptos node"
    fi
    
    sleep $INTERVAL
done
