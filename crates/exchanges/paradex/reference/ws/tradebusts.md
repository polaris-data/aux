> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# tradebusts

GET /tradebusts

Reference: https://docs.paradex.trade/ws/web-socket-channels/tradebusts/tradebusts

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: tradebusts
  version: subpackage_tradebusts.tradebusts
channels:
  /tradebusts:
    publish:
      operationId: tradebusts-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/TradebustsSubscribe'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    TradebustsSubscribe:
      type: object
      properties:
        account:
          type: string
          description: Paradex Account
        busted_fill_id:
          type: string
          description: Unique ID of the busted fill
        created_at:
          type: integer
          description: Unix timestamp in milliseconds when bust was created
      required:
        - account
        - busted_fill_id
        - created_at
      title: TradebustsSubscribe

```