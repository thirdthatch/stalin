"""Classic Stalin sort: retain a nondecreasing subsequence, discard the rest."""

from collections.abc import Callable, Iterable
from math import isnan
from typing import Any, TypeVar

T = TypeVar("T")


def stalin_sort(values: Iterable[T], *, key: Callable[[T], Any] | None = None) -> list[T]:
    """Return retained items without mutating the input; equal keys are retained.

    Keys must have a consistent total ordering. Float NaN keys are rejected.
    This intentionally removes items and is NOT a permutation-preserving sort.
    Time: O(n). Extra space including the result: O(n).
    """
    result: list[T] = []
    last_key: Any = None
    for value in values:
        score = value if key is None else key(value)
        if isinstance(score, float) and isnan(score):
            raise ValueError("NaN has no consistent ordering")
        if not result or not score < last_key:
            result.append(value)
            last_key = score
    return result
