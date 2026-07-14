"""
Pytest-compatible test script for quantrs Python bindings.
"""

import pytest
import quantrs


class TestQuantrsPythonBindings:
    """Test class for quantrs Python bindings."""

    def test_day_count(self):
        """Test day count functionality."""
        conventions = [
            "ACT/365F",
            "ACT/360",
            "30/360US",
            "30/360E",
            "ACT/ACT ISDA",
        ]

        for conv in conventions:
            dc = quantrs.DayCount(conv)

            year_frac = dc.year_fraction("2025-01-01", "2025-07-01")
            days = dc.day_count("2025-01-01", "2025-07-01")

            assert isinstance(year_frac, float)
            assert isinstance(days, int)
            assert year_frac > 0
            assert days > 0

    def test_zero_coupon_bond_pricing(self):
        """Test zero coupon bond pricing."""
        bond = quantrs.ZeroCouponBond(1000.0, "2030-12-31")
        dc = quantrs.DayCount("ACT/365F")

        assert bond.face_value == 1000.0
        assert bond.maturity == "2030-12-31"

        settlement = "2025-06-19"

        yields = [0.01, 0.02, 0.03, 0.04, 0.05]

        previous_price = None

        for ytm in yields:
            result = bond.price(settlement, ytm, dc)

            assert isinstance(result.clean, float)
            assert isinstance(result.dirty, float)
            assert isinstance(result.accrued, float)

            assert result.clean > 0
            assert result.dirty > 0
            assert result.accrued == 0.0

            if previous_price is not None:
                assert result.clean < previous_price

            previous_price = result.clean

    def test_zero_coupon_negative_yield(self):
        """Test negative yields are supported."""
        bond = quantrs.ZeroCouponBond(1000.0, "2030-12-31")
        dc = quantrs.DayCount("ACT/365F")

        result = bond.price(
            "2025-06-19",
            -0.01,
            dc,
        )

        assert result.clean > 1000.0

    def test_corporate_bond_pricing(self):
        """Test corporate bond pricing."""
        bond = quantrs.CorporateBond(
            1000.0,
            0.05,
            "2020-01-15",
            "2030-01-15",
            2,
            "BBB",
        )

        dc = quantrs.DayCount("30/360US")

        assert bond.face_value == 1000.0
        assert bond.coupon_rate == 0.05
        assert bond.frequency == 2
        assert bond.credit_rating == "BBB"

        result = bond.price(
            "2025-04-15",
            0.06,
            dc,
        )

        assert result.clean > 0
        assert result.dirty > result.clean
        assert result.accrued > 0

    def test_act_act_icma_requires_coupon_data(self):
        """ACT/ACT ICMA requires coupon period information."""
        dc = quantrs.DayCount("ACT/ACT ICMA")

        with pytest.raises(ValueError):
            dc.year_fraction("2025-01-01", "2025-07-01")
            
    def test_credit_spread(self):
        """Test corporate bond credit spreads."""
        ratings = {
            "AAA": 0.005,
            "AA": 0.010,
            "A": 0.015,
            "BBB": 0.025,
            "BB": 0.050,
            "B": 0.100,
        }

        for rating, spread in ratings.items():
            bond = quantrs.CorporateBond(
                1000.0,
                0.05,
                "2020-01-01",
                "2030-01-01",
                2,
                rating,
            )

            assert bond.credit_spread() == spread

    def test_convenience_functions(self):
        """Test convenience functions."""
        result1 = quantrs.calculate_year_fraction(
            "2025-01-01",
            "2025-07-01",
            "ACT/365F",
        )

        dc = quantrs.DayCount("ACT/365F")
        result2 = dc.year_fraction(
            "2025-01-01",
            "2025-07-01",
        )

        assert abs(result1 - result2) < 1e-10

    def test_error_handling(self):
        """Test error handling."""

        with pytest.raises(ValueError):
            quantrs.DayCount("INVALID_CONVENTION")

        with pytest.raises(ValueError):
            dc = quantrs.DayCount("ACT/365F")
            dc.year_fraction("invalid-date", "2025-07-01")

        with pytest.raises(ValueError):
            quantrs.ZeroCouponBond(
                1000.0,
                "invalid-date",
            )

    def test_repr_methods(self):
        """Test string representations."""

        dc = quantrs.DayCount("ACT/365F")
        assert "DayCount" in repr(dc)

        bond = quantrs.ZeroCouponBond(
            1000.0,
            "2030-12-31",
        )

        bond_repr = repr(bond)

        assert "ZeroCouponBond" in bond_repr
        assert "1000" in bond_repr
        assert "2030-12-31" in bond_repr

    def test_performance(self):
        """Basic performance test."""

        import time

        dc = quantrs.DayCount("ACT/365F")

        bond = quantrs.ZeroCouponBond(
            1000.0,
            "2030-12-31",
        )

        start = time.time()

        for _ in range(100):
            bond.price(
                "2025-06-19",
                0.04,
                dc,
            )

        elapsed = time.time() - start

        assert elapsed < 1.0