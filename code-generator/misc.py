from os import path
import pandas
from pandas import DataFrame

def main():
	mods_data = DataFrame.from_dict({
		'values': [1, 0.001, 0.000001, 0.000000001, 1000, 1000000, 1000000000],
		'symbol prefixes': ['', 'm', 'u', 'n', 'k', 'M', 'G'],
		'name prefixes': ['', 'milli', 'micro', 'nano', 'kilo', 'mega', 'giga'],
	})
	this_dir = path.dirname(path.abspath(__file__))
	units_data: DataFrame = pandas.read_csv(path.join(this_dir, 'unit-type-conversions.csv'))
	unit_names = units_data['name'].unique()
	# print('\n'.join(list(unit_names))); exit(0)
	unit_symbol_lut = {row['name']: row['unit symbol'] for i, row in units_data.iterrows()}
	unit_measure_lut = {row['name']: row['unit name'] for i, row in units_data.iterrows()}
	for u in unit_names:
		if u == 'temperature' or u == 'amount' or 'area' in u or 'volume' in u or 'angle' in u: continue
		for i, row in mods_data.iterrows():
			measure = row['name prefixes']+unit_measure_lut[u]
			symbol = row['symbol prefixes']+unit_symbol_lut[u]
			if row['symbol prefixes'] == '':
				print(u, measure, unit_symbol_lut[u], row['values'], '', sep='\t') # extra row for plain symbol
				symbol = unit_measure_lut[u].replace(' ', '_')
			print(u, measure, symbol, row['values'], '', sep='\t')

if __name__ == '__main__':
	main()