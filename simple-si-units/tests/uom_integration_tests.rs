
#[cfg(feature="uom")]
#[cfg(test)]
mod uom_integration {
	use uom::si;
	use simple_si_units;

	#[test]
	fn spot_check_uom_library(){
		// test that the uom lib is working
		// example from uom docs:
		{
			use uom::si::f32::*;
			use uom::si::length::kilometer;
			use uom::si::time::second;
			let length = Length::new::<kilometer>(5.0);
			let time = Time::new::<second>(15.0);
			let velocity = length / time;
			println!("length {:?} / time {:?} = velocity {:?}", length, time, velocity);
			let x: f32 = velocity.value;
			println!("{}", x);
		}
		// uom unit conversion spot check
		assert_eq!(
			si::f64::Length::new::<si::length::meter>(1000.),
			si::f64::Length::new::<si::length::kilometer>(1.),
			"uom assert fail: 1000 m per km");
		assert_eq!(
			si::f64::MolarConcentration::new::<si::molar_concentration::mole_per_cubic_meter>(1.),
			si::f64::MolarConcentration::new::<si::molar_concentration::millimole_per_liter>(1.),
			"uom assert fail: 1 mol/m3 = 1 mM");
		assert_eq!(
			si::f64::Velocity::new::<si::velocity::meter_per_second>(5.)
			/ si::f64::Length::new::<si::length::meter>(2.),
			si::f64::Frequency::new::<si::frequency::hertz>(2.5),
			"uom assert fail: 5 mps / 2 m = 2.5 Hz");
	}

	#[test]
	fn uom_equivalence_test() {
		let x: f64 = 2.5;
		assert_eq!(simple_si_units::base::Amount{mol: x}.mol,
			uom::si::f64::AmountOfSubstance::new::<uom::si::amount_of_substance::mole>(x).value);
		assert_eq!(simple_si_units::base::Current{A: x}.A,
			uom::si::f64::ElectricCurrent::new::<uom::si::electric_current::ampere>(x).value);
		assert_eq!(simple_si_units::base::Distance{m: x}.m,
			uom::si::f64::Length::new::<uom::si::length::meter>(x).value);
		assert_eq!(simple_si_units::base::Luminosity{cd: x}.cd,
			uom::si::f64::LuminousIntensity::new::<uom::si::luminous_intensity::candela>(x).value);
		assert_eq!(simple_si_units::base::Mass{kg: x}.kg,
			uom::si::f64::Mass::new::<uom::si::mass::kilogram>(x).value);
		assert_eq!(simple_si_units::base::Temperature{K: x}.K,
			uom::si::f64::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::kelvin>(x).value);
		assert_eq!(simple_si_units::base::Time{s: x}.s,
			uom::si::f64::Time::new::<uom::si::time::second>(x).value);
		assert_eq!(simple_si_units::chemical::CatalyticActivity{molps: x}.molps,
			uom::si::f64::CatalyticActivity::new::<uom::si::catalytic_activity::mole_per_second>(x).value);
		assert_eq!(simple_si_units::chemical::Concentration{molpm3: x}.molpm3,
			uom::si::f64::MolarConcentration::new::<uom::si::molar_concentration::mole_per_cubic_meter>(x).value);
		assert_eq!(simple_si_units::chemical::Molality{molpkg: x}.molpkg,
			uom::si::f64::Molality::new::<uom::si::molality::mole_per_kilogram>(x).value);
		assert_eq!(simple_si_units::chemical::MolarMass{kgpmol: x}.kgpmol,
			uom::si::f64::MolarMass::new::<uom::si::molar_mass::kilogram_per_mole>(x).value);
		assert_eq!(simple_si_units::chemical::SpecificHeatCapacity{J_per_kgK: x}.J_per_kgK,
			uom::si::f64::SpecificHeatCapacity::new::<uom::si::specific_heat_capacity::joule_per_kilogram_kelvin>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Conductance{S: x}.S,
			uom::si::f64::ElectricalConductance::new::<uom::si::electrical_conductance::siemens>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Capacitance{F: x}.F,
			uom::si::f64::Capacitance::new::<uom::si::capacitance::farad>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Charge{C: x}.C,
			uom::si::f64::ElectricCharge::new::<uom::si::electric_charge::coulomb>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Illuminance{lux: x}.lux,
			uom::si::f64::Luminance::new::<uom::si::luminance::candela_per_square_meter>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Inductance{H: x}.H,
			uom::si::f64::Inductance::new::<uom::si::inductance::henry>(x).value);
		assert_eq!(simple_si_units::electromagnetic::MagneticFlux{Wb: x}.Wb,
			uom::si::f64::MagneticFlux::new::<uom::si::magnetic_flux::weber>(x).value);
		assert_eq!(simple_si_units::electromagnetic::MagneticFluxDensity{T: x}.T,
			uom::si::f64::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::tesla>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Resistance{Ohm: x}.Ohm,
			uom::si::f64::ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(x).value);
		assert_eq!(simple_si_units::electromagnetic::Voltage{V: x}.V,
			uom::si::f64::ElectricPotential::new::<uom::si::electric_potential::volt>(x).value);
		assert_eq!(simple_si_units::geometry::Angle{rad: x}.rad,
			uom::si::f64::Angle::new::<uom::si::angle::radian>(x).value);
		assert_eq!(simple_si_units::geometry::Area{m2: x}.m2,
			uom::si::f64::Area::new::<uom::si::area::square_meter>(x).value);
		assert_eq!(simple_si_units::geometry::Volume{m3: x}.m3,
			uom::si::f64::Volume::new::<uom::si::volume::cubic_meter>(x).value);
		assert_eq!(simple_si_units::mechanical::Acceleration{mps2: x}.mps2,
			uom::si::f64::Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(x).value);
		assert_eq!(simple_si_units::mechanical::AngularAcceleration{radps2: x}.radps2,
			uom::si::f64::AngularAcceleration::new::<uom::si::angular_acceleration::radian_per_second_squared>(x).value);
		assert_eq!(simple_si_units::mechanical::AngularVelocity{radps: x}.radps,
			uom::si::f64::AngularVelocity::new::<uom::si::angular_velocity::radian_per_second>(x).value);
		assert_eq!(simple_si_units::mechanical::AreaDensity{kgpm2: x}.kgpm2,
			uom::si::f64::ArealMassDensity::new::<uom::si::areal_mass_density::kilogram_per_square_meter>(x).value);
		assert_eq!(simple_si_units::mechanical::Density{kgpm3: x}.kgpm3,
			uom::si::f64::MassDensity::new::<uom::si::mass_density::kilogram_per_cubic_meter>(x).value);
		assert_eq!(simple_si_units::mechanical::Energy{J: x}.J,
			uom::si::f64::Energy::new::<uom::si::energy::joule>(x).value);
		assert_eq!(simple_si_units::mechanical::Force{N: x}.N,
			uom::si::f64::Force::new::<uom::si::force::newton>(x).value);
		assert_eq!(simple_si_units::mechanical::Frequency{Hz: x}.Hz,
			uom::si::f64::Frequency::new::<uom::si::frequency::hertz>(x).value);
		assert_eq!(simple_si_units::mechanical::MomentOfInertia{kgm2: x}.kgm2,
			uom::si::f64::MomentOfInertia::new::<uom::si::moment_of_inertia::kilogram_square_meter>(x).value);
		assert_eq!(simple_si_units::mechanical::Momentum{kgmps: x}.kgmps,
			uom::si::f64::Momentum::new::<uom::si::momentum::kilogram_meter_per_second>(x).value);
		assert_eq!(simple_si_units::mechanical::Power{W: x}.W,
			uom::si::f64::Power::new::<uom::si::power::watt>(x).value);
		assert_eq!(simple_si_units::mechanical::Pressure{Pa: x}.Pa,
			uom::si::f64::Pressure::new::<uom::si::pressure::pascal>(x).value);
		assert_eq!(simple_si_units::mechanical::Torque{Nm: x}.Nm,
			uom::si::f64::Torque::new::<uom::si::torque::newton_meter>(x).value);
		assert_eq!(simple_si_units::mechanical::Velocity{mps: x}.mps,
			uom::si::f64::Velocity::new::<uom::si::velocity::meter_per_second>(x).value);
		assert_eq!(simple_si_units::nuclear::Radioactivity{Bq: x}.Bq,
			uom::si::f64::Radioactivity::new::<uom::si::radioactivity::becquerel>(x).value);
		assert_eq!(simple_si_units::base::InverseDistance{per_m: x}.per_m,
				   uom::si::f64::LinearNumberDensity::new::<uom::si::linear_number_density::per_meter>(x).value);
		assert_eq!(simple_si_units::base::InverseTemperature{per_K: x}.per_K,
				   uom::si::f64::TemperatureCoefficient::new::<uom::si::temperature_coefficient::per_kelvin>(x).value);
		assert_eq!(simple_si_units::chemical::MolarVolume{m3_per_mol: x}.m3_per_mol,
				   uom::si::f64::MolarVolume::new::<uom::si::molar_volume::cubic_meter_per_mole>(x).value);
		assert_eq!(simple_si_units::geometry::InverseArea{per_m2: x}.per_m2,
				   uom::si::f64::ArealNumberDensity::new::<uom::si::areal_number_density::per_square_meter>(x).value);
		assert_eq!(simple_si_units::geometry::InverseVolume{per_m3: x}.per_m3,
				   uom::si::f64::VolumetricNumberDensity::new::<uom::si::volumetric_number_density::per_cubic_meter>(x).value);
		assert_eq!(simple_si_units::geometry::SolidAngle{sr: x}.sr,
				   uom::si::f64::SolidAngle::new::<uom::si::solid_angle::steradian>(x).value);
		assert_eq!(simple_si_units::mechanical::AreaPerMass{m2_per_kg: x}.m2_per_kg,
				   uom::si::f64::SpecificArea::new::<uom::si::specific_area::square_meter_per_kilogram>(x).value);
		assert_eq!(simple_si_units::mechanical::VolumePerMass{m3_per_kg: x}.m3_per_kg,
				   uom::si::f64::SpecificVolume::new::<uom::si::specific_volume::cubic_meter_per_kilogram>(x).value);
	}

