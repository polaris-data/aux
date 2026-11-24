"""Birdeye REST wire models generated from src/data/birdeye/schema/birdeye.json."""
from __future__ import annotations
from decimal import Decimal
from typing import Any, TypedDict

_PriceRequestOptional = TypedDict("_PriceRequestOptional", {'check_liquidity': 'int | Decimal | None', 'include_liquidity': 'bool | None', 'ui_amount_mode': 'str | None'}, total=False)
class PriceRequest(_PriceRequestOptional):
    address: str

_MultiPriceRequestOptional = TypedDict("_MultiPriceRequestOptional", {'check_liquidity': 'int | Decimal | None', 'include_liquidity': 'bool | None', 'ui_amount_mode': 'str | None'}, total=False)
class MultiPriceRequest(_MultiPriceRequestOptional):
    list_address: str

class MultiPricePostBody(TypedDict):
    list_address: str

_MultiPricePostRequestOptional = TypedDict("_MultiPricePostRequestOptional", {'check_liquidity': 'int | Decimal | None', 'include_liquidity': 'bool | None', 'ui_amount_mode': 'str | None'}, total=False)
class MultiPricePostRequest(_MultiPricePostRequestOptional):
    body: MultiPricePostBody

_TokenTradesRequestOptional = TypedDict("_TokenTradesRequestOptional", {'offset': 'int | None', 'limit': 'int | None', 'tx_type': 'str | None', 'sort_type': 'str | None', 'ui_amount_mode': 'str | None'}, total=False)
class TokenTradesRequest(_TokenTradesRequestOptional):
    address: str

_PairTradesRequestOptional = TypedDict("_PairTradesRequestOptional", {'offset': 'int | None', 'limit': 'int | None', 'tx_type': 'str | None', 'sort_type': 'str | None', 'ui_amount_mode': 'str | None'}, total=False)
class PairTradesRequest(_PairTradesRequestOptional):
    address: str

_TokenTradesByTimeRequestOptional = TypedDict("_TokenTradesByTimeRequestOptional", {'offset': 'int | None', 'limit': 'int | None', 'tx_type': 'str | None', 'before_time': 'int | None', 'after_time': 'int | None', 'ui_amount_mode': 'str | None'}, total=False)
class TokenTradesByTimeRequest(_TokenTradesByTimeRequestOptional):
    address: str

_PairTradesByTimeRequestOptional = TypedDict("_PairTradesByTimeRequestOptional", {'offset': 'int | None', 'limit': 'int | None', 'tx_type': 'str | None', 'before_time': 'int | None', 'after_time': 'int | None', 'ui_amount_mode': 'str | None'}, total=False)
class PairTradesByTimeRequest(_PairTradesByTimeRequestOptional):
    address: str

_TradesV3RequestOptional = TypedDict("_TradesV3RequestOptional", {'offset': 'int | None', 'limit': 'int | None', 'sort_by': 'str | None', 'sort_type': 'str | None', 'tx_type': 'str | None', 'source': 'str | None', 'owner': 'str | None', 'pool_id': 'str | None', 'before_time': 'int | None', 'after_time': 'int | None', 'before_block_number': 'int | None', 'after_block_number': 'int | None', 'ui_amount_mode': 'str | None'}, total=False)
class TradesV3Request(_TradesV3RequestOptional):
    pass

_TokenTradesV3RequestOptional = TypedDict("_TokenTradesV3RequestOptional", {'offset': 'int | None', 'limit': 'int | None', 'sort_by': 'str | None', 'sort_type': 'str | None', 'tx_type': 'str | None', 'source': 'str | None', 'owner': 'str | None', 'pool_id': 'str | None', 'before_time': 'int | None', 'after_time': 'int | None', 'before_block_number': 'int | None', 'after_block_number': 'int | None', 'ui_amount_mode': 'str | None'}, total=False)
class TokenTradesV3Request(_TokenTradesV3RequestOptional):
    address: str

_RecentTradesRequestOptional = TypedDict("_RecentTradesRequestOptional", {'offset': 'int | None', 'limit': 'int | None', 'tx_type': 'str | None', 'owner': 'str | None', 'before_time': 'int | None', 'after_time': 'int | None', 'before_block_number': 'int | None', 'after_block_number': 'int | None', 'ui_amount_mode': 'str | None'}, total=False)
class RecentTradesRequest(_RecentTradesRequestOptional):
    pass

