import os, sys
from os import path
from collections import defaultdict
from typing import *
import pandas, numpy
from pandas import DataFrame, Series
from numpy import ndarray
from templates import *

def main(*args):
	'''
	Generate code for units and operator conversions (eg distance / time -> velocity)

	:param args: CLI args
	'''
	this_dir = path.dirname(path.abspath(__file__))
	project_root_dir= path.dirname(this_dir)
	main_proj_dir = path.join(project_root_dir, 'simple-si-units')
	data: DataFrame = pandas.read_csv(path.join(this_dir, 'unit-type-conversions.csv'))
	from_to_unit_conversions: DataFrame = pandas.read_csv(path.join(this_dir, 'measurement-units.csv'))
	print('Loaded units: %s' % ', '.join(data['name'].values))
	conversions = find_unit_conversions(data)
	#
	data = add_capital_names(data, columns='category,name,desc first name,desc name,unit name'.split(','))
	data.insert(0, 'code name', data['name'].apply(to_code_name))
	conversions.insert(len(conversions.columns), 'op-function', conversions['operator'].apply(op_function_name))
	conversions = add_capital_names(conversions, columns=['left-side', 'right-side', 'result', 'operator', 'verbing'])
	#
	modules = data['category'].unique()
	for module_name in modules:
		module_file = path.join(main_proj_dir, 'src', '%s.rs' % module_name)
		generated_code = generate_modules(module_name, data, conversions, from_to_unit_conversions)
		print('\n\n%s.rs:\n%s' % (module_file, generated_code))
		with open(module_file, 'w', newline='\n') as fout:
			fout.write(generated_code)
	raise Exception('Work in Progress (WIP)')

