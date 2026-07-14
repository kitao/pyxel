from collections.abc import Iterator
from contextlib import contextmanager

import pytest


@contextmanager
def raises_exact(exception_type: type[BaseException], message: str) -> Iterator[None]:
    with pytest.raises(exception_type) as exc_info:
        yield

    assert type(exc_info.value) is exception_type
    assert str(exc_info.value) == message
