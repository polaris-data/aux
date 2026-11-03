> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# auth

GET /auth

Reference: https://docs.paradex.trade/ws/web-socket-channels/auth/auth

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: auth
  version: subpackage_auth.auth
channels:
  /auth:
    publish:
      operationId: auth-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/AuthSubscribe'
    subscribe:
      operationId: auth-subscribe
      summary: publish
      description: >-
        Validates the JWT bearer token and associates the account with the
        connection. Must be called before subscribing to private channels or
        using order management methods.
      message:
        name: publish
        title: publish
        description: >-
          Validates the JWT bearer token and associates the account with the
          connection. Must be called before subscribing to private channels or
          using order management methods.
        payload:
          $ref: '#/components/schemas/AuthPublish'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    AuthSubscribe:
      type: object
      properties:
        node_id:
          type: string
          description: Opaque server node identifier
      title: AuthSubscribe
    AuthPublish:
      type: object
      properties:
        bearer:
          type: string
          description: JWT string obtained from POST /auth
      required:
        - bearer
      title: AuthPublish

```