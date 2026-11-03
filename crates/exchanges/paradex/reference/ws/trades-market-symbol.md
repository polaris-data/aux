> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# trades.{market_symbol}

GET /trades.{market_symbol}

Reference: https://docs.paradex.trade/ws/web-socket-channels/trades-market-symbol/trades-market-symbol

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: trades.{market_symbol}
  version: subpackage_tradesMarketSymbol.tradesMarketSymbol
channels:
  /trades.{market_symbol}:
    parameters:
      market_symbol:
        description: Symbol of the market (e.g., BTC-PERP)
        schema:
          type: string
    publish:
      operationId: trades-market-symbol-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/TradesMarketSymbolSubscribe'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    ChannelsTradesMarketSymbolSubscribeSide:
      type: string
      enum:
        - BUY
        - SELL
      description: Taker side
      title: ChannelsTradesMarketSymbolSubscribeSide
    ChannelsTradesMarketSymbolSubscribeTradeType:
      type: string
      enum:
        - LIQUIDATION
        - UNWIND_TRANSFER
        - FILL
        - SETTLE_MARKET
        - RPI
        - BLOCK_TRADE
      description: >-
        Trade type: FILL, LIQUIDATION, RPI, UNWIND_TRANSFER, SETTLE_MARKET or
        BLOCK_TRADE
      title: ChannelsTradesMarketSymbolSubscribeTradeType
    TradesMarketSymbolSubscribe:
      type: object
      properties:
        block_id:
          type: string
          description: >-
            Groups the legs settled together in one block execution, one id per
            accepted offer for offer-based blocks. Empty for non-block trades.
            Delivered on the JSON encoding only, not the SBE binary encoding.
        created_at:
          type: integer
          description: Unix Millisecond timestamp at which trade was done
        id:
          type: string
          description: Unique Trade ID per TradeType
        market:
          type: string
          description: Market for which trade was done
        price:
          type: string
          description: Trade price
        side:
          $ref: '#/components/schemas/ChannelsTradesMarketSymbolSubscribeSide'
          description: Taker side
        size:
          type: string
          description: Trade size
        trade_type:
          $ref: '#/components/schemas/ChannelsTradesMarketSymbolSubscribeTradeType'
          description: >-
            Trade type: FILL, LIQUIDATION, RPI, UNWIND_TRANSFER, SETTLE_MARKET
            or BLOCK_TRADE
      required:
        - created_at
        - id
        - market
        - price
        - side
        - size
        - trade_type
      title: TradesMarketSymbolSubscribe

```