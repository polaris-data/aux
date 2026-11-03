> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# modify_subscription

GET /modify_subscription

Reference: https://docs.paradex.trade/ws/web-socket-channels/modify-subscription/modify-subscription

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: modify_subscription
  version: subpackage_modifySubscription.modifySubscription
channels:
  /modify_subscription:
    publish:
      operationId: modify-subscription-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/ModifySubscriptionSubscribe'
    subscribe:
      operationId: modify-subscription-subscribe
      summary: publish
      description: >-
        Adds and/or removes markets on a live scoped markets_summary
        subscription without unsubscribing and resubscribing, so markets that
        remain in the filter stream uninterrupted. Requires a subscription
        opened in scoped mode (subscribe with a markets array).
      message:
        name: publish
        title: publish
        description: >-
          Adds and/or removes markets on a live scoped markets_summary
          subscription without unsubscribing and resubscribing, so markets that
          remain in the filter stream uninterrupted. Requires a subscription
          opened in scoped mode (subscribe with a markets array).
        payload:
          $ref: '#/components/schemas/ModifySubscriptionPublish'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    ModifySubscriptionSubscribe:
      type: object
      properties:
        add:
          type: array
          items:
            type: string
          description: Markets that were added
        channel:
          type: string
          description: Channel whose subscription filter was modified
        remove:
          type: array
          items:
            type: string
          description: Markets that were removed
      title: ModifySubscriptionSubscribe
    ModifySubscriptionPublish:
      type: object
      properties:
        add:
          type: array
          items:
            type: string
          description: Markets to add to the subscription's filter in place
        channel:
          type: string
          description: >-
            Channel of the live scoped subscription to modify (e.g.
            markets_summary)
        remove:
          type: array
          items:
            type: string
          description: Markets to remove from the subscription's filter in place
      required:
        - channel
      title: ModifySubscriptionPublish

```