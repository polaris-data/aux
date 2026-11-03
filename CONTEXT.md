# Connectors

An SDK for accessing exchange-native market data, account information and trading services, alongside data vendors’ native data services.

## Language

**Exchange client**:
An application's connection to one exchange's native services, including public data and authenticated trading. Each exchange has its own client and native operations.
_Avoid_: Provider (for the application-facing exchange connection)

**Data vendor**:
A service that distributes market data and related reference information for one or more trading venues.

**Data client**:
An application's connection to a data vendor's native services, including discovery, historical data, replay, downloads and real-time feeds where available.
