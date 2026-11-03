> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# balance_events

GET /balance_events

Reference: https://docs.paradex.trade/ws/web-socket-channels/balance-events/balance-events

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: balance_events
  version: subpackage_balanceEvents.balanceEvents
channels:
  /balance_events:
    publish:
      operationId: balance-events-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/BalanceEventsSubscribe'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    BalanceEventsSubscribe:
      type: object
      properties:
        created_at:
          type: integer
          description: Unix Millisecond timestamp at which the event occurred
        fees:
          type: string
          description: Fees in USDC (for fill events)
        fill_id:
          type: string
          description: Unique string ID of the fill (for fill events)
        funding_index:
          type: string
          description: Funding index used for the fill (for fill events)
        market:
          type: string
          description: Name of the market the update is referring to (for fill events)
        realized_funding:
          type: string
          description: Realized Funding in USDC (for fill events)
        realized_pnl:
          type: string
          description: Realized PnL (excluding funding) in USDC (for fill events)
        settlement_asset_address:
          type: string
          description: L2 Address of the settlement asset
        settlement_asset_balance_after:
          type: string
          description: Balance of the settled asset after the update
        settlement_asset_balance_before:
          type: string
          description: Balance of the settled asset before the update
        settlement_asset_name:
          type: string
          description: Name of the settlement asset
        settlement_asset_price:
          type: string
          description: Price of the settled asset at the update
        status:
          type: string
          description: Status of the balance event on chain
        type:
          type: string
          description: Type of the event
      required:
        - created_at
        - fees
        - fill_id
        - funding_index
        - market
        - realized_funding
        - realized_pnl
        - settlement_asset_address
        - settlement_asset_balance_after
        - settlement_asset_balance_before
        - settlement_asset_name
        - settlement_asset_price
        - status
        - type
      title: BalanceEventsSubscribe

```