import os, sys, re
from os import path
from collections import defaultdict
from typing import *
import pandas, numpy
from pandas import DataFrame, Series
from numpy import ndarray
from templates import *

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
## the following are blacklisted as outputs because they are dimensionally equivalent to other more commonly used units
output_blacklist: Set[str] = set([
	'torque', 'moment of inertia', 'radioactivity', 'absorbed dose', 'dose equivalent'
])


def main(*args):
	'''
	Generate code for units and operator conversions (eg distance / time -> velocity)

	:param args: CLI args
	'''
	this_dir = path.dirname(path.abspath(__file__))
	project_root_dir= path.dirname(this_dir)
	main_proj_dir = path.join(project_root_dir, 'simple-si-units')
	recommended_unit_tests = defaultdict(lambda: [])
	#
	data: DataFrame = pandas.read_csv(path.join(this_dir, 'unit-type-definitions.csv'))
	data.sort_values(by=['category', 'name'], axis=0, ascending=[True, True], inplace=True)
	for i, row in data.iterrows():
		recommended_unit_tests['unit_names_and_symbols_test'].append(UNIT_NAME_TEST_TEMPLATE % {
			'struct': to_code_name(row['name']),
			'unit name': row['unit name'],
			'symbol': row['unit symbol human']
		})
		recommended_unit_tests['unit_print_display_test'].append(DISPLAY_TEST_TEMPLATE % {
			'struct': to_code_name(row['name']),
			'symbol': row['unit symbol']
		})
	from_to_unit_conversions: DataFrame = pandas.read_csv(path.join(this_dir, 'measurement-units.csv'))
	inverse_check(data, from_to_unit_conversions)
	print('Loaded units: %s' % ', '.join(data['name'].values))
	conversions = find_unit_conversions(data, test_recs=recommended_unit_tests)
	#
	data = add_capital_names(data, columns='category,name,desc first name,desc name,unit name'.split(','))
	data.insert(0, 'code name', data['name'].apply(to_code_name))
	conversions.insert(len(conversions.columns), 'op-function', conversions['operator'].apply(op_function_name))
	conversions = add_capital_names(conversions, columns=['left-side', 'right-side', 'result', 'operator', 'verbing'])
	#
	modules = list(data['category'].unique())
	modules.sort()
	for module_name in modules:
		module_file = path.join(main_proj_dir, 'src', '%s.rs' % module_name)
		generated_code = generate_modules(module_name, data, conversions, from_to_unit_conversions, test_recs=recommended_unit_tests)
		generated_code = post_gen_patching(generated_code)
		print('\n\n%s.rs:\n%s' % (module_file, generated_code))
		with open(module_file, 'w', newline='\n') as fout:
			fout.write(generated_code)
	#
	recommend_unit_tests(recommended_unit_tests, path.join(main_proj_dir, 'src', 'lib.rs'))
	# done!
def post_gen_patching(code: str) -> str:
	if '#[cfg(feature="num-bigfloat")]\nimpl' not in code:
		code = code.replace('#[cfg(feature="num-bigfloat")]\nuse num_bigfloat;', '')
	if '#[cfg(feature="num-complex")]\nimpl' not in code:
		code = code.replace('#[cfg(feature="num-complex")]\nuse num_complex;', '')
	if '#[cfg(feature="num-rational")]\nimpl' not in code:
		code = code.replace('#[cfg(feature="num-rational")]\nuse num_rational;', '')
	return code

