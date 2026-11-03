> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# funding_data.{market_symbol}

GET /funding_data.{market_symbol}

Reference: https://docs.paradex.trade/ws/web-socket-channels/funding-data-market-symbol/funding-data-market-symbol

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: funding_data.{market_symbol}
  version: subpackage_fundingDataMarketSymbol.fundingDataMarketSymbol
channels:
  /funding_data.{market_symbol}:
    parameters:
      market_symbol:
        description: Symbol of the market (e.g., BTC-PERP)
        schema:
          type: string
    publish:
      operationId: funding-data-market-symbol-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/FundingDataMarketSymbolSubscribe'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    FundingDataMarketSymbolSubscribe:
      type: object
      properties:
        created_at:
          type: integer
          description: Timestamp in milliseconds when the funding data was created
        funding_index:
          type: string
          description: Current funding index value as a decimal string
        funding_period_hours:
          type: number
          format: double
          description: Actual funding period in hours for this market
        funding_premium:
          type: string
          description: Current funding premium as a decimal string
        funding_rate:
          type: string
          description: Raw funding rate for the actual funding period as a decimal string
        funding_rate_8h:
          type: string
          description: Normalized 8-hour funding rate as a decimal string
        impact_premium_rate:
          type: string
        market:
          type: string
          description: Market represents the market identifier
      required:
        - created_at
        - funding_index
        - funding_period_hours
        - funding_premium
        - funding_rate
        - funding_rate_8h
        - market
      title: FundingDataMarketSymbolSubscribe

```