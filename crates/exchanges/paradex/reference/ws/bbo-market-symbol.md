> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# bbo.{market_symbol}

GET /bbo.{market_symbol}

Reference: https://docs.paradex.trade/ws/web-socket-channels/bbo-market-symbol/bbo-market-symbol

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: bbo.{market_symbol}
  version: subpackage_bboMarketSymbol.bboMarketSymbol
channels:
  /bbo.{market_symbol}:
    parameters:
      market_symbol:
        description: Symbol of the market (e.g., BTC-PERP)
        schema:
          type: string
    publish:
      operationId: bbo-market-symbol-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/BboMarketSymbolSubscribe'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    BboMarketSymbolSubscribe:
      type: object
      properties:
        ask:
          type: string
          description: Best ask price
        ask_size:
          type: string
          description: Best ask size
        bid:
          type: string
          description: Best bid price
        bid_size:
          type: string
          description: Best bid size
        last_updated_at:
          type: integer
          description: Last update to the orderbook in milliseconds
        market:
          type: string
          description: Symbol of the market
        seq_no:
          type: integer
          description: Sequence number of the orderbook
      required:
        - ask
        - ask_size
        - bid
        - bid_size
        - last_updated_at
        - market
        - seq_no
      title: BboMarketSymbolSubscribe

```