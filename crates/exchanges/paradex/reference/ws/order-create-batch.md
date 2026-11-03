> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# order.create_batch

GET /order.create_batch

Reference: https://docs.paradex.trade/ws/web-socket-channels/order-create-batch/order-create-batch

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: order.create_batch
  version: subpackage_orderCreateBatch.orderCreateBatch
channels:
  /order.create_batch:
    publish:
      operationId: order-create-batch-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/OrderCreateBatchSubscribe'
    subscribe:
      operationId: order-create-batch-subscribe
      summary: publish
      description: >-
        Creates up to the configured maximum number of orders atomically. All
        signatures are validated before any orders reach the matching engine.
        Rate limit is consumed once for the entire batch.
      message:
        name: publish
        title: publish
        description: >-
          Creates up to the configured maximum number of orders atomically. All
          signatures are validated before any orders reach the matching engine.
          Rate limit is consumed once for the entire batch.
        payload:
          $ref: '#/components/schemas/OrderCreateBatchPublish'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    ChannelsOrderCreateBatchSubscribeResultsItemsOrderInstruction:
      type: string
      enum:
        - GTC
        - IOC
        - POST_ONLY
        - RPI
      description: Time-in-force instruction
      title: ChannelsOrderCreateBatchSubscribeResultsItemsOrderInstruction
    ChannelsOrderCreateBatchSubscribeResultsItemsOrderSide:
      type: string
      enum:
        - BUY
        - SELL
      description: Order side
      title: ChannelsOrderCreateBatchSubscribeResultsItemsOrderSide
    ChannelsOrderCreateBatchSubscribeResultsItemsOrderStatus:
      type: string
      enum:
        - NEW
        - OPEN
        - CLOSED
      description: Order status
      title: ChannelsOrderCreateBatchSubscribeResultsItemsOrderStatus
    ChannelsOrderCreateBatchSubscribeResultsItemsOrderType:
      type: string
      enum:
        - LIMIT
        - MARKET
        - STOP_LIMIT
        - STOP_MARKET
      description: Order type
      title: ChannelsOrderCreateBatchSubscribeResultsItemsOrderType
    ChannelsOrderCreateBatchSubscribeResultsItemsOrder:
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
          $ref: >-
            #/components/schemas/ChannelsOrderCreateBatchSubscribeResultsItemsOrderInstruction
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
          $ref: >-
            #/components/schemas/ChannelsOrderCreateBatchSubscribeResultsItemsOrderSide
          description: Order side
        size:
          type: string
          description: Order size
        status:
          $ref: >-
            #/components/schemas/ChannelsOrderCreateBatchSubscribeResultsItemsOrderStatus
          description: Order status
        timestamp:
          type: integer
          description: Signature timestamp supplied at creation
        trigger_price:
          type: string
          description: Trigger price for stop orders
        type:
          $ref: >-
            #/components/schemas/ChannelsOrderCreateBatchSubscribeResultsItemsOrderType
          description: Order type
      description: Created order (present on success)
      title: ChannelsOrderCreateBatchSubscribeResultsItemsOrder
    ChannelsOrderCreateBatchSubscribeResultsItems:
      type: object
      properties:
        created_at:
          type: integer
          description: Matching-engine timestamp in milliseconds (present on success)
        error:
          type: string
          description: Error message (present on failure)
        order:
          $ref: >-
            #/components/schemas/ChannelsOrderCreateBatchSubscribeResultsItemsOrder
          description: Created order (present on success)
        received_at:
          type: integer
          description: Server receipt timestamp in nanoseconds (present on success)
      title: ChannelsOrderCreateBatchSubscribeResultsItems
    OrderCreateBatchSubscribe:
      type: object
      properties:
        results:
          type: array
          items:
            $ref: '#/components/schemas/ChannelsOrderCreateBatchSubscribeResultsItems'
          description: Per-order outcomes in the same order as the request array
      title: OrderCreateBatchSubscribe
    ChannelsOrderCreateBatchPublishOrdersItemsFlagsItems:
      type: string
      enum:
        - REDUCE_ONLY
        - MMP
      title: ChannelsOrderCreateBatchPublishOrdersItemsFlagsItems
    ChannelsOrderCreateBatchPublishOrdersItemsInstruction:
      type: string
      enum:
        - GTC
        - IOC
        - POST_ONLY
        - RPI
      description: Time-in-force instruction
      title: ChannelsOrderCreateBatchPublishOrdersItemsInstruction
    ChannelsOrderCreateBatchPublishOrdersItemsSide:
      type: string
      enum:
        - BUY
        - SELL
      description: Order side
      title: ChannelsOrderCreateBatchPublishOrdersItemsSide
    ChannelsOrderCreateBatchPublishOrdersItemsStp:
      type: string
      enum:
        - EXPIRE_MAKER
        - EXPIRE_TAKER
        - EXPIRE_BOTH
      description: Self-trade prevention
      title: ChannelsOrderCreateBatchPublishOrdersItemsStp
    ChannelsOrderCreateBatchPublishOrdersItemsType:
      type: string
      enum:
        - LIMIT
        - MARKET
        - STOP_LIMIT
        - STOP_MARKET
      description: Order type
      title: ChannelsOrderCreateBatchPublishOrdersItemsType
    ChannelsOrderCreateBatchPublishOrdersItems:
      type: object
      properties:
        client_id:
          type: string
          description: Client-assigned order ID (max 64 chars)
        flags:
          type: array
          items:
            $ref: >-
              #/components/schemas/ChannelsOrderCreateBatchPublishOrdersItemsFlagsItems
          description: >-
            Order flags. MMP is for LIMIT orders on dated options and perpetual
            futures; only resting maker fills count
        instruction:
          $ref: >-
            #/components/schemas/ChannelsOrderCreateBatchPublishOrdersItemsInstruction
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
          $ref: '#/components/schemas/ChannelsOrderCreateBatchPublishOrdersItemsSide'
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
          $ref: '#/components/schemas/ChannelsOrderCreateBatchPublishOrdersItemsStp'
          description: Self-trade prevention
        trigger_price:
          type: string
          description: Trigger price for stop orders
        type:
          $ref: '#/components/schemas/ChannelsOrderCreateBatchPublishOrdersItemsType'
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
      title: ChannelsOrderCreateBatchPublishOrdersItems
    OrderCreateBatchPublish:
      type: object
      properties:
        orders:
          type: array
          items:
            $ref: '#/components/schemas/ChannelsOrderCreateBatchPublishOrdersItems'
          description: Array of order create requests (max batch size enforced server-side)
      required:
        - orders
      title: OrderCreateBatchPublish

```