_OhlcvRequestOptional = TypedDict("_OhlcvRequestOptional", {'currency': 'str | None', 'chart_type': 'str | None', 'ui_amount_mode': 'str | None'}, total=False)
class OhlcvRequest(_OhlcvRequestOptional):
    address: str
    type: str
    time_from: int
    time_to: int

class OhlcvPairRequest(TypedDict):
    address: str
    type: str
    time_from: int
    time_to: int

_OhlcvBaseQuoteRequestOptional = TypedDict("_OhlcvBaseQuoteRequestOptional", {'ui_amount_mode': 'str | None'}, total=False)
class OhlcvBaseQuoteRequest(_OhlcvBaseQuoteRequestOptional):
    base_address: str
    quote_address: str
    type: str
    time_from: int
    time_to: int

_OhlcvV3RequestOptional = TypedDict("_OhlcvV3RequestOptional", {'currency': 'str | None', 'chart_type': 'str | None', 'ui_amount_mode': 'str | None', 'mode': 'str | None', 'count_limit': 'int | None', 'padding': 'bool | None', 'outlier': 'bool | None'}, total=False)
class OhlcvV3Request(_OhlcvV3RequestOptional):
    address: str
    type: str
    time_from: int
    time_to: int

_OhlcvPairV3RequestOptional = TypedDict("_OhlcvPairV3RequestOptional", {'mode': 'str | None', 'count_limit': 'int | None', 'padding': 'bool | None', 'outlier': 'bool | None', 'inversion': 'bool | None'}, total=False)
class OhlcvPairV3Request(_OhlcvPairV3RequestOptional):
    address: str
    type: str
    time_from: int
    time_to: int

_LiquidityOhlcPairRequestOptional = TypedDict("_LiquidityOhlcPairRequestOptional", {'time': 'int | None', 'direction': 'str | None', 'count': 'int | None'}, total=False)
class LiquidityOhlcPairRequest(_LiquidityOhlcPairRequestOptional):
    address: str

_LiquidityOhlcTokenRequestOptional = TypedDict("_LiquidityOhlcTokenRequestOptional", {'resolution': 'str | None', 'time': 'int | None', 'direction': 'str | None', 'count': 'int | None'}, total=False)
class LiquidityOhlcTokenRequest(_LiquidityOhlcTokenRequestOptional):
    address: str

_LiquidityHistoryTokenRequestOptional = TypedDict("_LiquidityHistoryTokenRequestOptional", {'resolution': 'str | None', 'time': 'int | None', 'direction': 'str | None', 'count': 'int | None'}, total=False)
class LiquidityHistoryTokenRequest(_LiquidityHistoryTokenRequestOptional):
    address: str

_LiquidityHistoryPairRequestOptional = TypedDict("_LiquidityHistoryPairRequestOptional", {'unix_time': 'int | None', 'block_number': 'int | None', 'step': 'int | None', 'skip_empty': 'bool | None', 'direction': 'str | None', 'count': 'int | None'}, total=False)
class LiquidityHistoryPairRequest(_LiquidityHistoryPairRequestOptional):
    address: str

class LiquidityLatestTokenBody(TypedDict):
    addresses: list[str]

class LiquidityLatestTokenRequest(TypedDict):
    body: LiquidityLatestTokenBody

_HistoryPriceRequestOptional = TypedDict("_HistoryPriceRequestOptional", {'ui_amount_mode': 'str | None'}, total=False)
class HistoryPriceRequest(_HistoryPriceRequestOptional):
    address: str
    address_type: str
    type: str
    time_from: int
    time_to: int

_HistoricalPriceUnixRequestOptional = TypedDict("_HistoricalPriceUnixRequestOptional", {'unixtime': 'int | None', 'ui_amount_mode': 'str | None'}, total=False)
class HistoricalPriceUnixRequest(_HistoricalPriceUnixRequestOptional):
    address: str

_PriceVolumeSingleRequestOptional = TypedDict("_PriceVolumeSingleRequestOptional", {'type': 'str | None', 'ui_amount_mode': 'str | None'}, total=False)
class PriceVolumeSingleRequest(_PriceVolumeSingleRequestOptional):
    address: str

