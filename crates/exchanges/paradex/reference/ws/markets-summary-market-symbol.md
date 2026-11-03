> For clean Markdown of any page, append .md to the page URL.
> For a complete documentation index, see https://docs.paradex.trade/llms.txt.
> For AI client integration (Claude Code, Cursor, etc.), connect to the MCP server at https://docs.paradex.trade/_mcp/server.

# markets_summary[.{market_symbol}]

GET /markets_summary[.{market_symbol}]

Reference: https://docs.paradex.trade/ws/web-socket-channels/markets-summary-market-symbol/markets-summary-market-symbol

## AsyncAPI Specification

```yaml
asyncapi: 2.6.0
info:
  title: markets_summary[.{market_symbol}]
  version: subpackage_marketsSummaryMarketSymbol.marketsSummaryMarketSymbol
channels:
  /markets_summary[.{market_symbol}]:
    parameters:
      market_symbol:
        description: Symbol of the market (e.g., BTC-PERP)
        schema:
          type: string
    publish:
      operationId: markets-summary-market-symbol-publish
      summary: subscribe
      message:
        name: subscribe
        title: subscribe
        payload:
          $ref: '#/components/schemas/MarketsSummaryMarketSymbolSubscribe'
servers:
  production:
    url: wss://ws.api.prod.paradex.trade/v1?sbeSchemaId=1&sbeSchemaVersion=1
    protocol: wss
    x-default: true
components:
  schemas:
    ChannelsMarketsSummaryMarketSymbolSubscribeGreeks:
      type: object
      properties:
        delta:
          type: string
          description: Market Delta, to 5 decimal places
        gamma:
          type: string
          description: Market Gamma, to 6 significant digits
        rho:
          type: string
          description: Market Rho, to 6 significant digits
        theta:
          type: string
          description: Market Theta, to 6 significant digits
        vanna:
          type: string
          description: Market Vanna, to 6 significant digits
        vega:
          type: string
          description: Market Vega, to 6 significant digits
        volga:
          type: string
          description: Market Volga, to 6 significant digits
      required:
        - delta
        - gamma
        - vega
      description: Greeks (delta, gamma, vega). Partial for perpetual futures.
      title: ChannelsMarketsSummaryMarketSymbolSubscribeGreeks
    ChannelsMarketsSummaryMarketSymbolSubscribeRollingDetails:
      type: object
      properties:
        current_active_contract_symbol:
          type: string
          description: Current active contract symbol
        current_active_contract_weight:
          type: string
          description: Front contract weight ("0" to "1")
        next_active_contract_symbol:
          type: string
          description: Next active contract symbol
        period_end:
          type: integer
          description: UTC millis, end of current rolling period
        period_start:
          type: integer
          description: UTC millis, start of current rolling period
      title: ChannelsMarketsSummaryMarketSymbolSubscribeRollingDetails
    MarketsSummaryMarketSymbolSubscribe:
      type: object
      properties:
        ask:
          type: string
          description: Best ask price
        ask_iv:
          type: string
          description: Ask implied volatility, for options, to 5 decimal places
        ask_size:
          type: string
          description: Size at best ask price
        bid:
          type: string
          description: Best bid price
        bid_iv:
          type: string
          description: Bid implied volatility, for options, to 5 decimal places
        bid_size:
          type: string
          description: Size at best bid price
        created_at:
          type: integer
          description: Market summary creation time
        delta:
          type: string
          description: 'Deprecated: Use greeks.delta instead'
        external_fair_price:
          type: string
          description: >-
            External fair price, calculated from spot price and an external
            basis: spot_price * (1 + external_basis). where external basis is
            the median basis rate on different external exchanges
        forward_rate:
          type: string
          description: >-
            EWMA-smoothed annualized forward rate for dated options, to 6
            decimal places
        funding_rate:
          type: string
          description: >-
            This raw funding rate corresponds to the actual funding period of
            the instrument itself. It is not a normalized 8h funding rate. To 12
            decimal places.
        future_funding_rate:
          type: string
          description: For options it's a smoothed version of future's funding rate
        greeks:
          $ref: >-
            #/components/schemas/ChannelsMarketsSummaryMarketSymbolSubscribeGreeks
          description: Greeks (delta, gamma, vega). Partial for perpetual futures.
        last_iv:
          type: string
          description: >-
            Last traded price implied volatility, for options, to 5 decimal
            places
        last_traded_price:
          type: string
          description: Last traded price
        mark_iv:
          type: string
          description: Mark implied volatility, for options, to 5 decimal places
        mark_price:
          type: string
          description: >-
            [Mark
            price](https://docs.paradex.trade/risk-system/mark-price-calculation)
        open_interest:
          type: string
          description: Open interest in base currency
        price_change_rate_24h:
          type: string
          description: Price change rate in the last 24 hours, to 4 decimal places
        risk_free_rate:
          type: string
          description: Risk-free discounting rate used for option pricing
        rolling_details:
          $ref: >-
            #/components/schemas/ChannelsMarketsSummaryMarketSymbolSubscribeRollingDetails
        settlement_twap_estimate:
          type: string
          description: >-
            Running average of the underlying over the settlement window, on the
            live summary only while inside that window
        symbol:
          type: string
          description: Market symbol
        total_volume:
          type: string
          description: Lifetime total traded volume in USD
        underlying_price:
          type: string
          description: Underlying asset price (spot price)
        volume_24h:
          type: string
          description: 24 hour volume in USD
      required:
        - ask
        - ask_size
        - bid
        - bid_size
        - created_at
        - funding_rate
        - last_traded_price
        - mark_price
        - open_interest
        - price_change_rate_24h
        - symbol
        - total_volume
        - underlying_price
        - volume_24h
      title: MarketsSummaryMarketSymbolSubscribe

```