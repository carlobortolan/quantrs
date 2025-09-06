"""
Example showing integration with pandas for bulk calculations.
"""

import pandas as pd
import quantrs
from datetime import datetime, timedelta

def create_sample_data():
    """Create sample bond portfolio data."""
    
    # Generate sample bonds
    base_date = datetime(2025, 1, 1)
    bonds_data = []
    
    for i in range(10):
        face_value = 1000 + (i * 100)  # 1000, 1100, 1200, ...
        maturity = base_date + timedelta(days=365 * (2 + i))  # 2-11 years
        ytm = 0.02 + (i * 0.005)  # 2% to 6.5%
        
        bonds_data.append({
            'bond_id': f'BOND_{i+1:02d}',
            'face_value': face_value,
            'maturity': maturity.strftime('%Y-%m-%d'),
            'ytm': ytm
        })
    
    return pd.DataFrame(bonds_data)

def calculate_portfolio_values():
    """Calculate values for entire bond portfolio."""
    
    print("=== Portfolio Valuation with pandas ===")
    
    # Create sample data
    df = create_sample_data()
    print("Bond Portfolio:")
    print(df)
    print()
    
    # Setup pricing parameters
    settlement = "2025-06-19"
    day_count = quantrs.DayCount("ACT/365F")
    
    # Method 1: Using apply function
    def price_bond(row):
        bond = quantrs.ZeroCouponBond(row['face_value'], row['maturity'])
        return bond.price(settlement, row['ytm'], day_count)
    
    df['price'] = df.apply(price_bond, axis=1)
    df['market_value'] = df['price']  # For zero-coupon bonds, 1 bond = price
    
    # Calculate portfolio metrics
    total_face_value = df['face_value'].sum()
    total_market_value = df['market_value'].sum()
    average_ytm = df['ytm'].mean()
    
    print("Portfolio Results:")
    print(f"Total Face Value: ${total_face_value:,.2f}")
    print(f"Total Market Value: ${total_market_value:,.2f}")
    print(f"Average YTM: {average_ytm:.2%}")
    print(f"Portfolio Discount: ${total_face_value - total_market_value:,.2f}")
    print()
    
    # Show detailed results
    print("Detailed Bond Pricing:")
    print(df[['bond_id', 'face_value', 'ytm', 'price', 'market_value']].to_string(index=False))
    print()

def day_count_analysis():
    """Compare different day count conventions."""
    
    print("=== Day Count Convention Analysis ===")
    
    # Date range for analysis
    start_date = "2025-01-01"
    end_dates = pd.date_range('2025-02-01', '2025-12-31', freq='M').strftime('%Y-%m-%d').tolist()
    
    # Different conventions
    conventions = ["ACT/365F", "ACT/360", "30/360US", "ACT/ACT ISDA"]
    
    # Calculate year fractions for each convention
    results = []
    
    for end_date in end_dates:
        row = {'end_date': end_date}
        for conv in conventions:
            try:
                year_frac = quantrs.calculate_year_fraction(start_date, end_date, conv)
                row[conv] = year_frac
            except ValueError:
                row[conv] = None
        results.append(row)
    
    df = pd.DataFrame(results)
    print(f"Year fractions from {start_date} using different conventions:")
    print(df.to_string(index=False, float_format='%.6f'))
    print()

if __name__ == "__main__":
    calculate_portfolio_values()
    day_count_analysis()