	#[test]
	fn into_uom_test_f64(){
		let x: f64 = 2.5;
		assert!(uom::si::f64::AmountOfSubstance::new::<uom::si::amount_of_substance::mole>(x)
			== simple_si_units::base::Amount{mol: x}.into());
		assert!(uom::si::f64::ElectricCurrent::new::<uom::si::electric_current::ampere>(x)
			== simple_si_units::base::Current{A: x}.into());
		assert!(uom::si::f64::Length::new::<uom::si::length::meter>(x)
			== simple_si_units::base::Distance{m: x}.into());
		assert!(uom::si::f64::LuminousIntensity::new::<uom::si::luminous_intensity::candela>(x)
			== simple_si_units::base::Luminosity{cd: x}.into());
		assert!(uom::si::f64::Mass::new::<uom::si::mass::kilogram>(x)
			== simple_si_units::base::Mass{kg: x}.into());
		assert!(uom::si::f64::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::kelvin>(x)
			== simple_si_units::base::Temperature{K: x}.into());
		assert!(uom::si::f64::Time::new::<uom::si::time::second>(x)
			== simple_si_units::base::Time{s: x}.into());
		assert!(uom::si::f64::CatalyticActivity::new::<uom::si::catalytic_activity::mole_per_second>(x)
			== simple_si_units::chemical::CatalyticActivity{molps: x}.into());
		assert!(uom::si::f64::MolarConcentration::new::<uom::si::molar_concentration::mole_per_cubic_meter>(x)
			== simple_si_units::chemical::Concentration{molpm3: x}.into());
		assert!(uom::si::f64::Molality::new::<uom::si::molality::mole_per_kilogram>(x)
			== simple_si_units::chemical::Molality{molpkg: x}.into());
		assert!(uom::si::f64::MolarMass::new::<uom::si::molar_mass::kilogram_per_mole>(x)
			== simple_si_units::chemical::MolarMass{kgpmol: x}.into());
		assert!(uom::si::f64::SpecificHeatCapacity::new::<uom::si::specific_heat_capacity::joule_per_kilogram_kelvin>(x)
			== simple_si_units::chemical::SpecificHeatCapacity{J_per_kgK: x}.into());
		assert!(uom::si::f64::Capacitance::new::<uom::si::capacitance::farad>(x)
			== simple_si_units::electromagnetic::Capacitance{F: x}.into());
		assert!(uom::si::f64::ElectricCharge::new::<uom::si::electric_charge::coulomb>(x)
			== simple_si_units::electromagnetic::Charge{C: x}.into());
		assert!(uom::si::f64::ElectricalConductance::new::<uom::si::electrical_conductance::siemens>(x)
			== simple_si_units::electromagnetic::Conductance{S: x}.into());
		assert!(uom::si::f64::Luminance::new::<uom::si::luminance::candela_per_square_meter>(x)
			== simple_si_units::electromagnetic::Illuminance{lux: x}.into());
		assert!(uom::si::f64::Inductance::new::<uom::si::inductance::henry>(x)
			== simple_si_units::electromagnetic::Inductance{H: x}.into());
		assert!(uom::si::f64::MagneticFlux::new::<uom::si::magnetic_flux::weber>(x)
			== simple_si_units::electromagnetic::MagneticFlux{Wb: x}.into());
		assert!(uom::si::f64::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::tesla>(x)
			== simple_si_units::electromagnetic::MagneticFluxDensity{T: x}.into());
		assert!(uom::si::f64::ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(x)
			== simple_si_units::electromagnetic::Resistance{Ohm: x}.into());
		assert!(uom::si::f64::ElectricPotential::new::<uom::si::electric_potential::volt>(x)
			== simple_si_units::electromagnetic::Voltage{V: x}.into());
		assert!(uom::si::f64::Angle::new::<uom::si::angle::radian>(x)
			== simple_si_units::geometry::Angle{rad: x}.into());
		assert!(uom::si::f64::Area::new::<uom::si::area::square_meter>(x)
			== simple_si_units::geometry::Area{m2: x}.into());
		assert!(uom::si::f64::Volume::new::<uom::si::volume::cubic_meter>(x)
			== simple_si_units::geometry::Volume{m3: x}.into());
		assert!(uom::si::f64::Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(x)
			== simple_si_units::mechanical::Acceleration{mps2: x}.into());
		assert!(uom::si::f64::AngularAcceleration::new::<uom::si::angular_acceleration::radian_per_second_squared>(x)
			== simple_si_units::mechanical::AngularAcceleration{radps2: x}.into());
		assert!(uom::si::f64::AngularVelocity::new::<uom::si::angular_velocity::radian_per_second>(x)
			== simple_si_units::mechanical::AngularVelocity{radps: x}.into());
		assert!(uom::si::f64::ArealMassDensity::new::<uom::si::areal_mass_density::kilogram_per_square_meter>(x)
			== simple_si_units::mechanical::AreaDensity{kgpm2: x}.into());
		assert!(uom::si::f64::MassDensity::new::<uom::si::mass_density::kilogram_per_cubic_meter>(x)
			== simple_si_units::mechanical::Density{kgpm3: x}.into());
		assert!(uom::si::f64::Energy::new::<uom::si::energy::joule>(x)
			== simple_si_units::mechanical::Energy{J: x}.into());
		assert!(uom::si::f64::Force::new::<uom::si::force::newton>(x)
			== simple_si_units::mechanical::Force{N: x}.into());
		assert!(uom::si::f64::Frequency::new::<uom::si::frequency::hertz>(x)
			== simple_si_units::mechanical::Frequency{Hz: x}.into());
		assert!(uom::si::f64::MomentOfInertia::new::<uom::si::moment_of_inertia::kilogram_square_meter>(x)
			== simple_si_units::mechanical::MomentOfInertia{kgm2: x}.into());
		assert!(uom::si::f64::Momentum::new::<uom::si::momentum::kilogram_meter_per_second>(x)
			== simple_si_units::mechanical::Momentum{kgmps: x}.into());
		assert!(uom::si::f64::Power::new::<uom::si::power::watt>(x)
			== simple_si_units::mechanical::Power{W: x}.into());
		assert!(uom::si::f64::Pressure::new::<uom::si::pressure::pascal>(x)
			== simple_si_units::mechanical::Pressure{Pa: x}.into());
		assert!(uom::si::f64::Torque::new::<uom::si::torque::newton_meter>(x)
			== simple_si_units::mechanical::Torque{Nm: x}.into());
		assert!(uom::si::f64::Velocity::new::<uom::si::velocity::meter_per_second>(x)
			== simple_si_units::mechanical::Velocity{mps: x}.into());
		assert!(uom::si::f64::Radioactivity::new::<uom::si::radioactivity::becquerel>(x)
			== simple_si_units::nuclear::Radioactivity{Bq: x}.into());
		assert!(uom::si::f64::LinearNumberDensity::new::<uom::si::linear_number_density::per_meter>(x)
			== simple_si_units::base::InverseDistance{per_m: x}.into());
		assert!(uom::si::f64::TemperatureCoefficient::new::<uom::si::temperature_coefficient::per_kelvin>(x)
			== simple_si_units::base::InverseTemperature{per_K: x}.into());
		assert!(uom::si::f64::MolarVolume::new::<uom::si::molar_volume::cubic_meter_per_mole>(x)
			== simple_si_units::chemical::MolarVolume{m3_per_mol: x}.into());
		assert!(uom::si::f64::ArealNumberDensity::new::<uom::si::areal_number_density::per_square_meter>(x)
			== simple_si_units::geometry::InverseArea{per_m2: x}.into());
		assert!(uom::si::f64::VolumetricNumberDensity::new::<uom::si::volumetric_number_density::per_cubic_meter>(x)
			== simple_si_units::geometry::InverseVolume{per_m3: x}.into());
		assert!(uom::si::f64::SolidAngle::new::<uom::si::solid_angle::steradian>(x)
			== simple_si_units::geometry::SolidAngle{sr: x}.into());
		assert!(uom::si::f64::SpecificArea::new::<uom::si::specific_area::square_meter_per_kilogram>(x)
			== simple_si_units::mechanical::AreaPerMass{m2_per_kg: x}.into());
		assert!(uom::si::f64::SpecificVolume::new::<uom::si::specific_volume::cubic_meter_per_kilogram>(x)
			== simple_si_units::mechanical::VolumePerMass{m3_per_kg: x}.into());
	}

