# Safe-coverage taxonomy

This taxonomy separates mathematical capabilities before safe-coverage counts are interpreted. In particular, nonlinear equations, nonlinear least squares, linear least squares, and linear programming are not nonlinear objective minimization.

## Category counts

- `approximation_and_fitting_support`: 30
- `linear_least_squares`: 8
- `linear_programming`: 2
- `nonlinear_least_squares`: 6
- `nonlinear_systems_and_jacobian_checking`: 8
- `other`: 1412
- `polynomial_approximation_and_fitting`: 6
- `polynomial_representation_and_evaluation`: 8
- `polynomial_roots`: 4
- `scalar_roots`: 2
- `spline_representation_and_construction`: 23
- `stateful_spline_fitting`: 8

## Objective minimization and nonlinear programming

- Retained suitable drivers: 0.
- Disposition: **no suitable retained SLATEC high-level driver**.
- `SPLP`/`DSPLP` remain correctly classified as linear programming; the other listed neighboring capabilities are reported separately and are excluded from the nonlinear-objective count.
