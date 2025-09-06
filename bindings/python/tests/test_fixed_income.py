"""
Pytest-compatible test script for quantrs Python bindings.
"""

import pytest
import quantrs


class TestQuantrsPythonBindings:
    """Test class for quantrs Python bindings."""

    def test_day_count(self):
        """Test day count functionality."""
        # Test all supported conventions
        conventions = [
            "ACT/365F",
            "ACT/365",
            "ACT/360",
            "30/360US",
            "30/360E",
            "ACT/ACT ISDA",
            "ACT/ACT ICMA",
        ]

        for conv in conventions:
            dc = quantrs.DayCount(conv)
            year_frac = dc.year_fraction("2025-01-01", "2025-07-01")
            days = dc.day_count("2025-01-01", "2025-07-01")

            # Basic assertions
            assert isinstance(year_frac, float)
            assert isinstance(days, int)
            assert year_frac > 0
            assert days > 0

    def test_bond_pricing(self):
        """Test bond pricing functionality."""
        # Create bond and day count
        bond = quantrs.ZeroCouponBond(1000.0, "2030-12-31")
        dc = quantrs.DayCount("ACT/365F")

        # Test properties
        assert bond.face_value == 1000.0
        assert bond.maturity == "2030-12-31"

        # Test pricing at different yields
        yields = [0.01, 0.02, 0.03, 0.04, 0.05]
        settlement = "2025-06-19"

        prev_price = None
        for ytm in yields:
            price = bond.price(settlement, ytm, dc)
            assert isinstance(price, float)
            assert price > 0
            assert price <= 1000.0  # Price should be <= face value for positive yields

            # Higher yield = lower price
            if prev_price is not None:
                assert price < prev_price
            prev_price = price

    def test_convenience_functions(self):
        """Test convenience functions."""
        # Test convenience function
        result1 = quantrs.calculate_year_fraction(
            "2025-01-01", "2025-07-01", "ACT/365F"
        )

        # Compare with direct method
        dc = quantrs.DayCount("ACT/365F")
        result2 = dc.year_fraction("2025-01-01", "2025-07-01")

        assert abs(result1 - result2) < 1e-10

    def test_error_handling(self):
        """Test error handling."""
        # Test invalid day count convention
        with pytest.raises(ValueError):
            quantrs.DayCount("INVALID_CONVENTION")

        # Test invalid date format
        with pytest.raises(ValueError):
            dc = quantrs.DayCount("ACT/365F")
            dc.year_fraction("invalid-date", "2025-07-01")

        # Test invalid bond maturity
        with pytest.raises(ValueError):
            quantrs.ZeroCouponBond(1000.0, "invalid-date")

    def test_repr_methods(self):
        """Test string representations."""
        # Test day count repr
        dc = quantrs.DayCount("ACT/365F")
        dc_repr = repr(dc)
        assert "DayCount" in dc_repr

        # Test bond repr
        bond = quantrs.ZeroCouponBond(1000.0, "2030-12-31")
        bond_repr = repr(bond)
        assert "ZeroCouponBond" in bond_repr
        assert "1000" in bond_repr
        assert "2030-12-31" in bond_repr

    def test_performance(self):
        """Basic performance test."""
        import time

        # Setup
        dc = quantrs.DayCount("ACT/365F")
        bond = quantrs.ZeroCouponBond(1000.0, "2030-12-31")
        settlement = "2025-06-19"
        ytm = 0.04

        # Time 100 calculations (fewer for pytest)
        n_iterations = 100
        start_time = time.time()

        for _ in range(n_iterations):
            price = bond.price(settlement, ytm, dc)

        end_time = time.time()
        total_time = end_time - start_time

        # Should be reasonably fast (less than 1 second for 100 calcs)
        assert total_time < 1.0
