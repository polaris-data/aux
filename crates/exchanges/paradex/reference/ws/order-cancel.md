> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# order.cancel

GET /order.cancel

Reference: https://docs.paradex.trade/ws/web-socket-channels/order-cancel/order-cancel

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: order.cancel
  version: subpackage_orderCancel.orderCancel
channels:
  /order.cancel:
    publish:
      operationId: order-cancel-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/OrderCancelSubscribe'
    subscribe:
      operationId: order-cancel-subscribe
      summary: publish
      description: >-
        Cancels an order by exchange order ID or by client_id + market. Either
        id or (client_id + market) must be provided.
      message:
        name: publish
        title: publish
        description: >-
          Cancels an order by exchange order ID or by client_id + market. Either
          id or (client_id + market) must be provided.
        payload:
          $ref: '#/components/schemas/OrderCancelPublish'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    ChannelsOrderCancelSubscribeStatus:
      type: string
      enum:
        - QUEUED_FOR_CANCELLATION
      description: Cancellation status
      title: ChannelsOrderCancelSubscribeStatus
    OrderCancelSubscribe:
      type: object
      properties:
        order_id:
          type: string
          description: Cancelled order ID
        status:
          $ref: '#/components/schemas/ChannelsOrderCancelSubscribeStatus'
          description: Cancellation status
      title: OrderCancelSubscribe
    OrderCancelPublish:
      type: object
      properties:
        client_id:
          type: string
          description: Client-assigned order ID; requires market when used
        id:
          type: string
          description: Order ID to cancel; either id or client_id+market is required
        market:
          type: string
          description: Market symbol; required when cancelling by client_id
      title: OrderCancelPublish

```