_PriceVolumeMultiBodyOptional = TypedDict("_PriceVolumeMultiBodyOptional", {'type': 'str | None'}, total=False)
class PriceVolumeMultiBody(_PriceVolumeMultiBodyOptional):
    list_address: str

_PriceVolumeMultiRequestOptional = TypedDict("_PriceVolumeMultiRequestOptional", {'ui_amount_mode': 'str | None'}, total=False)
class PriceVolumeMultiRequest(_PriceVolumeMultiRequestOptional):
    body: PriceVolumeMultiBody

_PairOverviewRequestOptional = TypedDict("_PairOverviewRequestOptional", {'ui_amount_mode': 'str | None'}, total=False)
class PairOverviewRequest(_PairOverviewRequestOptional):
    address: str

_PairOverviewMultipleRequestOptional = TypedDict("_PairOverviewMultipleRequestOptional", {'ui_amount_mode': 'str | None'}, total=False)
class PairOverviewMultipleRequest(_PairOverviewMultipleRequestOptional):
    list_address: str

_PriceStatsRequestOptional = TypedDict("_PriceStatsRequestOptional", {'list_timeframe': 'str | None', 'ui_amount_mode': 'str | None'}, total=False)
class PriceStatsRequest(_PriceStatsRequestOptional):
    address: str

class PriceStatsMultipleBody(TypedDict):
    list_address: str

_PriceStatsMultipleRequestOptional = TypedDict("_PriceStatsMultipleRequestOptional", {'list_timeframe': 'str | None', 'ui_amount_mode': 'str | None'}, total=False)
class PriceStatsMultipleRequest(_PriceStatsMultipleRequestOptional):
    body: PriceStatsMultipleBody

_TokenTradesByVolumeRequestOptional = TypedDict("_TokenTradesByVolumeRequestOptional", {'min_volume': 'int | Decimal | None', 'max_volume': 'int | Decimal | None', 'sort_by': 'str | None', 'tx_type': 'str | None', 'source': 'str | None', 'owner': 'str | None', 'before_time': 'int | None', 'after_time': 'int | None', 'ui_amount_mode': 'str | None', 'sort_type': 'str | None', 'offset': 'int | None', 'limit': 'int | None'}, total=False)
class TokenTradesByVolumeRequest(_TokenTradesByVolumeRequestOptional):
    token_address: str
    volume_type: str

_TokenListRequestOptional = TypedDict("_TokenListRequestOptional", {'sort_by': 'str | None', 'sort_type': 'str | None', 'offset': 'int | None', 'limit': 'int | None', 'min_liquidity': 'int | Decimal | None', 'max_liquidity': 'int | Decimal | None', 'ui_amount_mode': 'str | None'}, total=False)
class TokenListRequest(_TokenListRequestOptional):
    pass

