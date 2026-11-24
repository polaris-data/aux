"""Translate Python calling conventions to the native request models."""
from collections.abc import Mapping
from time import time_ns


def request_params(request, params):
    if request is not None and params is not None:
        raise TypeError("supply the request once")
    if request is not None:
        return request
    return {} if params is None else params


def merge(request, fields):
    result = dict(request or {})
    if result.keys() & fields.keys():
        raise TypeError("duplicate native parameters")
    result.update(fields)
    return result


def fields(request, params, **values):
    return merge(request_params(request, params), {k: v for k, v in values.items() if v is not None})


def identifier(key, value, request, params, **values):
    if isinstance(value, Mapping):
        if request is not None:
            raise TypeError("supply the request once")
        request, value = value, None
    return fields(request, params, **{key: value}, **values)


def order_body(key, market, request, params, **values):
    if request is None or market is None:
        return fields(request_params(market, request), params, **values)
    return fields(None, params, **{key: market, "body": request}, **values)


def hyperliquid_action(request, params, nonce, expires_after):
    result = dict(request_params(request, params))
    if nonce is not None or expires_after is not None:
        result["options"] = fields(result.get("options"), None, nonce=nonce, expires_after=expires_after)
    return result


def lighter_account(request, params, account_index, l1_address):
    if account_index is None and l1_address is None:
        if request is None and params is None:
            raise ValueError("provide exactly one account_index or l1_address")
        return request_params(request, params)
    if account_index is not None and l1_address is not None:
        raise ValueError("provide exactly one account_index or l1_address")
    return fields(request, params, by="index" if l1_address is None else "l1_address",
                  value=account_index if l1_address is None else l1_address)


def lighter_order(request, order, options, expired_at, attributes):
    opts = fields(options, None, expired_at=expired_at, attributes=attributes)
    if "expired_at" not in opts:
        opts["expired_at"] = time_ns() // 1_000_000 + 600_000
    if opts.get("attributes") is not None:
        attrs = opts["attributes"]
        normalized = {str(k): v for k, v in attrs.items()}
        if len(normalized) != len(attrs):
            raise TypeError("duplicate Lighter attribute")
        opts["attributes"] = normalized
    return {"order": request_params(request, order), "options": opts}


def alias(first, second):
    if first is not None and second is not None:
        raise TypeError("supply the option once")
    return first if second is None else second


def metric_chain(request, chain):
    if chain is None:
        return request
    if "chainId" in request and request["chainId"] != chain:
        raise ValueError("request chain differs from client chain")
    return {**request, "chainId": chain}


def header_credentials(credentials):
    key, token, expires = (credentials.get(k) for k in ("api_key", "access_token", "expires_at"))
    for value in (key, token):
        if value is not None and (not isinstance(value, str) or not value or any(ord(c) < 32 or ord(c) > 126 for c in value)):
            raise ValueError("invalid credential")
    if expires is not None and (type(expires) is not int or not 0 <= expires < 2**64):
        raise ValueError("invalid credential expiry")
    return key, {k:v for k,v in (("access_token",token), ("expires_at",expires)) if v is not None}
