> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# markets[.{market_symbol}]

GET /markets[.{market_symbol}]

Reference: https://docs.paradex.trade/ws/web-socket-channels/markets-market-symbol/markets-market-symbol

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: markets[.{market_symbol}]
  version: subpackage_marketsMarketSymbol.marketsMarketSymbol
channels:
  /markets[.{market_symbol}]:
    parameters:
      market_symbol:
        description: Symbol of the market (e.g., BTC-PERP)
        schema:
          type: string
    publish:
      operationId: markets-market-symbol-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/MarketsMarketSymbolSubscribe'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    ChannelsMarketsMarketSymbolSubscribeChangedFieldsDelta1CrossMarginParams:
      type: object
      properties: {}
      title: ChannelsMarketsMarketSymbolSubscribeChangedFieldsDelta1CrossMarginParams
    ChannelsMarketsMarketSymbolSubscribeChangedFieldsFeeConfig:
      type: object
      properties: {}
      title: ChannelsMarketsMarketSymbolSubscribeChangedFieldsFeeConfig
    ChannelsMarketsMarketSymbolSubscribeChangedFieldsOptionCrossMarginParams:
      type: object
      properties: {}
      title: ChannelsMarketsMarketSymbolSubscribeChangedFieldsOptionCrossMarginParams
    ChannelsMarketsMarketSymbolSubscribeChangedFields:
      type: object
      properties:
        asset_kind:
          type: string
        base_currency:
          type: string
        clamp_rate:
          type: number
          format: double
        delta1_cross_margin_params:
          $ref: >-
            #/components/schemas/ChannelsMarketsMarketSymbolSubscribeChangedFieldsDelta1CrossMarginParams
        expiry_at:
          type: integer
        fee_config:
          $ref: >-
            #/components/schemas/ChannelsMarketsMarketSymbolSubscribeChangedFieldsFeeConfig
        funding_multiplier:
          type: number
          format: double
        funding_period_hours:
          type: number
          format: double
        interest_rate:
          type: number
          format: double
        market_kind:
          type: string
        max_funding_rate:
          type: number
          format: double
        max_funding_rate_change:
          type: number
          format: double
        max_open_orders:
          type: integer
        max_order_size:
          type: string
        max_slippage:
          type: number
          format: double
        max_tob_spread:
          type: number
          format: double
        min_notional:
          type: string
        open_at:
          type: integer
        option_cross_margin_params:
          $ref: >-
            #/components/schemas/ChannelsMarketsMarketSymbolSubscribeChangedFieldsOptionCrossMarginParams
        option_type:
          type: string
        oracle_ewma_factor:
          type: number
          format: double
        order_size_increment:
          type: string
        position_limit:
          type: string
        price_bands_width:
          type: number
          format: double
        price_tick_size:
          type: string
        quote_currency:
          type: string
        settlement_currency:
          type: string
        settlement_price:
          type: number
          format: double
        strike_price:
          type: string
        underlying_twap_price:
          type: number
          format: double
      title: ChannelsMarketsMarketSymbolSubscribeChangedFields
    ChannelsMarketsMarketSymbolSubscribeEventType:
      type: string
      enum:
        - CREATED
        - DELETED
        - SETTLED
        - UPDATED
      description: Type of change
      title: ChannelsMarketsMarketSymbolSubscribeEventType
    MarketsMarketSymbolSubscribe:
      type: object
      properties:
        changed_fields:
          $ref: >-
            #/components/schemas/ChannelsMarketsMarketSymbolSubscribeChangedFields
        event_type:
          $ref: '#/components/schemas/ChannelsMarketsMarketSymbolSubscribeEventType'
          description: Type of change
        symbol:
          type: string
          description: Market symbol
        updated_at:
          type: integer
          description: Time of change (unix time millisecond)
      required:
        - changed_fields
        - event_type
        - symbol
        - updated_at
      title: MarketsMarketSymbolSubscribe

```