_TokenListV3RequestOptional = TypedDict("_TokenListV3RequestOptional", {'sort_by': 'str | None', 'sort_type': 'str | None', 'platform_name': 'str | None', 'min_liquidity': 'int | Decimal | None', 'max_liquidity': 'int | Decimal | None', 'min_market_cap': 'int | Decimal | None', 'max_market_cap': 'int | Decimal | None', 'min_fdv': 'int | Decimal | None', 'max_fdv': 'int | Decimal | None', 'min_recent_listing_time': 'int | None', 'max_recent_listing_time': 'int | None', 'min_last_trade_unix_time': 'int | None', 'max_last_trade_unix_time': 'int | None', 'min_holder': 'int | None', 'min_volume_1m_usd': 'int | Decimal | None', 'min_volume_5m_usd': 'int | Decimal | None', 'min_volume_30m_usd': 'int | Decimal | None', 'min_volume_1h_usd': 'int | Decimal | None', 'min_volume_2h_usd': 'int | Decimal | None', 'min_volume_4h_usd': 'int | Decimal | None', 'min_volume_8h_usd': 'int | Decimal | None', 'min_volume_24h_usd': 'int | Decimal | None', 'min_volume_7d_usd': 'int | Decimal | None', 'min_volume_30d_usd': 'int | Decimal | None', 'min_volume_1m_change_percent': 'int | Decimal | None', 'min_volume_5m_change_percent': 'int | Decimal | None', 'min_volume_30m_change_percent': 'int | Decimal | None', 'min_volume_1h_change_percent': 'int | Decimal | None', 'min_volume_2h_change_percent': 'int | Decimal | None', 'min_volume_4h_change_percent': 'int | Decimal | None', 'min_volume_8h_change_percent': 'int | Decimal | None', 'min_volume_24h_change_percent': 'int | Decimal | None', 'min_volume_7d_change_percent': 'int | Decimal | None', 'min_volume_30d_change_percent': 'int | Decimal | None', 'min_price_change_1m_percent': 'int | Decimal | None', 'min_price_change_5m_percent': 'int | Decimal | None', 'min_price_change_30m_percent': 'int | Decimal | None', 'min_price_change_1h_percent': 'int | Decimal | None', 'min_price_change_2h_percent': 'int | Decimal | None', 'min_price_change_4h_percent': 'int | Decimal | None', 'min_price_change_8h_percent': 'int | Decimal | None', 'min_price_change_24h_percent': 'int | Decimal | None', 'min_price_change_7d_percent': 'int | Decimal | None', 'min_price_change_30d_percent': 'int | Decimal | None', 'min_trade_1m_count': 'int | None', 'min_trade_5m_count': 'int | None', 'min_trade_30m_count': 'int | None', 'min_trade_1h_count': 'int | None', 'min_trade_2h_count': 'int | None', 'min_trade_4h_count': 'int | None', 'min_trade_8h_count': 'int | None', 'min_trade_24h_count': 'int | None', 'min_trade_7d_count': 'int | None', 'min_trade_30d_count': 'int | None', 'min_global_fees_paid': 'int | Decimal | None', 'offset': 'int | None', 'limit': 'int | None', 'ui_amount_mode': 'str | None'}, total=False)
class TokenListV3Request(_TokenListV3RequestOptional):
    pass

_TokenListScrollRequestOptional = TypedDict("_TokenListScrollRequestOptional", {'limit': 'int | None', 'scroll_id': 'str | None', 'sort_by': 'str | None', 'sort_type': 'str | None', 'platform_name': 'str | None', 'min_liquidity': 'int | Decimal | None', 'max_liquidity': 'int | Decimal | None', 'min_market_cap': 'int | Decimal | None', 'max_market_cap': 'int | Decimal | None', 'min_fdv': 'int | Decimal | None', 'max_fdv': 'int | Decimal | None', 'min_recent_listing_time': 'int | None', 'max_recent_listing_time': 'int | None', 'min_last_trade_unix_time': 'int | None', 'max_last_trade_unix_time': 'int | None', 'min_holder': 'int | None', 'min_volume_1m_usd': 'int | Decimal | None', 'min_volume_5m_usd': 'int | Decimal | None', 'min_volume_30m_usd': 'int | Decimal | None', 'min_volume_1h_usd': 'int | Decimal | None', 'min_volume_2h_usd': 'int | Decimal | None', 'min_volume_4h_usd': 'int | Decimal | None', 'min_volume_8h_usd': 'int | Decimal | None', 'min_volume_24h_usd': 'int | Decimal | None', 'min_volume_7d_usd': 'int | Decimal | None', 'min_volume_30d_usd': 'int | Decimal | None', 'min_global_fees_paid': 'int | Decimal | None', 'min_volume_1m_change_percent': 'int | Decimal | None', 'min_volume_5m_change_percent': 'int | Decimal | None', 'min_volume_30m_change_percent': 'int | Decimal | None', 'min_volume_1h_change_percent': 'int | Decimal | None', 'min_volume_2h_change_percent': 'int | Decimal | None', 'min_volume_4h_change_percent': 'int | Decimal | None', 'min_volume_8h_change_percent': 'int | Decimal | None', 'min_volume_24h_change_percent': 'int | Decimal | None', 'min_volume_7d_change_percent': 'int | Decimal | None', 'min_volume_30d_change_percent': 'int | Decimal | None', 'min_price_change_1m_percent': 'int | Decimal | None', 'min_price_change_5m_percent': 'int | Decimal | None', 'min_price_change_30m_percent': 'int | Decimal | None', 'min_price_change_1h_percent': 'int | Decimal | None', 'min_price_change_2h_percent': 'int | Decimal | None', 'min_price_change_4h_percent': 'int | Decimal | None', 'min_price_change_8h_percent': 'int | Decimal | None', 'min_price_change_24h_percent': 'int | Decimal | None', 'min_price_change_7d_percent': 'int | Decimal | None', 'min_price_change_30d_percent': 'int | Decimal | None', 'min_trade_1m_count': 'int | None', 'min_trade_5m_count': 'int | None', 'min_trade_30m_count': 'int | None', 'min_trade_1h_count': 'int | None', 'min_trade_2h_count': 'int | None', 'min_trade_4h_count': 'int | None', 'min_trade_8h_count': 'int | None', 'min_trade_24h_count': 'int | None', 'min_trade_7d_count': 'int | None', 'min_trade_30d_count': 'int | None', 'ui_amount_mode': 'str | None'}, total=False)
class TokenListScrollRequest(_TokenListScrollRequestOptional):
    pass

