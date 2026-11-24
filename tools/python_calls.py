"""Named Python arguments that differ from the corresponding wire models."""


def unary(venue, method):
    if venue == "databento" or (venue == "tardis" and method == "symbols"):
        return "request=None, *, params=None, **parameters", "_args.merge(_request_params(request, params), parameters)"
    if venue == "hyperliquid" and method in ("cancel", "order", "cancel_by_cloid"):
        return ("request=None, *, params=None, nonce=None, expires_after=None",
                "_args.hyperliquid_action(request, params, nonce, expires_after)")
    if venue == "lighter" and method == "account":
        return ("request=None, *, params=None, account_index=None, l1_address=None",
                "_args.lighter_account(request, params, account_index, l1_address)")
    if venue == "tardis" and method == "instruments":
        return ("exchange=None, request=None, *, params=None, **parameters",
                "_args.merge(_args.identifier('exchange', exchange, request, params), parameters)")
    if (venue, method) in {("arcus", "place_order"), ("monaco", "place_order"), ("monaco", "batch_create"),
                           ("monaco", "batch_replace"), ("monaco", "replace_order")}:
        key = "markets" if method.startswith("batch_") else "market"
        order_id = method == "replace_order"
        return (f"{key}=None, " + ("order_id=None, " if order_id else "") + "request=None, *, params=None",
                f"_args.order_body('{key}', {key}, request, params" + (", orderId=order_id)" if order_id else ")"))
    # Each row maps native fields to reference argument names. Defaults only apply
    # to named calls; a caller supplying the entire wire model keeps its own values.
    rows = {
        "arcus": {
            "order_book": ("market", {"nLevels": ("n_levels", 20)}),
            "markets": (None, {"market": ("market", None)}),
            "trades": ("market", {"limit": ("limit", 100)}),
            **{op: ("address", {"accountIndex": ("account_index", 0)}) for op in ("positions", "account", "open_orders")},
        },
        "hyperliquid": {
            "meta": (None, {"dex": ("dex", None)}),
            "l2_book": ("coin", {"nSigFigs": ("n_sig_figs", None), "mantissa": ("mantissa", None)}),
            **{op: ("user", {"dex": ("dex", None)}) for op in ("clearinghouse_state", "open_orders")},
        },
        "lighter": {
            **{op: (None, {"market_id": ("market_id", None)}) for op in ("active_orders", "order_books")},
            **{op: ("market_id", {"limit": ("limit", 50)}) for op in ("order_book_orders", "recent_trades")},
        },
        "metric": {"bid_ask": ("pool", {"depth": ("depth", None)})},
        "qfex": {"refdata": (None, {"ticker": ("ticker", None)}), "contracts": (None, {"symbol": ("symbol", None)}),
                 "order_book": ("symbol", {})},
        "tardis": {"exchange_summary": ("exchange", {}), "exchange_details": ("exchange", {})},
        "monaco": {"cancel_all": ("trading_pair_id", {})},
    }
    if (venue, method) in (("tardis", "instrument"), ("lighter", "next_nonce")):
        a, b = ("exchange", "symbol") if venue == "tardis" else ("account_index", "api_key_index")
        return f"{a}=None, {b}=None, *, request=None, params=None", f"_args.identifier('{a}', {a}, request, params, {b}={b})"
    row = rows.get(venue, {}).get(method)
    if row is None:
        return None
    first, options = row
    sig = f"{first}=None, *, request=None, params=None" if first else "request=None, *, params=None"
    sig += "".join(f", {arg}=None" for arg, _ in options.values())
    args = []
    for field, (arg, default) in options.items():
        value = arg
        if default is not None:
            value = f"{default} if {arg} is None and isinstance({first}, (str, int)) else {arg}"
        args.append(f"{field}={value}")
    wire_key = "tradingPairId" if first == "trading_pair_id" else first
    call = f"_args.identifier('{wire_key}', {first}, request, params" if first else "_args.fields(request, params"
    return sig, ", ".join([call, *args]) + ")"