def generate_modules(module: str, data: DataFrame, conversions: DataFrame, from_to_unit_conversions: DataFrame, test_recs: defaultdict) -> str:
	out_buf = ''
	mod_units = data[data['category'] == module]
	print(mod_units)
	out_buf += MODULE_TEMPLATE % {
		'category': module,
		'crate imports': generate_local_imports(module, data, conversions, test_recs),
		'example1': mod_units['desc first name'].iloc[0],
		'example2': mod_units['desc first name'].iloc[min(1 + len(mod_units)//2, len(mod_units)-1)],
		'content': generate_unit_structs(mod_units, conversions, from_to_unit_conversions, all_units=data.copy(), test_recs=test_recs),
		'appendix': get_appendix_for_module(module)
	}
	return out_buf


def get_appendix_for_module(module: str) -> str:
	return ''

def generate_local_imports(module: str, data: DataFrame, conversions: DataFrame, test_recs: defaultdict) -> str:
	other_modules = set()
	these_units = set(data[data['category'] == module]['name'].values)
	unit_module_lut = {row['name']: row['category'] for i, row in data.iterrows()}
	for i, row in conversions.iterrows():
		if row['left-side'] in these_units:
			other_modules.add(unit_module_lut[row['right-side']])
			other_modules.add(unit_module_lut[row['result']])
	for i, row in data[data['category'] == module].iterrows():
		_, inversion_mods = generate_inverse_unit_conversions(row, data, test_recs)
		other_modules = other_modules.union(inversion_mods)
	if module in other_modules: other_modules.discard(module)
	ilist = list(['use super::%s::*;' % m for m in other_modules])
	ilist.sort()
	return '\n'.join(ilist)

def generate_unit_structs(data: DataFrame, conversions: DataFrame, from_to_unit_conversions: DataFrame, all_units: DataFrame, test_recs: defaultdict) -> str:
	out_buf = ''
	for i, row in data.iterrows():
		inversions, _ = generate_inverse_unit_conversions(row, all_units, test_recs)
		out_buf += UNIT_STRUCT_DEFINITION_TEMPLATE % {
			**row.to_dict(),
			'non-converting methods': generate_nonconverting_from_to_conversions(row, from_to_unit_conversions, test_recs),
			'to-and-from': generate_from_to_conversions(row, from_to_unit_conversions, test_recs),
			'extended scalar ops': generate_extended_scalar_ops(row),
			'uom integration': generate_uom_conversions(row)
		}
		out_buf += generate_unit_conversions(row, conversions)
		out_buf += inversions
	return out_buf

def generate_uom_conversions(data_row: Series):
	if data_row['uom name'] is None or str(data_row['uom name']).lower() == 'nan':
		# no uom equivalent
		return ''
	output = ''
	for dt in ['f32', 'f64']:
		dd = {**data_row, 'data type': dt, 'uom data type': dt}
		output += INTO_UOM_TEMPLATE % dd
		output += FROM_UOM_TEMPLATE % dd
	return output

def generate_nonconverting_from_to_conversions(data_row: Series, from_to_unit_conversions: DataFrame, test_recs: defaultdict) -> str:
	unit_name = data_row['name']
	local_to_from = from_to_unit_conversions[from_to_unit_conversions['name'] == unit_name]
	local_to_from = local_to_from[numpy.logical_or(local_to_from['offset'] == 0, numpy.isnan(local_to_from['offset']))]
	local_to_from = local_to_from[local_to_from['slope'] == 1]
	out_buf = ''
	for i, row in local_to_from.iterrows():
		out_buf += NON_COEFFICIENT_TO_FROM_TEMPLATE % {
			'si unit symbol': data_row['unit symbol'],
			**data_row,
			'user unit symbol': row['unit symbol'],
			'user unit name': row['unit name']
		}
	return out_buf

def generate_from_to_conversions(data_row: Series, from_to_unit_conversions: DataFrame, test_recs: defaultdict) -> str:
	unit_name = data_row['name']
	local_to_from = from_to_unit_conversions[from_to_unit_conversions['name'] == unit_name]
	out_buf = ''
	for i, row in local_to_from.iterrows():
		if float(row['slope']) == 1 and (row['offset'] is None or numpy.isnan(row['offset']) or float(row['offset']) == 0):
			# already accounted for
			continue
		if row['offset'] is not None and numpy.isfinite(row['offset']) and row['offset'] != 0:
			# 2-factor conversion (ie temperature)
			out_buf += TO_FROM_SLOPE_OFFSET_TEMPLATE % {
				'si unit symbol': data_row['unit symbol'],
				**data_row,
				**row
			}
			test_recs['%s_units' % data_row['name'].replace(' ', '_')].append(
				MEASUREMENT_UNIT_TEST % {
					'struct': data_row['code name'],
					'symbol1': data_row['unit symbol'],
					'symbol2': row['unit symbol'],
					'u2 per u1': '%.13f' % (float(row['inverse slope']) - float(row['offset'])),
					'u1 per u2': '%.13f' % ((1.0 + float(row['offset'])) * float(row['slope']))
				}
			)
		else:
			out_buf += TO_FROM_SLOPE_TEMPLATE % {
				'si unit symbol': data_row['unit symbol'],
				**data_row,
				**row
			}
			if data_row['unit symbol'] != row['unit symbol']:
				test_recs['%s_units' % data_row['name'].replace(' ', '_')].append(
					MEASUREMENT_UNIT_TEST % {
						'struct': data_row['code name'],
						'symbol1': data_row['unit symbol'],
						'symbol2': row['unit symbol'],
						'u2 per u1': row['inverse slope'],
						'u1 per u2': row['slope']
					}
				)
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

def generate_extended_scalar_ops(data_row: Series):
	output = ''
	concrete_types = [
		'num_bigfloat::BigFloat',
		'num_complex::Complex32', 'num_complex::Complex64'
	]
	cfg_attrs = [
		'#[cfg(feature="num-bigfloat")]\n',
		'#[cfg(feature="num-complex")]\n', '#[cfg(feature="num-complex")]\n'
	]
	for n in range(0, len(concrete_types)):
		stype = concrete_types[n]
		cfg_stmnt = cfg_attrs[n]
		output += SCALAR_EXTENDED_TYPES_TEMPLATE % {
			'config attr prefix': cfg_stmnt,
			'scalar type': stype,
			**data_row
		}
	return output

def generate_inverse_unit_conversions(data_row: Series, data: DataFrame, test_recs: defaultdict) -> Tuple[str, Set[str]]:
	out_buf = ''
	used_mods = set()
	src_unit_name = data_row['name']
	src_units = SIUnits.from_str(data_row['si units'])
	inverse_units = src_units.inverse()
	concrete_types = [
		'f64', 'f32', 'i64', 'i32',
		'num_bigfloat::BigFloat',
		'num_complex::Complex32', 'num_complex::Complex64'
	]
	cfg_attrs = [
		'', '', '', '',
		'#[cfg(feature="num-bigfloat")]\n',
		'#[cfg(feature="num-complex")]\n', '#[cfg(feature="num-complex")]\n'
	]
	for i, row in data.iterrows():
		# print('\t%s (%s)' % (row['name'], SIUnits.from_str(row['si units'])))
		if row['name'] not in output_blacklist and inverse_units == SIUnits.from_str(row['si units']):
			# print('1/%s = %s' % (src_unit_name, row['name']))
			# found a match
			for n in range(0, len(concrete_types)):
				stype = concrete_types[n]
				cfg_stmnt = cfg_attrs[n]
				out_buf += INVERSE_CONVERSION_TEMPLATE % {
					**data_row,
					'config attr prefix': cfg_stmnt,
					'scalar type': stype,
					'code right-side': to_code_name(src_unit_name),
					'code result': to_code_name(row['name']),
					'right-side symbol': data_row['unit symbol'],
					'result symbol': row['unit symbol']
				}
				if 'num' in stype:
					test_name = 'test_%s_unit_conversions' % stype.replace('num_', '').split('::')[0]
					test_recs[test_name].append(SPECIAL_UNIT_INVERSE_CONVERSION_TEST_TEMPLATE % {
						'scalar type': re.sub('.+\\:\\:', '', stype),
						'code right-side': to_code_name(src_unit_name),
						'code result': to_code_name(row['name']),
						'right-side symbol': data_row['unit symbol'],
						'result symbol': row['unit symbol']
					})
				else:
					test_recs['test_unit_converions'].append(UNIT_INVERSE_CONVERSION_TEST_TEMPLATE % {
						'scalar type': stype,
						'code right-side': to_code_name(src_unit_name),
						'code result': to_code_name(row['name']),
						'right-side symbol': data_row['unit symbol'],
						'result symbol': row['unit symbol']
					})
			used_mods.add(row['category'])
			# only use first found unit
			break
	return out_buf, used_mods

def inverse_check(data: DataFrame, measurement_units: DataFrame):
	# check for and suggest inverse units
	unit_lut = {}
	unit_reverse_lut = defaultdict(lambda: [])
	missing_inverses = []
	for _, row in data.iterrows():
		name = row['name']
		units = SIUnits.from_str(row['si units'])
		unit_lut[name] = units
		unit_reverse_lut[units].append(name)
	for _, row in data.iterrows():
		name = row['name']
		units = unit_lut[name]
		inverse_unit = units.inverse()
		if inverse_unit in unit_reverse_lut:
			for inverse_name in unit_reverse_lut[inverse_unit]:
				print('1/%s = %s' % (name, inverse_name))
		else:
			print('No inverse unit found for %s' % name)
			missing_inverses.append(name)
	print('Suggested inverse units:')
	inverse_defs_buffer = []
	inverse_meas_buffer = []
	for no_inverse in missing_inverses:
		row = data[data['name'] == no_inverse].iloc[0]
		inverse_name = 'inverse '+row['name']
		# category, name, desc first name, desc name, unit name, unit symbol, si units, unit symbol human, uom name, uom module, uom type
		if ' per ' in row['unit name']:
			inverted_unit_name = rotate_string(row['unit name'], ' per ')
		else:
			inverted_unit_name = 'inverse '+row['unit name']
		if 'p' in row['unit symbol'][1:]:
			inverted_symbol = rotate_string(row['unit symbol'], 'p')
		else:
			inverted_symbol = 'per_'+row['unit symbol']
		inverse_unit_label = rotate_string(row['si units'], '/')
		if inverse_unit_label.startswith('/'):
			inverse_unit_label = '1'+inverse_unit_label
		inverse_human_symbol = rotate_string(row['unit symbol human'], '/')
		if inverse_human_symbol.startswith('/'):
			inverse_human_symbol = '1'+inverse_human_symbol
		inverse_defs_buffer.append('\t'.join([row['category'], inverse_name, 'inverse of '+row['desc first name'],
			'inverse '+row['desc name'], inverted_unit_name, inverted_symbol,
			inverse_unit_label, inverse_human_symbol,
			'', '', '', # can't auto-detect uom units (and they don't have inverted units defined in their SI module anyways)
			]))
		for _, measures in measurement_units[measurement_units['name'] == row['name']].iterrows():
			# inverse units of measure
			# name, unit name, unit symbol, slope, offset, inverse slope
			## cannot inverse linear offsets!
			if measures['offset'] is not None and numpy.isfinite(float(measures['offset'])) and float(measures['offset']) != 0:
				continue
			inverse_meas_buffer.append('\t'.join([str(x) for x in [
				inverse_name,
				rotate_string(measures['unit name'], ' per ') if ' per ' in measures['unit name'] else 'inverse '+measures['unit name'],
				rotate_string(measures['unit symbol'], ' p ') if 'p' in measures['unit symbol'][1:] else 'per_' + measures['unit symbol'],
				measures['inverse slope'],
				'',
				measures['slope']
			]]))

	print('\n\tunit type definitions')
	print('\n'.join(inverse_defs_buffer))
	print('\n\tunit measurement definitions')
	print('\n'.join(inverse_meas_buffer))
	print('\n')

def rotate_string(text, pivot_key):
	if pivot_key in text and not text.startswith(pivot_key):
		split = text.split(pivot_key, maxsplit=1)
		return '%s%s%s' % (split[1], pivot_key, split[0])
	else:
		return '%s%s' % (pivot_key, text)

def find_unit_conversions(data: DataFrame, test_recs: defaultdict) -> DataFrame:
	# make a look-up table of SI units and the measures with those units (there can be more than one measure with same units)
	siunit_measure_lut: Dict[SIUnits, List[str]] = defaultdict(lambda: [])
	for i, row in data.iterrows():
		if len(str(row['name']).strip()) > 0:
			siunit_measure_lut[SIUnits.from_str(row['si units'])].append(row['name'])
	for kv in siunit_measure_lut.items():
		if len(kv[1]) > 1:
			print('WARNING: dimensionally equivalent measures (%s): %s' % kv)
	siunit_symbol_lut: Dict[str, str] = {row['name']: row['unit symbol'] for _, row in data.iterrows()}
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
						test_recs['test_unit_converions'].append(UNIT_CONVERSION_TEST_TEMPLATE % {
							'op': 'mul', 'op symbol': '*',
							'left struct': to_code_name(this_name),
							'left symbol': siunit_symbol_lut[this_name],
							'right struct': to_code_name(other_name),
							'right symbol': siunit_symbol_lut[other_name],
							'out struct': to_code_name(output_name),
							'out symbol': siunit_symbol_lut[output_name]
						})
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
						test_recs['test_unit_converions'].append(UNIT_CONVERSION_TEST_TEMPLATE % {
							'op': 'div', 'op symbol': '/',
							'left struct': to_code_name(this_name),
							'left symbol': siunit_symbol_lut[this_name],
							'right struct': to_code_name(other_name),
							'right symbol': siunit_symbol_lut[other_name],
							'out struct': to_code_name(output_name),
							'out symbol': siunit_symbol_lut[output_name]
						})
	return DataFrame(unit_conversions, columns=['left-side', 'left-side symbol', 'operator', 'verbing', 'right-side', 'right-side symbol', 'result', 'result symbol', 'code left-side', 'code right-side', 'code result'])

def recommend_unit_tests(test_recs: defaultdict, test_filepath: str):
	with open(test_filepath, 'r') as fin:
		test_file_content = fin.read()
	print()
	print('================  RECOMMENDED UNIT TESTS ================')
	for test_fn in test_recs:
		count = 0
		print_buffer = []
		print_buffer.append('\t#[test]')
		print_buffer.append('\tfn %s() {' % test_fn)
		if test_fn.startswith('test_complex'):
			# add import
			print_buffer.append('\t\tuse num_complex::{Complex32, Complex64};')
		if test_fn.startswith('test_bigfloat'):
			# add import
			print_buffer.append('\t\tuse num_bigfloat::BigFloat;')
		if (
				': x}' in test_recs[test_fn][0] and ': y}' in test_recs[test_fn][0]
		) or (
			'(x)' in test_recs[test_fn][0] and '(y)' in test_recs[test_fn][0]
		):
			# add x and y
			print_buffer.append('\t\tlet x = 4.5f64;\n\t\tlet y = 2.5f64;')
		print_buffer.append('\t\t// ...')
		for unit_test in test_recs[test_fn]:
			unit_test = unit_test.replace('Complex32::from(x)', 'Complex32::from(x as f32)').replace('Complex32::from(y)', 'Complex32::from(y as f32)')
			if unit_test.strip() not in test_file_content:
				print_buffer.append(unit_test)
				count += 1
		print_buffer.append('}\n')
		if count != 0:
			print('\n'.join(print_buffer))

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

	def inverse(self) -> 'SIUnits':
		return SIUnits(numerator=self.denominator, denominator=self.numerator)

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
