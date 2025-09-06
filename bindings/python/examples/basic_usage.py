"""
Basic usage examples for quantrs Python library.
"""

import quantrs

def day_count_examples():
    """Examples of day count calculations."""
    print("=== Day Count Examples ===")
    
    # Create day count convention
    dc = quantrs.DayCount("ACT/365F")
    print(f"Day count convention: {dc}")
    
    # Calculate year fraction
    start_date = "2025-01-01"
    end_date = "2025-07-01"
    
    year_frac = dc.year_fraction(start_date, end_date)
    print(f"Year fraction from {start_date} to {end_date}: {year_frac:.6f}")
    
    # Calculate day count
    days = dc.day_count(start_date, end_date)
    print(f"Day count: {days} days")
    
    # Convenience function
    year_frac2 = quantrs.calculate_year_fraction(start_date, end_date, "ACT/365F")
    print(f"Using convenience function: {year_frac2:.6f}")
    
    print()

def bond_pricing_examples():
    """Examples of bond pricing."""
    print("=== Bond Pricing Examples ===")
    
    # Create zero-coupon bond
    bond = quantrs.ZeroCouponBond(face_value=1000.0, maturity="2030-12-31")
    print(f"Bond: {bond}")
    
    # Create day count convention
    day_count = quantrs.DayCount("ACT/365F")
    
    # Calculate bond price at different yields
    settlement = "2025-06-19"
    yields = [0.02, 0.03, 0.04, 0.05, 0.06]
    
    print(f"Bond pricing on {settlement}:")
    print("YTM     | Price")
    print("--------|--------")
    
    for ytm in yields:
        price = bond.price(settlement, ytm, day_count)
        print(f"{ytm:6.1%} | ${price:6.2f}")
    
    print()

def error_handling_examples():
    """Examples of error handling."""
    print("=== Error Handling Examples ===")
    
    # Invalid day count convention
    try:
        dc = quantrs.DayCount("INVALID")
    except ValueError as e:
        print(f"Invalid convention error: {e}")
    
    # Invalid date format
    try:
        dc = quantrs.DayCount("ACT/365F")
        result = dc.year_fraction("invalid-date", "2025-07-01")
        print(f"Year fraction: {result}")
    except ValueError as e:
        print(f"Invalid date error: {e}")
    
    # Settlement after maturity
    try:
        bond = quantrs.ZeroCouponBond(1000.0, "2025-12-31")
        dc = quantrs.DayCount("ACT/365F")
        price = bond.price("2026-01-01", 0.04, dc)  # Settlement after maturity
        print(f"Bond price: {price}")
    except ValueError as e:
        print(f"Bond pricing error: {e}")
    
    print()

if __name__ == "__main__":
    day_count_examples()
    bond_pricing_examples()
    error_handling_examples()