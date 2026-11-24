"""Convert Python Dune method arguments to a native request."""


def request_args(field, identifier, request, params, *, post):
    if params is not None:
        if identifier is not None or request is not None:
            raise TypeError("supply the request once")
        return params
    if field is None:
        if request is None or "body" in request:
            return request or {}
        return {"body": request}
    if isinstance(identifier, dict):
        if request is not None:
            raise TypeError("supply the request once")
        return identifier
    if identifier is None:
        return request or {}
    payload = request or {}
    if post:
        return {field: identifier, "body": payload}
    if field in payload:
        raise TypeError("supply the identifier once")
    return {field: identifier, **payload}
