> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# order.create

GET /order.create

Reference: https://docs.paradex.trade/ws/web-socket-channels/order-create/order-create

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: order.create
  version: subpackage_orderCreate.orderCreate
channels:
  /order.create:
    publish:
      operationId: order-create-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/OrderCreateSubscribe'
    subscribe:
      operationId: order-create-subscribe
      summary: publish
      description: >-
        Creates a single order. Requires authentication and a valid Starknet
        signature over the order payload. Rate limits are shared with the REST
        API (same Redis counters).
      message:
        name: publish
        title: publish
        description: >-
          Creates a single order. Requires authentication and a valid Starknet
          signature over the order payload. Rate limits are shared with the REST
          API (same Redis counters).
        payload:
          $ref: '#/components/schemas/OrderCreatePublish'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    ChannelsOrderCreateSubscribeOrderInstruction:
      type: string
      enum:
        - GTC
        - IOC
        - POST_ONLY
        - RPI
      description: Time-in-force instruction
      title: ChannelsOrderCreateSubscribeOrderInstruction
    ChannelsOrderCreateSubscribeOrderSide:
      type: string
      enum:
        - BUY
        - SELL
      description: Order side
      title: ChannelsOrderCreateSubscribeOrderSide
    ChannelsOrderCreateSubscribeOrderStatus:
      type: string
      enum:
        - NEW
        - OPEN
        - CLOSED
      description: Order status
      title: ChannelsOrderCreateSubscribeOrderStatus
    ChannelsOrderCreateSubscribeOrderType:
      type: string
      enum:
        - LIMIT
        - MARKET
        - STOP_LIMIT
        - STOP_MARKET
      description: Order type
      title: ChannelsOrderCreateSubscribeOrderType
    ChannelsOrderCreateSubscribeOrder:
      type: object
      properties:
        account:
          type: string
          description: Account address that owns the order
        avg_fill_price:
          type: string
          description: Volume-weighted average fill price; "0" if unfilled
        cancel_reason:
          type: string
          description: Reason for cancellation (populated when status=CLOSED)
        client_id:
          type: string
          description: Client-assigned order ID
        created_at:
          type: integer
          description: Order creation time in milliseconds
        flags:
          type: array
          items:
            type: string
          description: Order flags applied
        id:
          type: string
          description: Unique order identifier assigned by exchange
        instruction:
          $ref: '#/components/schemas/ChannelsOrderCreateSubscribeOrderInstruction'
          description: Time-in-force instruction
        last_updated_at:
          type: integer
          description: Order last update time in milliseconds
        market:
          type: string
          description: Market symbol
        price:
          type: string
          description: Limit price
        published_at:
          type: integer
          description: Exchange publish time in milliseconds
        received_at:
          type: integer
          description: Exchange receipt time in milliseconds
        remaining_size:
          type: string
          description: Remaining unfilled size
        seq_no:
          type: integer
          description: Monotonically increasing sequence number for this order's updates
        side:
          $ref: '#/components/schemas/ChannelsOrderCreateSubscribeOrderSide'
          description: Order side
        size:
          type: string
          description: Order size
        status:
          $ref: '#/components/schemas/ChannelsOrderCreateSubscribeOrderStatus'
          description: Order status
        timestamp:
          type: integer
          description: Signature timestamp supplied at creation
        trigger_price:
          type: string
          description: Trigger price for stop orders
        type:
          $ref: '#/components/schemas/ChannelsOrderCreateSubscribeOrderType'
          description: Order type
      description: Created order
      title: ChannelsOrderCreateSubscribeOrder
    OrderCreateSubscribe:
      type: object
      properties:
        created_at:
          type: integer
          description: Matching-engine timestamp in milliseconds
        order:
          $ref: '#/components/schemas/ChannelsOrderCreateSubscribeOrder'
          description: Created order
        received_at:
          type: integer
          description: >-
            Server receipt timestamp in nanoseconds, recorded before any
            speedbump
      title: OrderCreateSubscribe
    ChannelsOrderCreatePublishFlagsItems:
      type: string
      enum:
        - REDUCE_ONLY
        - MMP
      title: ChannelsOrderCreatePublishFlagsItems
    ChannelsOrderCreatePublishInstruction:
      type: string
      enum:
        - GTC
        - IOC
        - POST_ONLY
        - RPI
      description: Time-in-force instruction
      title: ChannelsOrderCreatePublishInstruction
    ChannelsOrderCreatePublishSide:
      type: string
      enum:
        - BUY
        - SELL
      description: Order side
      title: ChannelsOrderCreatePublishSide
    ChannelsOrderCreatePublishStp:
      type: string
      enum:
        - EXPIRE_MAKER
        - EXPIRE_TAKER
        - EXPIRE_BOTH
      description: Self-trade prevention
      title: ChannelsOrderCreatePublishStp
    ChannelsOrderCreatePublishType:
      type: string
      enum:
        - LIMIT
        - MARKET
        - STOP_LIMIT
        - STOP_MARKET
      description: Order type
      title: ChannelsOrderCreatePublishType
    OrderCreatePublish:
      type: object
      properties:
        client_id:
          type: string
          description: Client-assigned order ID (max 64 chars)
        flags:
          type: array
          items:
            $ref: '#/components/schemas/ChannelsOrderCreatePublishFlagsItems'
          description: >-
            Order flags. MMP is for LIMIT orders on dated options and perpetual
            futures; only resting maker fills count
        instruction:
          $ref: '#/components/schemas/ChannelsOrderCreatePublishInstruction'
          description: Time-in-force instruction
        market:
          type: string
          description: Market symbol
        on_behalf_of_account:
          type: string
          description: Isolated margin account address for isolated margin orders
        price:
          type: string
          description: Limit price; use "0" for MARKET orders
        recv_window:
          type: integer
          description: >-
            Max milliseconds between signature_timestamp and server receipt (min
            10)
        side:
          $ref: '#/components/schemas/ChannelsOrderCreatePublishSide'
          description: Order side
        signature:
          type: string
          description: Starknet signature "[r,s]" over the order payload
        signature_timestamp:
          type: integer
          description: Unix timestamp in milliseconds used for signature verification
        size:
          type: string
          description: Order size (decimal string)
        stp:
          $ref: '#/components/schemas/ChannelsOrderCreatePublishStp'
          description: Self-trade prevention
        trigger_price:
          type: string
          description: Trigger price for stop orders
        type:
          $ref: '#/components/schemas/ChannelsOrderCreatePublishType'
          description: Order type
      required:
        - instruction
        - market
        - price
        - side
        - signature
        - signature_timestamp
        - size
        - type
      title: OrderCreatePublish

```