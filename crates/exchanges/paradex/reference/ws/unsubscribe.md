> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# unsubscribe

GET /unsubscribe

Reference: https://docs.paradex.trade/ws/web-socket-channels/unsubscribe/unsubscribe

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: unsubscribe
  version: subpackage_unsubscribe.unsubscribe
channels:
  /unsubscribe:
    publish:
      operationId: unsubscribe-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/UnsubscribeSubscribe'
    subscribe:
      operationId: unsubscribe-subscribe
      summary: publish
      message:
        name: publish
        title: publish
        payload:
          $ref: '#/components/schemas/UnsubscribePublish'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    UnsubscribeSubscribe:
      type: object
      properties:
        channel:
          type: string
          description: Channel name that was unsubscribed
      title: UnsubscribeSubscribe
    UnsubscribePublish:
      type: object
      properties:
        channel:
          type: string
          description: Channel name to unsubscribe from
      required:
        - channel
      title: UnsubscribePublish

```