def generate_modules(module: str, data: DataFrame, conversions: DataFrame, from_to_unit_conversions: DataFrame) -> str:
	out_buf = ''
	mod_units = data[data['category'] == module]
	print(mod_units)
	out_buf += MODULE_TEMPLATE % {
		'category': module,
		'example1': mod_units['desc first name'].iloc[0],
		'example2': mod_units['desc first name'].iloc[min(1 + len(mod_units)//2, len(mod_units)-1)],
		'content': generate_unit_structs(mod_units, conversions, from_to_unit_conversions)
	}
	return out_buf

def generate_unit_structs(data: DataFrame, conversions: DataFrame, from_to_unit_conversions: DataFrame) -> str:
	out_buf = ''
	for i, row in data.iterrows():
		out_buf += UNIT_STRUCT_DEFINITION_TEMPLATE % {
			**row.to_dict(),
			'to-and-from': generate_from_to_conversions(row, from_to_unit_conversions) + generate_unit_conversions(row, conversions),

		}
	return out_buf


def generate_from_to_conversions(data_row: Series, from_to_unit_conversions: DataFrame) -> str:
	unit_name = data_row['name']
	local_to_from = from_to_unit_conversions[from_to_unit_conversions['name'] == unit_name]
	out_buf = ''
	for i, row in local_to_from.iterrows():
		if row['unit symbol'] == data_row['unit symbol']:
			# already accounted for
			continue
		if row['offset'] is not None and numpy.isfinite(row['offset']) and row['offset'] != 0:
			# 2-factor conversion (ie temperature)
			out_buf += TO_FROM_SLOPE_OFFSET_TEMPLATE % {
				'si unit symbol': data_row['unit symbol'],
				**data_row,
				**row
			}
		else:
			out_buf += TO_FROM_SLOPE_TEMPLATE % {
				'si unit symbol': data_row['unit symbol'],
				**data_row,
				**row
			}
	return out_buf


def generate_unit_conversions(data_row: Series, conversions: DataFrame) -> str:
	out_buf = ''
	left_unit = data_row['name']
	converts = conversions[conversions['left-side'] == left_unit]
	for i, row in converts.iterrows():
		out_buf += UNIT_CONVERSION_TEMPLATE % {
			'capital op-function': str(row['op-function']).capitalize(),
			**data_row,
			**row
		}
	return out_buf
def find_unit_conversions(data: DataFrame) -> DataFrame:
	# make a look-up table of SI units and the measures with those units (there can be more than one measure with same units)
	siunit_measure_lut: Dict[SIUnits, List[str]] = defaultdict(lambda: [])
	for i, row in data.iterrows():
		if len(str(row['name']).strip()) > 0:
			siunit_measure_lut[SIUnits.from_str(row['si units'])].append(row['name'])
	for kv in siunit_measure_lut.items():
		if len(kv[1]) > 1:
			print('WARNING: dimensionally equivalent measures (%s): %s' % kv)
	siunit_symbol_lut: Dict[str, str] = {row['name']: row['unit symbol'] for _, row in data.iterrows()}
	# blacklist a few illogical combinations:
	input_blacklist: Set[str] = set([
		'radioactivity', 'absorbed dose', 'dose equivalent',
	])
	combo_whitelist: Set[Tuple[str, str, str]] = set([
		('mass', 'absorbed dose', 'energy'),
		('absorbed dose', 'mass', 'energy'),
		('energy', 'absorbed dose', 'mass'),
		('absorbed dose', 'energy', 'mass'),
		('mass', 'dose equivalent', 'energy'),
		('dose equivalent', 'mass', 'energy'),
		('energy', 'dose equivalent', 'mass'),
		('dose equivalent', 'energy', 'mass'),
		('moment of inertia', 'angular acceleration', 'torque'),
		('angular acceleration', 'moment of inertia', 'torque'),
		('angular acceleration', 'torque', 'moment of inertia'),
		('torque', 'angular acceleration', 'moment of inertia'),
		('torque', 'moment of inertia', 'angular acceleration'),
	])
	output_blacklist: Set[str] = set([
		'torque', 'moment of inertia', 'radioactivity', 'absorbed dose', 'dose equivalent'
	])
	unit_conversions = []
	# now check every non-blacklisted  A * B and A / B combination for possible unit conversions
	for i, row in data.iterrows():
		this_unit = SIUnits.from_str(row['si units'])
		this_name = row['name']
		print(this_name, this_unit)
		for other_unit in siunit_measure_lut:
			mul_unit = this_unit * other_unit
			if mul_unit in siunit_measure_lut:
				for other_name in siunit_measure_lut[other_unit]:
					for output_name in siunit_measure_lut[mul_unit]:
						if (this_name, other_name, output_name) not in combo_whitelist and \
							(this_name in input_blacklist or other_name in input_blacklist or output_name in output_blacklist):
							continue
						print('\t%s (%s) * %s (%s) -> %s (%s)' % (
							this_name, this_unit,
							other_name, other_unit,
							output_name, mul_unit
						))
						unit_conversions.append((this_name, siunit_symbol_lut[this_name], '*', 'multiplying', other_name, siunit_symbol_lut[other_name], output_name, siunit_symbol_lut[output_name], to_code_name(this_name), to_code_name(other_name), to_code_name(output_name)))
			div_unit = this_unit / other_unit
			if div_unit in siunit_measure_lut:
				for other_name in siunit_measure_lut[other_unit]:
					for output_name in siunit_measure_lut[div_unit]:
						if (this_name, other_name, output_name) not in combo_whitelist and \
							(this_name in input_blacklist or other_name in input_blacklist or output_name in output_blacklist):
							continue
						print('\t%s (%s) / %s (%s) -> %s (%s)' % (
							this_name, this_unit,
							other_name, other_unit,
							output_name, div_unit
						))
						unit_conversions.append((this_name, siunit_symbol_lut[this_name], '/', 'dividing', other_name, siunit_symbol_lut[other_name], output_name, siunit_symbol_lut[output_name], to_code_name(this_name), to_code_name(other_name), to_code_name(output_name)))
	return DataFrame(unit_conversions, columns=['left-side', 'left-side symbol', 'operator', 'verbing', 'right-side', 'right-side symbol', 'result', 'result symbol', 'code left-side', 'code right-side', 'code result'])

class SIUnits:
	def __init__(self, numerator: List[str], denominator: List[str]):
		self.numerator = numerator
		self.denominator = denominator
		self._simplify()

	def _simplify(self):
		self._cancel_units()
		self._sort()

	def _sort(self):
		self.numerator.sort()
		self.denominator.sort()

	def _cancel_units(self):
		for i in range(0, len(self.numerator)):
			n = self.numerator[i]
			for j in range(0, len(self.denominator)):
				d = self.denominator[j]
				if n == d:
					del self.numerator[i]
					del self.denominator[j]
					self._cancel_units()
					return
		return
		# done
	def __members(self):
		return (tuple(self.numerator), tuple(self.denominator))

	def __eq__(self, other):
		if type(other) is type(self):
			self._simplify()
			other._simplify()
			return self.__members() == other.__members()
		else:
			return False

	def __hash__(self):
		return hash(self.__members())
	
	def __str__(self):
		self._simplify()
		if self.denominator is not None and len(self.denominator) > 0:
			return '%s/%s' % (condense_units(self.numerator), condense_units(self.denominator))
		else:
			return condense_units(self.numerator)

	def from_str(s: str):
		if '/' in s:
			numer, denoms = s.split('/', maxsplit=1)
			denom = denoms.replace('/', '.')
		else:
			numer = s
			denom = ''
		n_list = expand_units(numer)
		d_list = expand_units(denom)
		return SIUnits(
			numerator=n_list,
			denominator=d_list
		)

	def __mul__(self, other):
		if type(other) is type(self):
			return SIUnits(
				numerator=self.numerator + other.numerator,
				denominator=self.denominator + other.denominator
			)
		else:
			raise TypeError('%s is not a type of %s' % (type(other).__name__, type(self).__name__))


	def __truediv__(self, other):
		if type(other) is type(self):
			return SIUnits(
				numerator=self.numerator + other.denominator,
				denominator=self.denominator + other.numerator
			)
		else:
			raise TypeError('%s is not a type of %s' % (type(other).__name__, type(self).__name__))

	
def condense_units(unit_list: List[str]) -> str:
	'''condense repeated units to power notation'''
	cond_output = ''
	n_parsed = set()
	for n in unit_list:
		if n in n_parsed:
			continue
		if len(cond_output) > 0:
			cond_output += '.' # dot multiply
		count = unit_list.count(n)
		if count > 1:
			cond_output += '%s^%s' % (n, count)
		else:
			cond_output += n
		n_parsed.add(n)
	return cond_output

def expand_units(units_str: str) -> List[str]:
	'''convert 'kg.m^3' to ['kg', 'm', 'm', 'm']'''
	if units_str == '1':
		# special case: inverted unit
		return []
	u_list = [x for x in units_str.split('.') if len(x) > 0]
	out_list = []
	for u in u_list:
		if '^' in u:
			# raised to a power
			x, n = u.split('^')
			for i in range(0, int(n)):
				out_list.append(x)
		else:
			out_list.append(u)
	return out_list


def add_capital_names(df: DataFrame, columns: List[str]) -> DataFrame:
	df2 = df.copy()
	for col in columns:
		new_col = 'capital '+col
		df2.insert(len(df2.columns), new_col, df2[col].apply(capitalize))
	return df2


def capitalize(x: Any) -> str:
	if x is None: return ''
	return str(x).capitalize()


def to_code_name(n: str) -> str:
	return ''.join([capitalize(x) for x in str(n).split(' ')])


def op_function_name(symbol: str) -> str:
	if symbol == '*': return 'mul'
	if symbol == '/': return 'div'
	raise ValueError('"%s" is not a supported operator' % symbol)

if __name__ == '__main__':
	main(*sys.argv[1:])