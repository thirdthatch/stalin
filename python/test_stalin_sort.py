import itertools
import csv
from pathlib import Path
import unittest

from stalin_sort import stalin_sort


class StalinSortTests(unittest.TestCase):
    def test_javascript_reference_fixtures(self):
        path = Path(__file__).resolve().parents[1] / "tests/fixtures/javascript.tsv"
        with path.open() as source:
            for row in csv.DictReader(source, delimiter="\t"):
                data = [int(value) for value in row["input"].split(",")] if row["input"] != "[]" else []
                expected = [int(value) for value in row["kept"].split(",")] if row["kept"] != "[]" else []
                self.assertEqual(stalin_sort(data), expected, row["name"])

    def test_examples(self):
        for source, expected in [
            ([], []), ([1], [1]), ([3, 1, 4, 2, 5, 0, 2, 6], [3, 4, 5, 6]),
            ([3, 2, 1], [3]), ([1, 2, 3], [1, 2, 3]),
            ([2, 2, 1, 2], [2, 2, 2]), ([-2.5, -3.0, 0.5, 0.5], [-2.5, 0.5, 0.5]),
        ]:
            before = source.copy()
            self.assertEqual(stalin_sort(source), expected)
            self.assertEqual(source, before)

    def test_exhaustive_prefix_maxima(self):
        cases = itertools.chain(itertools.permutations(range(7)), itertools.product(range(3), repeat=6))
        for case in cases:
            # Independent characterization: retain exactly the prefix maxima,
            # including ties. Rejected values never reset the threshold.
            expected = [value for i, value in enumerate(case) if value == max(case[:i + 1])]
            self.assertEqual(stalin_sort(case), expected)

    def test_iterator_key_and_equal_identity(self):
        rows = [(2, "a"), (1, "b"), (2, "c"), (3, "d")]
        calls = []

        def score(row):
            calls.append(row)
            return row[0]

        output = stalin_sort(iter(rows), key=score)
        self.assertEqual(output, [rows[0], rows[2], rows[3]])
        self.assertIs(output[0], rows[0])
        self.assertEqual(calls, rows)

    def test_nan_rejected(self):
        with self.assertRaises(ValueError):
            stalin_sort([1.0, float("nan")])


if __name__ == "__main__":
    unittest.main()
