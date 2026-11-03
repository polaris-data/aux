> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# order_book.{market_symbol}.{feed_type}@15@{refresh_rate}[@{price_tick}]

GET /order_book.{market_symbol}.{feed_type}@15@{refresh_rate}[@{price_tick}]

Reference: https://docs.paradex.trade/ws/web-socket-channels/order-book-market-symbol-feed-type-15-refresh-rate-price-tick/order-book-market-symbol-feed-type-15-refresh-rate-price-tick

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: order_book.{market_symbol}.{feed_type}@15@{refresh_rate}[@{price_tick}]
  version: >-
    subpackage_orderBookMarketSymbolFeedType15RefreshRatePriceTick.orderBookMarketSymbolFeedType15RefreshRatePriceTick
channels:
  /order_book.{market_symbol}.{feed_type}@15@{refresh_rate}[@{price_tick}]:
    parameters:
      feed_type:
        description: Type of feed to subscribe to
        schema:
          $ref: '#/components/schemas/feed_type'
      market_symbol:
        description: Symbol of the market (e.g., BTC-PERP)
        schema:
          type: string
      price_tick:
        description: Optional price tick for grouping order book prices
        schema:
          $ref: '#/components/schemas/price_tick'
      refresh_rate:
        description: Rate of sending updates (used in BBO channels)
        schema:
          $ref: '#/components/schemas/refresh_rate'
    publish:
      operationId: order-book-market-symbol-feed-type-15-refresh-rate-price-tick-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: >-
            #/components/schemas/OrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribe
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    feed_type:
      type: string
      enum:
        - snapshot
        - deltas
        - interactive
        - interactive_deltas
      title: feed_type
    price_tick:
      type: string
      enum:
        - 0_0000001
        - 0_0000005
        - 0_000001
        - 0_00001
        - 0_0001
        - 0_001
        - 0_01
        - 0_1
        - '1'
        - '10'
        - '100'
      title: price_tick
    refresh_rate:
      type: string
      enum:
        - 50ms
        - 100ms
      title: refresh_rate
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskApiSide:
      type: string
      enum:
        - BUY
        - SELL
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskApiSide
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskApi:
      type: object
      properties:
        price:
          type: string
        side:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskApiSide
        size:
          type: string
      required:
        - price
        - side
        - size
      description: Size on the best ask from API (excluding RPI)
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskApi
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskInteractiveSide:
      type: string
      enum:
        - BUY
        - SELL
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskInteractiveSide
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskInteractive:
      type: object
      properties:
        price:
          type: string
        side:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskInteractiveSide
        size:
          type: string
      required:
        - price
        - side
        - size
      description: Size on the best ask from UI
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskInteractive
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidApiSide:
      type: string
      enum:
        - BUY
        - SELL
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidApiSide
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidApi:
      type: object
      properties:
        price:
          type: string
        side:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidApiSide
        size:
          type: string
      required:
        - price
        - side
        - size
      description: Size on the best bid from API (excluding RPI)
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidApi
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidInteractiveSide:
      type: string
      enum:
        - BUY
        - SELL
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidInteractiveSide
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidInteractive:
      type: object
      properties:
        price:
          type: string
        side:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidInteractiveSide
        size:
          type: string
      required:
        - price
        - side
        - size
      description: Size on the best bid from UI
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidInteractive
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeDeletesItemsSide:
      type: string
      enum:
        - BUY
        - SELL
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeDeletesItemsSide
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeDeletesItems:
      type: object
      properties:
        price:
          type: string
        side:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeDeletesItemsSide
        size:
          type: string
      required:
        - price
        - side
        - size
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeDeletesItems
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeInsertsItemsSide:
      type: string
      enum:
        - BUY
        - SELL
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeInsertsItemsSide
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeInsertsItems:
      type: object
      properties:
        price:
          type: string
        side:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeInsertsItemsSide
        size:
          type: string
      required:
        - price
        - side
        - size
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeInsertsItems
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeUpdateType:
      type: string
      enum:
        - s
        - d
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeUpdateType
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeUpdatesItemsSide:
      type: string
      enum:
        - BUY
        - SELL
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeUpdatesItemsSide
    ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeUpdatesItems:
      type: object
      properties:
        price:
          type: string
        side:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeUpdatesItemsSide
        size:
          type: string
      required:
        - price
        - side
        - size
      title: >-
        ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeUpdatesItems
    OrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribe:
      type: object
      properties:
        best_ask_api:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskApi
          description: Size on the best ask from API (excluding RPI)
        best_ask_interactive:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskInteractive
          description: Size on the best ask from UI
        best_bid_api:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidApi
          description: Size on the best bid from API (excluding RPI)
        best_bid_interactive:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidInteractive
          description: Size on the best bid from UI
        deletes:
          type: array
          items:
            $ref: >-
              #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeDeletesItems
        inserts:
          type: array
          items:
            $ref: >-
              #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeInsertsItems
        last_updated_at:
          type: integer
          description: in milliseconds
        market:
          type: string
        relative_spread:
          type: string
          description: >-
            Relative spread: (ask-bid)/midpoint. Only included for the periodic
            interactive feed.
        seq_no:
          type: integer
        update_type:
          $ref: >-
            #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeUpdateType
        updates:
          type: array
          items:
            $ref: >-
              #/components/schemas/ChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeUpdatesItems
      required:
        - deletes
        - inserts
        - last_updated_at
        - market
        - seq_no
        - update_type
        - updates
      title: OrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribe

```