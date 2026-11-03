> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# account

GET /account

Reference: https://docs.paradex.trade/ws/web-socket-channels/account/account

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: account
  version: subpackage_account.account
channels:
  /account:
    publish:
      operationId: account-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/AccountSubscribe'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    ChannelsAccountSubscribeWithdrawalLockStatus:
      type: string
      enum:
        - PENDING
        - ACTIVE
      description: PENDING until Paraclear holds the lock; withdraw only once ACTIVE
      title: ChannelsAccountSubscribeWithdrawalLockStatus
    ChannelsAccountSubscribeWithdrawalLock:
      type: object
      properties:
        amount:
          type: string
          description: Settlement asset amount that can be withdrawn or transferred
        expires_at:
          type: integer
          description: Unix seconds after which the lock can no longer be spent
        request_id:
          type: string
          description: >-
            The request this lock answers, as returned by POST
            /account/withdrawal-lock
        status:
          $ref: '#/components/schemas/ChannelsAccountSubscribeWithdrawalLockStatus'
          description: PENDING until Paraclear holds the lock; withdraw only once ACTIVE
      required:
        - amount
        - expires_at
        - status
      description: >-
        Settlement asset amount reserved for a portfolio margin withdrawal,
        absent when there is none
      title: ChannelsAccountSubscribeWithdrawalLock
    ChannelsAccountSubscribeWithdrawalLockRejection:
      type: object
      properties:
        amount:
          type: string
          description: Settlement asset amount requested
        reason:
          type: string
          description: Why the lock was refused
        rejected_at:
          type: integer
          description: Unix seconds
        request_id:
          type: string
          description: The request refused, as returned by POST /account/withdrawal-lock
      required:
        - amount
        - reason
        - rejected_at
      description: >-
        The last withdrawal lock request refused, absent once it is five minutes
        old
      title: ChannelsAccountSubscribeWithdrawalLockRejection
    AccountSubscribe:
      type: object
      properties:
        account:
          type: string
          description: User's starknet account
        account_value:
          type: string
          description: Current account value [with unrealized P&Ls]
        free_collateral:
          type: string
          description: >-
            Free collateral available (Account value in excess of Initial Margin
            required)
        initial_margin_requirement:
          type: string
          description: Amount required to open trade for the existing positions
        last_seen_notification:
          type: integer
          description: >-
            Latest notification event timestamp (ms) the user has seen — used
            for cross-device dedup
        maintenance_margin_requirement:
          type: string
          description: Amount required to maintain exisiting positions
        margin_cushion:
          type: string
          description: Acc value in excess of maintenance margin required
        seq_no:
          type: integer
          description: >-
            Unique increasing number (non-sequential) that is assigned to this
            account update. Can be used to deduplicate multiple feeds
        settlement_asset:
          type: string
          description: Settlement asset for the account
        status:
          type: string
          description: Status of the acc - like ACTIVE, LIQUIDATION
        total_collateral:
          type: string
          description: User's total collateral
        updated_at:
          type: integer
          description: Account last updated time
        withdrawal_lock:
          $ref: '#/components/schemas/ChannelsAccountSubscribeWithdrawalLock'
          description: >-
            Settlement asset amount reserved for a portfolio margin withdrawal,
            absent when there is none
        withdrawal_lock_rejection:
          $ref: '#/components/schemas/ChannelsAccountSubscribeWithdrawalLockRejection'
          description: >-
            The last withdrawal lock request refused, absent once it is five
            minutes old
      required:
        - account
        - account_value
        - free_collateral
        - initial_margin_requirement
        - last_seen_notification
        - maintenance_margin_requirement
        - margin_cushion
        - seq_no
        - settlement_asset
        - status
        - total_collateral
        - updated_at
      title: AccountSubscribe

```