_TokenOverviewRequestOptional = TypedDict("_TokenOverviewRequestOptional", {'frames': 'str | None', 'includes': 'str | None', 'ui_amount_mode': 'str | None'}, total=False)
class TokenOverviewRequest(_TokenOverviewRequestOptional):
    address: str

class TokenMetadataRequest(TypedDict):
    address: str

class TokenMetadataMultipleRequest(TypedDict):
    list_address: str

_TokenMarketDataRequestOptional = TypedDict("_TokenMarketDataRequestOptional", {'ui_amount_mode': 'str | None'}, total=False)
class TokenMarketDataRequest(_TokenMarketDataRequestOptional):
    address: str

_TokenMarketDataMultipleRequestOptional = TypedDict("_TokenMarketDataMultipleRequestOptional", {'ui_amount_mode': 'str | None'}, total=False)
class TokenMarketDataMultipleRequest(_TokenMarketDataMultipleRequestOptional):
    list_address: str

_TokenTradeDataRequestOptional = TypedDict("_TokenTradeDataRequestOptional", {'frames': 'str | None', 'ui_amount_mode': 'str | None', 'includes': 'str | None'}, total=False)
class TokenTradeDataRequest(_TokenTradeDataRequestOptional):
    address: str

_TokenTradeDataMultipleRequestOptional = TypedDict("_TokenTradeDataMultipleRequestOptional", {'frames': 'str | None', 'ui_amount_mode': 'str | None', 'includes': 'str | None'}, total=False)
class TokenTradeDataMultipleRequest(_TokenTradeDataMultipleRequestOptional):
    list_address: str

_TokenNewListingRequestOptional = TypedDict("_TokenNewListingRequestOptional", {'time_to': 'int | None', 'limit': 'int | None', 'meme_platform_enabled': 'bool | None'}, total=False)
class TokenNewListingRequest(_TokenNewListingRequestOptional):
    pass

_TokenMarketsRequestOptional = TypedDict("_TokenMarketsRequestOptional", {'sort_type': 'str | None', 'sort_by': 'str | None', 'offset': 'int | None', 'limit': 'int | None'}, total=False)
class TokenMarketsRequest(_TokenMarketsRequestOptional):
    address: str

_TokenSecurityRequestOptional = TypedDict("_TokenSecurityRequestOptional", {'include': 'list[str] | None'}, total=False)
class TokenSecurityRequest(_TokenSecurityRequestOptional):
    address: str

_TokenMintBurnRequestOptional = TypedDict("_TokenMintBurnRequestOptional", {'sort_by': 'str | None', 'sort_type': 'str | None', 'type': 'str | None', 'after_time': 'int | None', 'before_time': 'int | None', 'offset': 'int | None', 'limit': 'int | None'}, total=False)
class TokenMintBurnRequest(_TokenMintBurnRequestOptional):
    address: str

class TokenExitLiquidityRequest(TypedDict):
    address: str

class TokenExitLiquidityMultipleRequest(TypedDict):
    list_address: str

_TraderTradesByTimeRequestOptional = TypedDict("_TraderTradesByTimeRequestOptional", {'offset': 'int | None', 'limit': 'int | None', 'tx_type': 'str | None', 'before_time': 'int | None', 'after_time': 'int | None', 'ui_amount_mode': 'str | None'}, total=False)
class TraderTradesByTimeRequest(_TraderTradesByTimeRequestOptional):
    address: str

class ApiResponse(TypedDict):
    success: bool
    data: Any