	#[test]
	fn into_uom_test_f32(){
		let x: f32 = 2.5;
		assert!(uom::si::f32::AmountOfSubstance::new::<uom::si::amount_of_substance::mole>(x)
			== simple_si_units::base::Amount{mol: x}.into());
		assert!(uom::si::f32::ElectricCurrent::new::<uom::si::electric_current::ampere>(x)
			== simple_si_units::base::Current{A: x}.into());
		assert!(uom::si::f32::Length::new::<uom::si::length::meter>(x)
			== simple_si_units::base::Distance{m: x}.into());
		assert!(uom::si::f32::LuminousIntensity::new::<uom::si::luminous_intensity::candela>(x)
			== simple_si_units::base::Luminosity{cd: x}.into());
		assert!(uom::si::f32::Mass::new::<uom::si::mass::kilogram>(x)
			== simple_si_units::base::Mass{kg: x}.into());
		assert!(uom::si::f32::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::kelvin>(x)
			== simple_si_units::base::Temperature{K: x}.into());
		assert!(uom::si::f32::Time::new::<uom::si::time::second>(x)
			== simple_si_units::base::Time{s: x}.into());
		assert!(uom::si::f32::CatalyticActivity::new::<uom::si::catalytic_activity::mole_per_second>(x)
			== simple_si_units::chemical::CatalyticActivity{molps: x}.into());
		assert!(uom::si::f32::MolarConcentration::new::<uom::si::molar_concentration::mole_per_cubic_meter>(x)
			== simple_si_units::chemical::Concentration{molpm3: x}.into());
		assert!(uom::si::f32::Molality::new::<uom::si::molality::mole_per_kilogram>(x)
			== simple_si_units::chemical::Molality{molpkg: x}.into());
		assert!(uom::si::f32::MolarMass::new::<uom::si::molar_mass::kilogram_per_mole>(x)
			== simple_si_units::chemical::MolarMass{kgpmol: x}.into());
		assert!(uom::si::f32::SpecificHeatCapacity::new::<uom::si::specific_heat_capacity::joule_per_kilogram_kelvin>(x)
			== simple_si_units::chemical::SpecificHeatCapacity{J_per_kgK: x}.into());
		assert!(uom::si::f32::Capacitance::new::<uom::si::capacitance::farad>(x)
			== simple_si_units::electromagnetic::Capacitance{F: x}.into());
		assert!(uom::si::f32::ElectricCharge::new::<uom::si::electric_charge::coulomb>(x)
			== simple_si_units::electromagnetic::Charge{C: x}.into());
		assert!(uom::si::f32::ElectricalConductance::new::<uom::si::electrical_conductance::siemens>(x)
			== simple_si_units::electromagnetic::Conductance{S: x}.into());
		assert!(uom::si::f32::Luminance::new::<uom::si::luminance::candela_per_square_meter>(x)
			== simple_si_units::electromagnetic::Illuminance{lux: x}.into());
		assert!(uom::si::f32::Inductance::new::<uom::si::inductance::henry>(x)
			== simple_si_units::electromagnetic::Inductance{H: x}.into());
		assert!(uom::si::f32::MagneticFlux::new::<uom::si::magnetic_flux::weber>(x)
			== simple_si_units::electromagnetic::MagneticFlux{Wb: x}.into());
		assert!(uom::si::f32::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::tesla>(x)
			== simple_si_units::electromagnetic::MagneticFluxDensity{T: x}.into());
		assert!(uom::si::f32::ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(x)
			== simple_si_units::electromagnetic::Resistance{Ohm: x}.into());
		assert!(uom::si::f32::ElectricPotential::new::<uom::si::electric_potential::volt>(x)
			== simple_si_units::electromagnetic::Voltage{V: x}.into());
		assert!(uom::si::f32::Angle::new::<uom::si::angle::radian>(x)
			== simple_si_units::geometry::Angle{rad: x}.into());
		assert!(uom::si::f32::Area::new::<uom::si::area::square_meter>(x)
			== simple_si_units::geometry::Area{m2: x}.into());
		assert!(uom::si::f32::Volume::new::<uom::si::volume::cubic_meter>(x)
			== simple_si_units::geometry::Volume{m3: x}.into());
		assert!(uom::si::f32::Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(x)
			== simple_si_units::mechanical::Acceleration{mps2: x}.into());
		assert!(uom::si::f32::AngularAcceleration::new::<uom::si::angular_acceleration::radian_per_second_squared>(x)
			== simple_si_units::mechanical::AngularAcceleration{radps2: x}.into());
		assert!(uom::si::f32::AngularVelocity::new::<uom::si::angular_velocity::radian_per_second>(x)
			== simple_si_units::mechanical::AngularVelocity{radps: x}.into());
		assert!(uom::si::f32::ArealMassDensity::new::<uom::si::areal_mass_density::kilogram_per_square_meter>(x)
			== simple_si_units::mechanical::AreaDensity{kgpm2: x}.into());
		assert!(uom::si::f32::MassDensity::new::<uom::si::mass_density::kilogram_per_cubic_meter>(x)
			== simple_si_units::mechanical::Density{kgpm3: x}.into());
		assert!(uom::si::f32::Energy::new::<uom::si::energy::joule>(x)
			== simple_si_units::mechanical::Energy{J: x}.into());
		assert!(uom::si::f32::Force::new::<uom::si::force::newton>(x)
			== simple_si_units::mechanical::Force{N: x}.into());
		assert!(uom::si::f32::Frequency::new::<uom::si::frequency::hertz>(x)
			== simple_si_units::mechanical::Frequency{Hz: x}.into());
		assert!(uom::si::f32::MomentOfInertia::new::<uom::si::moment_of_inertia::kilogram_square_meter>(x)
			== simple_si_units::mechanical::MomentOfInertia{kgm2: x}.into());
		assert!(uom::si::f32::Momentum::new::<uom::si::momentum::kilogram_meter_per_second>(x)
			== simple_si_units::mechanical::Momentum{kgmps: x}.into());
		assert!(uom::si::f32::Power::new::<uom::si::power::watt>(x)
			== simple_si_units::mechanical::Power{W: x}.into());
		assert!(uom::si::f32::Pressure::new::<uom::si::pressure::pascal>(x)
			== simple_si_units::mechanical::Pressure{Pa: x}.into());
		assert!(uom::si::f32::Torque::new::<uom::si::torque::newton_meter>(x)
			== simple_si_units::mechanical::Torque{Nm: x}.into());
		assert!(uom::si::f32::Velocity::new::<uom::si::velocity::meter_per_second>(x)
			== simple_si_units::mechanical::Velocity{mps: x}.into());
		assert!(uom::si::f32::Radioactivity::new::<uom::si::radioactivity::becquerel>(x)
			== simple_si_units::nuclear::Radioactivity{Bq: x}.into());
		assert!(uom::si::f32::LinearNumberDensity::new::<uom::si::linear_number_density::per_meter>(x)
			== simple_si_units::base::InverseDistance{per_m: x}.into());
		assert!(uom::si::f32::TemperatureCoefficient::new::<uom::si::temperature_coefficient::per_kelvin>(x)
			== simple_si_units::base::InverseTemperature{per_K: x}.into());
		assert!(uom::si::f32::MolarVolume::new::<uom::si::molar_volume::cubic_meter_per_mole>(x)
			== simple_si_units::chemical::MolarVolume{m3_per_mol: x}.into());
		assert!(uom::si::f32::ArealNumberDensity::new::<uom::si::areal_number_density::per_square_meter>(x)
			== simple_si_units::geometry::InverseArea{per_m2: x}.into());
		assert!(uom::si::f32::VolumetricNumberDensity::new::<uom::si::volumetric_number_density::per_cubic_meter>(x)
			== simple_si_units::geometry::InverseVolume{per_m3: x}.into());
		assert!(uom::si::f32::SolidAngle::new::<uom::si::solid_angle::steradian>(x)
			== simple_si_units::geometry::SolidAngle{sr: x}.into());
		assert!(uom::si::f32::SpecificArea::new::<uom::si::specific_area::square_meter_per_kilogram>(x)
			== simple_si_units::mechanical::AreaPerMass{m2_per_kg: x}.into());
		assert!(uom::si::f32::SpecificVolume::new::<uom::si::specific_volume::cubic_meter_per_kilogram>(x)
			== simple_si_units::mechanical::VolumePerMass{m3_per_kg: x}.into());
	}
	#[test]
	fn from_uom_test_f64(){
		let x: f64 = 2.5;
		assert!(simple_si_units::base::Amount::from(
				uom::si::f64::AmountOfSubstance::new::<uom::si::amount_of_substance::mole>(x)
			) == simple_si_units::base::Amount{mol: x});
		assert!(simple_si_units::base::Current::from(
				uom::si::f64::ElectricCurrent::new::<uom::si::electric_current::ampere>(x)
			) == simple_si_units::base::Current{A: x});
		assert!(simple_si_units::base::Distance::from(
				uom::si::f64::Length::new::<uom::si::length::meter>(x)
			) == simple_si_units::base::Distance{m: x});
		assert!(simple_si_units::base::Luminosity::from(
				uom::si::f64::LuminousIntensity::new::<uom::si::luminous_intensity::candela>(x)
			) == simple_si_units::base::Luminosity{cd: x});
		assert!(simple_si_units::base::Mass::from(
				uom::si::f64::Mass::new::<uom::si::mass::kilogram>(x)
			) == simple_si_units::base::Mass{kg: x});
		assert!(simple_si_units::base::Temperature::from(
				uom::si::f64::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::kelvin>(x)
			) == simple_si_units::base::Temperature{K: x});
		assert!(simple_si_units::base::Time::from(
				uom::si::f64::Time::new::<uom::si::time::second>(x)
			) == simple_si_units::base::Time{s: x});
		assert!(simple_si_units::chemical::CatalyticActivity::from(
				uom::si::f64::CatalyticActivity::new::<uom::si::catalytic_activity::mole_per_second>(x)
			) == simple_si_units::chemical::CatalyticActivity{molps: x});
		assert!(simple_si_units::chemical::Concentration::from(
				uom::si::f64::MolarConcentration::new::<uom::si::molar_concentration::mole_per_cubic_meter>(x)
			) == simple_si_units::chemical::Concentration{molpm3: x});
		assert!(simple_si_units::chemical::Molality::from(
				uom::si::f64::Molality::new::<uom::si::molality::mole_per_kilogram>(x)
			) == simple_si_units::chemical::Molality{molpkg: x});
		assert!(simple_si_units::chemical::MolarMass::from(
				uom::si::f64::MolarMass::new::<uom::si::molar_mass::kilogram_per_mole>(x)
			) == simple_si_units::chemical::MolarMass{kgpmol: x});
		assert!(simple_si_units::chemical::SpecificHeatCapacity::from(
				uom::si::f64::SpecificHeatCapacity::new::<uom::si::specific_heat_capacity::joule_per_kilogram_kelvin>(x)
			) == simple_si_units::chemical::SpecificHeatCapacity{J_per_kgK: x});
		assert!(simple_si_units::electromagnetic::Capacitance::from(
				uom::si::f64::Capacitance::new::<uom::si::capacitance::farad>(x)
			) == simple_si_units::electromagnetic::Capacitance{F: x});
		assert!(simple_si_units::electromagnetic::Charge::from(
				uom::si::f64::ElectricCharge::new::<uom::si::electric_charge::coulomb>(x)
			) == simple_si_units::electromagnetic::Charge{C: x});
		assert!(simple_si_units::electromagnetic::Conductance::from(
				uom::si::f64::ElectricalConductance::new::<uom::si::electrical_conductance::siemens>(x)
			) == simple_si_units::electromagnetic::Conductance{S: x});
		assert!(simple_si_units::electromagnetic::Illuminance::from(
				uom::si::f64::Luminance::new::<uom::si::luminance::candela_per_square_meter>(x)
			) == simple_si_units::electromagnetic::Illuminance{lux: x});
		assert!(simple_si_units::electromagnetic::Inductance::from(
				uom::si::f64::Inductance::new::<uom::si::inductance::henry>(x)
			) == simple_si_units::electromagnetic::Inductance{H: x});
		assert!(simple_si_units::electromagnetic::MagneticFlux::from(
				uom::si::f64::MagneticFlux::new::<uom::si::magnetic_flux::weber>(x)
			) == simple_si_units::electromagnetic::MagneticFlux{Wb: x});
		assert!(simple_si_units::electromagnetic::MagneticFluxDensity::from(
				uom::si::f64::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::tesla>(x)
			) == simple_si_units::electromagnetic::MagneticFluxDensity{T: x});
		assert!(simple_si_units::electromagnetic::Resistance::from(
				uom::si::f64::ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(x)
			) == simple_si_units::electromagnetic::Resistance{Ohm: x});
		assert!(simple_si_units::electromagnetic::Voltage::from(
				uom::si::f64::ElectricPotential::new::<uom::si::electric_potential::volt>(x)
			) == simple_si_units::electromagnetic::Voltage{V: x});
		assert!(simple_si_units::geometry::Angle::from(
				uom::si::f64::Angle::new::<uom::si::angle::radian>(x)
			) == simple_si_units::geometry::Angle{rad: x});
		assert!(simple_si_units::geometry::Area::from(
				uom::si::f64::Area::new::<uom::si::area::square_meter>(x)
			) == simple_si_units::geometry::Area{m2: x});
		assert!(simple_si_units::geometry::Volume::from(
				uom::si::f64::Volume::new::<uom::si::volume::cubic_meter>(x)
			) == simple_si_units::geometry::Volume{m3: x});
		assert!(simple_si_units::mechanical::Acceleration::from(
				uom::si::f64::Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(x)
			) == simple_si_units::mechanical::Acceleration{mps2: x});
		assert!(simple_si_units::mechanical::AngularAcceleration::from(
				uom::si::f64::AngularAcceleration::new::<uom::si::angular_acceleration::radian_per_second_squared>(x)
			) == simple_si_units::mechanical::AngularAcceleration{radps2: x});
		assert!(simple_si_units::mechanical::AngularVelocity::from(
				uom::si::f64::AngularVelocity::new::<uom::si::angular_velocity::radian_per_second>(x)
			) == simple_si_units::mechanical::AngularVelocity{radps: x});
		assert!(simple_si_units::mechanical::AreaDensity::from(
				uom::si::f64::ArealMassDensity::new::<uom::si::areal_mass_density::kilogram_per_square_meter>(x)
			) == simple_si_units::mechanical::AreaDensity{kgpm2: x});
		assert!(simple_si_units::mechanical::Density::from(
				uom::si::f64::MassDensity::new::<uom::si::mass_density::kilogram_per_cubic_meter>(x)
			) == simple_si_units::mechanical::Density{kgpm3: x});
		assert!(simple_si_units::mechanical::Energy::from(
				uom::si::f64::Energy::new::<uom::si::energy::joule>(x)
			) == simple_si_units::mechanical::Energy{J: x});
		assert!(simple_si_units::mechanical::Force::from(
				uom::si::f64::Force::new::<uom::si::force::newton>(x)
			) == simple_si_units::mechanical::Force{N: x});
		assert!(simple_si_units::mechanical::Frequency::from(
				uom::si::f64::Frequency::new::<uom::si::frequency::hertz>(x)
			) == simple_si_units::mechanical::Frequency{Hz: x});
		assert!(simple_si_units::mechanical::MomentOfInertia::from(
				uom::si::f64::MomentOfInertia::new::<uom::si::moment_of_inertia::kilogram_square_meter>(x)
			) == simple_si_units::mechanical::MomentOfInertia{kgm2: x});
		assert!(simple_si_units::mechanical::Momentum::from(
				uom::si::f64::Momentum::new::<uom::si::momentum::kilogram_meter_per_second>(x)
			) == simple_si_units::mechanical::Momentum{kgmps: x});
		assert!(simple_si_units::mechanical::Power::from(
				uom::si::f64::Power::new::<uom::si::power::watt>(x)
			) == simple_si_units::mechanical::Power{W: x});
		assert!(simple_si_units::mechanical::Pressure::from(
				uom::si::f64::Pressure::new::<uom::si::pressure::pascal>(x)
			) == simple_si_units::mechanical::Pressure{Pa: x});
		assert!(simple_si_units::mechanical::Torque::from(
				uom::si::f64::Torque::new::<uom::si::torque::newton_meter>(x)
			) == simple_si_units::mechanical::Torque{Nm: x});
		assert!(simple_si_units::mechanical::Velocity::from(
				uom::si::f64::Velocity::new::<uom::si::velocity::meter_per_second>(x)
			) == simple_si_units::mechanical::Velocity{mps: x});
		assert!(simple_si_units::nuclear::Radioactivity::from(
				uom::si::f64::Radioactivity::new::<uom::si::radioactivity::becquerel>(x)
			) == simple_si_units::nuclear::Radioactivity{Bq: x});
		assert!(simple_si_units::base::InverseDistance::from(
			uom::si::f64::LinearNumberDensity::new::<uom::si::linear_number_density::per_meter>(x)
			) == simple_si_units::base::InverseDistance{per_m: x});
		assert!(simple_si_units::base::InverseTemperature::from(
			uom::si::f64::TemperatureCoefficient::new::<uom::si::temperature_coefficient::per_kelvin>(x)
			) == simple_si_units::base::InverseTemperature{per_K: x});
		assert!(simple_si_units::chemical::MolarVolume::from(
			uom::si::f64::MolarVolume::new::<uom::si::molar_volume::cubic_meter_per_mole>(x)
			) == simple_si_units::chemical::MolarVolume{m3_per_mol: x});
		assert!(simple_si_units::geometry::InverseArea::from(
			uom::si::f64::ArealNumberDensity::new::<uom::si::areal_number_density::per_square_meter>(x)
			) == simple_si_units::geometry::InverseArea{per_m2: x});
		assert!(simple_si_units::geometry::InverseVolume::from(
			uom::si::f64::VolumetricNumberDensity::new::<uom::si::volumetric_number_density::per_cubic_meter>(x)
			) == simple_si_units::geometry::InverseVolume{per_m3: x});
		assert!(simple_si_units::geometry::SolidAngle::from(
			uom::si::f64::SolidAngle::new::<uom::si::solid_angle::steradian>(x)
			) == simple_si_units::geometry::SolidAngle{sr: x});
		assert!(simple_si_units::mechanical::AreaPerMass::from(
			uom::si::f64::SpecificArea::new::<uom::si::specific_area::square_meter_per_kilogram>(x)
			) == simple_si_units::mechanical::AreaPerMass{m2_per_kg: x});
		assert!(simple_si_units::mechanical::VolumePerMass::from(
			uom::si::f64::SpecificVolume::new::<uom::si::specific_volume::cubic_meter_per_kilogram>(x)
			) == simple_si_units::mechanical::VolumePerMass{m3_per_kg: x});
	}

