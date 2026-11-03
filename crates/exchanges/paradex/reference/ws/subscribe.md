> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# subscribe

GET /subscribe

Reference: https://docs.paradex.trade/ws/web-socket-channels/subscribe/subscribe

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: subscribe
  version: subpackage_subscribe.subscribe
channels:
  /subscribe:
    publish:
      operationId: subscribe-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/SubscribeSubscribe'
    subscribe:
      operationId: subscribe-subscribe
      summary: publish
      description: >-
        Starts receiving real-time messages for the given channel. Private
        channels (orders, fills, positions, account, etc.) require prior
        authentication. See the WebSocket Channels reference for all available
        channel names.
      message:
        name: publish
        title: publish
        description: >-
          Starts receiving real-time messages for the given channel. Private
          channels (orders, fills, positions, account, etc.) require prior
          authentication. See the WebSocket Channels reference for all available
          channel names.
        payload:
          $ref: '#/components/schemas/SubscribePublish'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    SubscribeSubscribe:
      type: object
      properties:
        channel:
          type: string
          description: Channel name that was subscribed
      title: SubscribeSubscribe
    SubscribePublish:
      type: object
      properties:
        channel:
          type: string
          description: Channel name to subscribe to (e.g. trades.BTC-USD-PERP)
        markets:
          type: array
          items:
            type: string
          description: >-
            For markets_summary only: the initial set of markets to scope the
            subscription to (may be empty). Present selects scoped
            mutable-filter mode whose set is changed via modify_subscription;
            omit for the all-markets firehose.
      required:
        - channel
      title: SubscribePublish

```