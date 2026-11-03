> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# funding_payments.{market_symbol}

GET /funding_payments.{market_symbol}

Reference: https://docs.paradex.trade/ws/web-socket-channels/funding-payments-market-symbol/funding-payments-market-symbol

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: funding_payments.{market_symbol}
  version: subpackage_fundingPaymentsMarketSymbol.fundingPaymentsMarketSymbol
channels:
  /funding_payments.{market_symbol}:
    parameters:
      market_symbol:
        description: Symbol of the market (e.g., BTC-PERP)
        schema:
          type: string
    publish:
      operationId: funding-payments-market-symbol-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/FundingPaymentsMarketSymbolSubscribe'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    FundingPaymentsMarketSymbolSubscribe:
      type: object
      properties:
        account:
          type: string
          description: Account that made the payment
        created_at:
          type: integer
          description: Funding payment time
        fill_id:
          type: string
          description: Fill id that triggered the payment (if any)
        id:
          type: string
          description: Unique string ID to identify the payment
        index:
          type: string
          description: Value of the funding index at the time of payment
        market:
          type: string
          description: Market against which payment is made
        payment:
          type: string
          description: Payment amount in settlement asset
      required:
        - account
        - created_at
        - fill_id
        - id
        - index
        - market
        - payment
      title: FundingPaymentsMarketSymbolSubscribe

```