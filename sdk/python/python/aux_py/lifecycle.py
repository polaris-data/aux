"""Ownership of outstanding calls and streams for named clients."""
import asyncio
from contextvars import ContextVar
from functools import wraps
from inspect import iscoroutinefunction
from weakref import WeakSet

from ._native import ConnectorError


_owner = ContextVar("aux_client", default=None)


def ensure_open():
    owner = _owner.get()
    if owner is not None:
        owner._require_open()


class ClosedError(ConnectorError):
    pass


def _managed(method):
    if iscoroutinefunction(method):
        @wraps(method)
        async def run(self, *args, **kwargs):
            self._require_open()
            token = _owner.set(self)
            try:
                task = asyncio.create_task(method(self, *args, **kwargs))
            finally:
                _owner.reset(token)
            self._calls.add(task)
            try:
                result = await task
            except asyncio.CancelledError:
                if self.closed:
                    raise ClosedError("client is closed") from None
                raise
            finally:
                self._calls.discard(task)
            if callable(getattr(result, "close", None)):
                if self.closed:
                    await result.close()
                else:
                    self._streams.add(result)
            self._require_open()
            return result
        return run

    @wraps(method)
    def run(self, *args, **kwargs):
        self._require_open()
        return method(self, *args, **kwargs)
    return run


class ClientLifecycle:
    def __init__(self):
        self._closed = False
        self._calls = set()
        self._streams = WeakSet()
        self._closing = None

    def __init_subclass__(cls):
        super().__init_subclass__()
        for name, method in vars(cls).copy().items():
            if not name.startswith("_") and callable(method) and not isinstance(method, staticmethod):
                setattr(cls, name, _managed(method))

    @property
    def closed(self):
        return self._closed

    def _require_open(self):
        if self.closed:
            raise ClosedError("client is closed")

    async def aclose(self):
        if not self.closed:
            self._closed = True
            calls = tuple(task for task in self._calls if task is not asyncio.current_task())
            streams = tuple(self._streams)
            self._streams.clear()
            for task in calls:
                task.cancel()
            self._closing = asyncio.gather(*(stream.close() for stream in streams), return_exceptions=False)
        await asyncio.shield(self._closing)

    async def __aenter__(self):
        self._require_open()
        return self

    async def __aexit__(self, *_):
        await self.aclose()
