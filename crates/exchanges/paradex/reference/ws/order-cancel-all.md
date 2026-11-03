> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# order.cancel_all

GET /order.cancel_all

Reference: https://docs.paradex.trade/ws/web-socket-channels/order-cancel-all/order-cancel-all

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: order.cancel_all
  version: subpackage_orderCancelAll.orderCancelAll
channels:
  /order.cancel_all:
    publish:
      operationId: order-cancel-all-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/OrderCancelAllSubscribe'
    subscribe:
      operationId: order-cancel-all-subscribe
      summary: publish
      description: >-
        Cancels all open orders for the authenticated account. If market is
        provided only orders in that market are cancelled.
      message:
        name: publish
        title: publish
        description: >-
          Cancels all open orders for the authenticated account. If market is
          provided only orders in that market are cancelled.
        payload:
          $ref: '#/components/schemas/OrderCancelAllPublish'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    OrderCancelAllSubscribe:
      type: object
      properties:
        status:
          type: string
          description: Always "ok"
      title: OrderCancelAllSubscribe
    OrderCancelAllPublish:
      type: object
      properties:
        market:
          type: string
          description: >-
            If set, only cancel orders in this market; omit to cancel all open
            orders
      title: OrderCancelAllPublish

```