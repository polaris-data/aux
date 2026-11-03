> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# transaction

GET /transaction

Reference: https://docs.paradex.trade/ws/web-socket-channels/transaction/transaction

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: transaction
  version: subpackage_transaction.transaction
channels:
  /transaction:
    publish:
      operationId: transaction-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/TransactionSubscribe'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    ChannelsTransactionSubscribeState:
      type: string
      enum:
        - ACCEPTED_ON_L1
        - ACCEPTED_ON_L2
        - NOT_RECEIVED
        - RECEIVED
        - REJECTED
        - REVERTED
      description: Status of the transaction on Starknet
      title: ChannelsTransactionSubscribeState
    ChannelsTransactionSubscribeType:
      type: string
      enum:
        - TRANSACTION_FILL
        - TRANSACTION_LIQUIDATE
        - TRANSACTION_SETTLE_MARKET
      description: Event that triggered the transaction
      title: ChannelsTransactionSubscribeType
    TransactionSubscribe:
      type: object
      properties:
        completed_at:
          type: integer
          description: Timestamp from when the transaction was completed
        created_at:
          type: integer
          description: Timestamp from when the transaction was sent to blockchain gateway
        hash:
          type: string
          description: >-
            Tx Hash of the settled
            trade                                                // Hash of the
            transaction
        id:
          type: string
          description: >-
            Unique string ID of the event that triggered the transaction. For
            example, fill ID or liquidation ID
        state:
          $ref: '#/components/schemas/ChannelsTransactionSubscribeState'
          description: Status of the transaction on Starknet
        type:
          $ref: '#/components/schemas/ChannelsTransactionSubscribeType'
          description: Event that triggered the transaction
      required:
        - completed_at
        - created_at
        - hash
        - id
        - state
        - type
      title: TransactionSubscribe

```