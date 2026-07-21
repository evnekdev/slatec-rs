# Purpose

Usage: REAL L2, L3, L4, L5, L6, L1MIN, L1MAX, SIXCOF(NDIM) INTEGER NDIM, IER CALL RC6J(L2, L3, L4, L5, L6, L1MIN, L1MAX, SIXCOF, NDIM, IER) The definition and properties of 6j symbols can be found, for example, in Appendix C of Volume II of A. Messiah. Although the parameters of the vector addition coefficients satisfy certain conventional restrictions, the restriction that they be non-negative integers or non-negative integers plus 1/2 is not imposed on input to this subroutine. The restrictions imposed are

# Description

This canonical unsafe binding exposes original SLATEC routine `RC6J`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RC6J](https://www.netlib.org/slatec/src/rc6j.f).

# Arguments

## 1. `L2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. IN      Parameter in 6j symbol. must be integers; must be integers; L4).LE.L6.LE.L2+L4 must be satisfied; L3),ABS(L5-L6)). L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the not satisfy the conventional restrictions. For example, the the subroutine produces values of the subroutine produces values of h(L1) = { L1 2/3  1 } h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) IN      Parameter in 6j symbol. must be integers; must be integers; L4).LE.L6.LE.L2+L4 must be satisfied; L3),ABS(L5-L6)). L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the not satisfy the conventional restrictions. For example, the the subroutine produces values of the subroutine produces values of h(L1) = { L1 2/3  1 } h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) not applicable or not stated by selected source not a workspace argument

## 2. `L3`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. IN      Parameter in 6j symbol. must be integers; L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) IN      Parameter in 6j symbol. must be integers; L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) not applicable or not stated by selected source not a workspace argument

## 3. `L4`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. L5 L6} for all allowed values of L1, the other parameters being held fixed. IN      Parameter in 6j symbol. must be integers; L5).LE.L3.LE.L4+L5 must be satisfied; L5 L6} for all allowed values of L1, the other parameters being held fixed. IN      Parameter in 6j symbol. must be integers; L5).LE.L3.LE.L4+L5 must be satisfied; not applicable or not stated by selected source not a workspace argument

## 4. `L5`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. IN      Parameter in 6j symbol. must be integers; L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) IN      Parameter in 6j symbol. must be integers; L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) not applicable or not stated by selected source not a workspace argument

## 5. `L6`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. IN      Parameter in 6j symbol. must be integers; must be integers; L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) IN      Parameter in 6j symbol. must be integers; must be integers; L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) not applicable or not stated by selected source not a workspace argument

## 6. `L1MIN`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. OUT  Smallest allowable L1 in 6j symbol. 1), I=1,2,...,L1MAX-L1MIN+1. L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) are defined above. The sequence h(L1) is generated by a three-term recurrence algorithm with scaling to control overflow. Both backward and forward recurrence are used to maintain numerical stability. The two recurrence sequences are matched at an interior point and are normalized from the unitary property of 6j coefficients and Wigner's phase convention. The algorithm is suited to applications in which large quantum numbers arise, such as in molecular dynamics. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 7. `L1MAX`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. OUT  Largest allowable L1 in 6j symbol. L1MIN not an integer. L1MIN+1. L1MIN must be a non-negative integer, where L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) are defined above. The sequence h(L1) is generated by a three-term recurrence algorithm with scaling to control overflow. Both backward and forward recurrence are used to maintain numerical stability. The two recurrence sequences are matched at an interior point and are normalized from the unitary property of 6j coefficients and Wigner's phase convention. The algorithm is suited to applications in which large quantum numbers arise, such as in molecular dynamics. OUT  Largest allowable L1 in 6j symbol. L1MIN not an integer. L1MIN+1. L1MIN must be a non-negative integer, where L3),ABS(L5-L6)). If all the conventional restrictions are satisfied, then these restrictions are met. Conversely, if input to this subroutine meets all of these restrictions and the conventional restriction stated above, then all the conventional restrictions are satisfied. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of h(L1) = { L1 2/3  1 } {2/3 2/3 2/3} for L1=1/3 and 4/3 but none of the symmetry properties of the 6j symbol, set forth on pages 1063 and 1064 of Messiah, is satisfied. The subroutine generates h(L1MIN), h(L1MIN+1), ..., h(L1MAX) are defined above. The sequence h(L1) is generated by a three-term recurrence algorithm with scaling to control overflow. Both backward and forward recurrence are used to maintain numerical stability. The two recurrence sequences are matched at an interior point and are normalized from the unitary property of 6j coefficients and Wigner's phase convention. The algorithm is suited to applications in which large quantum numbers arise, such as in molecular dynamics. not applicable or not stated by selected source not a workspace argument

## 8. `SIXCOF`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (NDIM). OUT Set of 6j coefficients generated by evaluating the 6j symbol for all allowed values of L1.  SIXCOF(I) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 9. `NDIM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN    Declared length of SIXCOF in calling program. L1MIN+1. IN    Declared length of SIXCOF in calling program. L1MIN+1. not applicable or not stated by selected source not a workspace argument

## 10. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT    Error flag. 0 No errors. 1 L2+L3+L5+L6 or L4+L2+L6 not an integer. 2 L4, L2, L6 triangular condition not satisfied. 3 L4, L5, L3 triangular condition not satisfied. L1MIN not an integer. 5 L1MAX less than L1MIN. L1MIN+1. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `L2`: not a workspace argument
- `L3`: not a workspace argument
- `L4`: not a workspace argument
- `L5`: not a workspace argument
- `L6`: not a workspace argument
- `L1MIN`: not a workspace argument
- `L1MAX`: not a workspace argument
- `SIXCOF`: not a workspace argument
- `NDIM`: not a workspace argument
- `IER`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::rc6j`
- Original SLATEC routine: `RC6J`
- Native symbol: `rc6j_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [RC6J](https://www.netlib.org/slatec/src/rc6j.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
