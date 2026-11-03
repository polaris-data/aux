> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# order.cancel_on_disconnect

GET /order.cancel_on_disconnect

Reference: https://docs.paradex.trade/ws/web-socket-channels/order-cancel-on-disconnect/order-cancel-on-disconnect

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: order.cancel_on_disconnect
  version: subpackage_orderCancelOnDisconnect.orderCancelOnDisconnect
channels:
  /order.cancel_on_disconnect:
    publish:
      operationId: order-cancel-on-disconnect-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/OrderCancelOnDisconnectSubscribe'
    subscribe:
      operationId: order-cancel-on-disconnect-subscribe
      summary: publish
      description: >-
        When enabled, all orders placed (or already tracked) during this
        connection are automatically cancelled if the WebSocket disconnects
        unexpectedly. CoD state is scoped to the connection and does not persist
        across reconnects.
      message:
        name: publish
        title: publish
        description: >-
          When enabled, all orders placed (or already tracked) during this
          connection are automatically cancelled if the WebSocket disconnects
          unexpectedly. CoD state is scoped to the connection and does not
          persist across reconnects.
        payload:
          $ref: '#/components/schemas/OrderCancelOnDisconnectPublish'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    OrderCancelOnDisconnectSubscribe:
      type: object
      properties:
        enabled:
          type: boolean
          description: The new Cancel on Disconnect state for this connection
      title: OrderCancelOnDisconnectSubscribe
    OrderCancelOnDisconnectPublish:
      type: object
      properties:
        enabled:
          type: boolean
          description: true to enable Cancel on Disconnect, false to disable
      required:
        - enabled
      title: OrderCancelOnDisconnectPublish

```