	#[test]
	fn from_uom_test_f32(){
		let x: f32 = 2.5;
		assert!(simple_si_units::base::Amount::from(
			uom::si::f32::AmountOfSubstance::new::<uom::si::amount_of_substance::mole>(x)
		) == simple_si_units::base::Amount{mol: x});
		assert!(simple_si_units::base::Current::from(
			uom::si::f32::ElectricCurrent::new::<uom::si::electric_current::ampere>(x)
		) == simple_si_units::base::Current{A: x});
		assert!(simple_si_units::base::Distance::from(
			uom::si::f32::Length::new::<uom::si::length::meter>(x)
		) == simple_si_units::base::Distance{m: x});
		assert!(simple_si_units::base::Luminosity::from(
			uom::si::f32::LuminousIntensity::new::<uom::si::luminous_intensity::candela>(x)
		) == simple_si_units::base::Luminosity{cd: x});
		assert!(simple_si_units::base::Mass::from(
			uom::si::f32::Mass::new::<uom::si::mass::kilogram>(x)
		) == simple_si_units::base::Mass{kg: x});
		assert!(simple_si_units::base::Temperature::from(
			uom::si::f32::ThermodynamicTemperature::new::<uom::si::thermodynamic_temperature::kelvin>(x)
		) == simple_si_units::base::Temperature{K: x});
		assert!(simple_si_units::base::Time::from(
			uom::si::f32::Time::new::<uom::si::time::second>(x)
		) == simple_si_units::base::Time{s: x});
		assert!(simple_si_units::chemical::CatalyticActivity::from(
			uom::si::f32::CatalyticActivity::new::<uom::si::catalytic_activity::mole_per_second>(x)
		) == simple_si_units::chemical::CatalyticActivity{molps: x});
		assert!(simple_si_units::chemical::Molality::from(
			uom::si::f32::Molality::new::<uom::si::molality::mole_per_kilogram>(x)
		) == simple_si_units::chemical::Molality{molpkg: x});
		assert!(simple_si_units::chemical::MolarMass::from(
			uom::si::f32::MolarMass::new::<uom::si::molar_mass::kilogram_per_mole>(x)
		) == simple_si_units::chemical::MolarMass{kgpmol: x});
		assert!(simple_si_units::chemical::SpecificHeatCapacity::from(
			uom::si::f32::SpecificHeatCapacity::new::<uom::si::specific_heat_capacity::joule_per_kilogram_kelvin>(x)
		) == simple_si_units::chemical::SpecificHeatCapacity{J_per_kgK: x});
		assert!(simple_si_units::chemical::Concentration::from(
			uom::si::f32::MolarConcentration::new::<uom::si::molar_concentration::mole_per_cubic_meter>(x)
		) == simple_si_units::chemical::Concentration{molpm3: x});
		assert!(simple_si_units::electromagnetic::Capacitance::from(
			uom::si::f32::Capacitance::new::<uom::si::capacitance::farad>(x)
		) == simple_si_units::electromagnetic::Capacitance{F: x});
		assert!(simple_si_units::electromagnetic::Charge::from(
			uom::si::f32::ElectricCharge::new::<uom::si::electric_charge::coulomb>(x)
		) == simple_si_units::electromagnetic::Charge{C: x});
		assert!(simple_si_units::electromagnetic::Conductance::from(
			uom::si::f32::ElectricalConductance::new::<uom::si::electrical_conductance::siemens>(x)
		) == simple_si_units::electromagnetic::Conductance{S: x});
		assert!(simple_si_units::electromagnetic::Illuminance::from(
			uom::si::f32::Luminance::new::<uom::si::luminance::candela_per_square_meter>(x)
		) == simple_si_units::electromagnetic::Illuminance{lux: x});
		assert!(simple_si_units::electromagnetic::Inductance::from(
			uom::si::f32::Inductance::new::<uom::si::inductance::henry>(x)
		) == simple_si_units::electromagnetic::Inductance{H: x});
		assert!(simple_si_units::electromagnetic::MagneticFlux::from(
			uom::si::f32::MagneticFlux::new::<uom::si::magnetic_flux::weber>(x)
		) == simple_si_units::electromagnetic::MagneticFlux{Wb: x});
		assert!(simple_si_units::electromagnetic::MagneticFluxDensity::from(
			uom::si::f32::MagneticFluxDensity::new::<uom::si::magnetic_flux_density::tesla>(x)
		) == simple_si_units::electromagnetic::MagneticFluxDensity{T: x});
		assert!(simple_si_units::electromagnetic::Resistance::from(
			uom::si::f32::ElectricalResistance::new::<uom::si::electrical_resistance::ohm>(x)
		) == simple_si_units::electromagnetic::Resistance{Ohm: x});
		assert!(simple_si_units::electromagnetic::Voltage::from(
			uom::si::f32::ElectricPotential::new::<uom::si::electric_potential::volt>(x)
		) == simple_si_units::electromagnetic::Voltage{V: x});
		assert!(simple_si_units::geometry::Angle::from(
			uom::si::f32::Angle::new::<uom::si::angle::radian>(x)
		) == simple_si_units::geometry::Angle{rad: x});
		assert!(simple_si_units::geometry::Area::from(
			uom::si::f32::Area::new::<uom::si::area::square_meter>(x)
		) == simple_si_units::geometry::Area{m2: x});
		assert!(simple_si_units::geometry::Volume::from(
			uom::si::f32::Volume::new::<uom::si::volume::cubic_meter>(x)
		) == simple_si_units::geometry::Volume{m3: x});
		assert!(simple_si_units::mechanical::Acceleration::from(
			uom::si::f32::Acceleration::new::<uom::si::acceleration::meter_per_second_squared>(x)
		) == simple_si_units::mechanical::Acceleration{mps2: x});
		assert!(simple_si_units::mechanical::AngularAcceleration::from(
			uom::si::f32::AngularAcceleration::new::<uom::si::angular_acceleration::radian_per_second_squared>(x)
		) == simple_si_units::mechanical::AngularAcceleration{radps2: x});
		assert!(simple_si_units::mechanical::AngularVelocity::from(
			uom::si::f32::AngularVelocity::new::<uom::si::angular_velocity::radian_per_second>(x)
		) == simple_si_units::mechanical::AngularVelocity{radps: x});
		assert!(simple_si_units::mechanical::AreaDensity::from(
			uom::si::f32::ArealMassDensity::new::<uom::si::areal_mass_density::kilogram_per_square_meter>(x)
		) == simple_si_units::mechanical::AreaDensity{kgpm2: x});
		assert!(simple_si_units::mechanical::Density::from(
			uom::si::f32::MassDensity::new::<uom::si::mass_density::kilogram_per_cubic_meter>(x)
		) == simple_si_units::mechanical::Density{kgpm3: x});
		assert!(simple_si_units::mechanical::Energy::from(
			uom::si::f32::Energy::new::<uom::si::energy::joule>(x)
		) == simple_si_units::mechanical::Energy{J: x});
		assert!(simple_si_units::mechanical::Force::from(
			uom::si::f32::Force::new::<uom::si::force::newton>(x)
		) == simple_si_units::mechanical::Force{N: x});
		assert!(simple_si_units::mechanical::Frequency::from(
			uom::si::f32::Frequency::new::<uom::si::frequency::hertz>(x)
		) == simple_si_units::mechanical::Frequency{Hz: x});
		assert!(simple_si_units::mechanical::MomentOfInertia::from(
			uom::si::f32::MomentOfInertia::new::<uom::si::moment_of_inertia::kilogram_square_meter>(x)
		) == simple_si_units::mechanical::MomentOfInertia{kgm2: x});
		assert!(simple_si_units::mechanical::Momentum::from(
			uom::si::f32::Momentum::new::<uom::si::momentum::kilogram_meter_per_second>(x)
		) == simple_si_units::mechanical::Momentum{kgmps: x});
		assert!(simple_si_units::mechanical::Power::from(
			uom::si::f32::Power::new::<uom::si::power::watt>(x)
		) == simple_si_units::mechanical::Power{W: x});
		assert!(simple_si_units::mechanical::Pressure::from(
			uom::si::f32::Pressure::new::<uom::si::pressure::pascal>(x)
		) == simple_si_units::mechanical::Pressure{Pa: x});
		assert!(simple_si_units::mechanical::Torque::from(
			uom::si::f32::Torque::new::<uom::si::torque::newton_meter>(x)
		) == simple_si_units::mechanical::Torque{Nm: x});
		assert!(simple_si_units::mechanical::Velocity::from(
			uom::si::f32::Velocity::new::<uom::si::velocity::meter_per_second>(x)
		) == simple_si_units::mechanical::Velocity{mps: x});
		assert!(simple_si_units::nuclear::Radioactivity::from(
			uom::si::f32::Radioactivity::new::<uom::si::radioactivity::becquerel>(x)
		) == simple_si_units::nuclear::Radioactivity{Bq: x});
		assert!(simple_si_units::base::InverseDistance::from(
			uom::si::f32::LinearNumberDensity::new::<uom::si::linear_number_density::per_meter>(x)
		) == simple_si_units::base::InverseDistance{per_m: x});
		assert!(simple_si_units::base::InverseTemperature::from(
			uom::si::f32::TemperatureCoefficient::new::<uom::si::temperature_coefficient::per_kelvin>(x)
		) == simple_si_units::base::InverseTemperature{per_K: x});
		assert!(simple_si_units::chemical::MolarVolume::from(
			uom::si::f32::MolarVolume::new::<uom::si::molar_volume::cubic_meter_per_mole>(x)
		) == simple_si_units::chemical::MolarVolume{m3_per_mol: x});
		assert!(simple_si_units::geometry::InverseArea::from(
			uom::si::f32::ArealNumberDensity::new::<uom::si::areal_number_density::per_square_meter>(x)
		) == simple_si_units::geometry::InverseArea{per_m2: x});
		assert!(simple_si_units::geometry::InverseVolume::from(
			uom::si::f32::VolumetricNumberDensity::new::<uom::si::volumetric_number_density::per_cubic_meter>(x)
		) == simple_si_units::geometry::InverseVolume{per_m3: x});
		assert!(simple_si_units::geometry::SolidAngle::from(
			uom::si::f32::SolidAngle::new::<uom::si::solid_angle::steradian>(x)
		) == simple_si_units::geometry::SolidAngle{sr: x});
		assert!(simple_si_units::mechanical::AreaPerMass::from(
			uom::si::f32::SpecificArea::new::<uom::si::specific_area::square_meter_per_kilogram>(x)
		) == simple_si_units::mechanical::AreaPerMass{m2_per_kg: x});
		assert!(simple_si_units::mechanical::VolumePerMass::from(
			uom::si::f32::SpecificVolume::new::<uom::si::specific_volume::cubic_meter_per_kilogram>(x)
		) == simple_si_units::mechanical::VolumePerMass{m3_per_kg: x});
	}

}