# Purpose

Usage: REAL L2, L3, M2, M3, L1MIN, L1MAX, THRCOF(NDIM) INTEGER NDIM, IER CALL RC3JJ (L2, L3, M2, M3, L1MIN, L1MAX, THRCOF, NDIM, IER) Although conventionally the parameters of the vector addition coefficients satisfy certain restrictions, such as being integers or integers plus 1/2, the restrictions imposed on input to this subroutine are somewhat weaker. See, for example, Section 27.9 of Abramowitz and Stegun or Appendix C of Volume II of A. Messiah. The restrictions imposed by this subroutine are 1. L2 .GE. ABS(M2) and L3 .GE. ABS(M3);

# Description

This canonical unsafe binding exposes original SLATEC routine `RC3JJ`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [RC3JJ](https://www.netlib.org/slatec/src/rc3jj.f).

# Arguments

## 1. `L2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. IN      Parameter in 3j symbol. integer. must be integers; L3),ABS(M2+M3)). L3),ABS(M2+M3)). If the conventional restrictions are satisfied, then these If the conventional restrictions are satisfied, then these restrictions are met. restrictions are met. The user should be cautious in using input parameters that do The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the not satisfy the conventional restrictions. For example, the the subroutine produces values of the subroutine produces values of f(L1) = ( L1  2.5  5.8) f(L1) = ( L1  2.5  5.8) (-0.3 1.5 -1.2) (-0.3 1.5 -1.2) for L1=3.3,4.3,...,8.3 but none of the symmetry properties of the 3j for L1=3.3,4.3,...,8.3 but none of the symmetry properties of the 3j symbol, set forth on page 1056 of Messiah, is satisfied. symbol, set forth on page 1056 of Messiah, is satisfied. The subroutine generates f(L1MIN), f(L1MIN+1), ..., f(L1MAX) The subroutine generates f(L1MIN), f(L1MIN+1), ..., f(L1MAX) IN      Parameter in 3j symbol. integer. must be integers; L3),ABS(M2+M3)). L3),ABS(M2+M3)). If the conventional restrictions are satisfied, then these If the conventional restrictions are satisfied, then these restrictions are met. restrictions are met. The user should be cautious in using input parameters that do The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the not satisfy the conventional restrictions. For example, the the subroutine produces values of the subroutine produces values of f(L1) = ( L1  2.5  5.8) f(L1) = ( L1  2.5  5.8) (-0.3 1.5 -1.2) (-0.3 1.5 -1.2) for L1=3.3,4.3,...,8.3 but none of the symmetry properties of the 3j for L1=3.3,4.3,...,8.3 but none of the symmetry properties of the 3j symbol, set forth on page 1056 of Messiah, is satisfied. symbol, set forth on page 1056 of Messiah, is satisfied. The subroutine generates f(L1MIN), f(L1MIN+1), ..., f(L1MAX) The subroutine generates f(L1MIN), f(L1MIN+1), ..., f(L1MAX) not applicable or not stated by selected source not a workspace argument

## 2. `L3`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. IN      Parameter in 3j symbol. integer. must be integers; L3),ABS(M2+M3)). If the conventional restrictions are satisfied, then these restrictions are met. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of f(L1) = ( L1  2.5  5.8) (-0.3 1.5 -1.2) for L1=3.3,4.3,...,8.3 but none of the symmetry properties of the 3j symbol, set forth on page 1056 of Messiah, is satisfied. The subroutine generates f(L1MIN), f(L1MIN+1), ..., f(L1MAX) IN      Parameter in 3j symbol. integer. must be integers; L3),ABS(M2+M3)). If the conventional restrictions are satisfied, then these restrictions are met. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of f(L1) = ( L1  2.5  5.8) (-0.3 1.5 -1.2) for L1=3.3,4.3,...,8.3 but none of the symmetry properties of the 3j symbol, set forth on page 1056 of Messiah, is satisfied. The subroutine generates f(L1MIN), f(L1MIN+1), ..., f(L1MAX) not applicable or not stated by selected source not a workspace argument

## 3. `M2`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. M2 M3) for all allowed values of L1, the other parameters being held fixed. IN      Parameter in 3j symbol. integer. must be integers; M2 M3) for all allowed values of L1, the other parameters being held fixed. IN      Parameter in 3j symbol. integer. must be integers; not applicable or not stated by selected source not a workspace argument

## 4. `M3`

input `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. M2 M3) for all allowed values of L1, the other parameters being held fixed. IN      Parameter in 3j symbol. integer. must be integers; M2 M3) for all allowed values of L1, the other parameters being held fixed. IN      Parameter in 3j symbol. integer. must be integers; not applicable or not stated by selected source not a workspace argument

## 5. `L1MIN`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. OUT  Smallest allowable L1 in 3j symbol. 1), I=1,2,...,L1MAX+L1MIN+1. L3),ABS(M2+M3)). If the conventional restrictions are satisfied, then these restrictions are met. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of f(L1) = ( L1  2.5  5.8) (-0.3 1.5 -1.2) for L1=3.3,4.3,...,8.3 but none of the symmetry properties of the 3j symbol, set forth on page 1056 of Messiah, is satisfied. The subroutine generates f(L1MIN), f(L1MIN+1), ..., f(L1MAX) are defined above. The sequence f(L1) is generated by a three-term recurrence algorithm with scaling to control overflow. Both backward and forward recurrence are used to maintain numerical stability. The two recurrence sequences are matched at an interior point and are normalized from the unitary property of 3j coefficients and Wigner's phase convention. The algorithm is suited to applications in which large quantum numbers arise, such as in molecular dynamics. not stated by selected source not applicable or not stated by selected source not a workspace argument

## 6. `L1MAX`

input-output `scalar` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and scalar. OUT  Largest allowable L1 in 3j symbol. L1MIN not an integer. L1MIN+1. L1MIN must be a non-negative integer, where L3),ABS(M2+M3)). If the conventional restrictions are satisfied, then these restrictions are met. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of f(L1) = ( L1  2.5  5.8) (-0.3 1.5 -1.2) for L1=3.3,4.3,...,8.3 but none of the symmetry properties of the 3j symbol, set forth on page 1056 of Messiah, is satisfied. The subroutine generates f(L1MIN), f(L1MIN+1), ..., f(L1MAX) are defined above. The sequence f(L1) is generated by a three-term recurrence algorithm with scaling to control overflow. Both backward and forward recurrence are used to maintain numerical stability. The two recurrence sequences are matched at an interior point and are normalized from the unitary property of 3j coefficients and Wigner's phase convention. The algorithm is suited to applications in which large quantum numbers arise, such as in molecular dynamics. OUT  Largest allowable L1 in 3j symbol. L1MIN not an integer. L1MIN+1. L1MIN must be a non-negative integer, where L3),ABS(M2+M3)). If the conventional restrictions are satisfied, then these restrictions are met. The user should be cautious in using input parameters that do not satisfy the conventional restrictions. For example, the the subroutine produces values of f(L1) = ( L1  2.5  5.8) (-0.3 1.5 -1.2) for L1=3.3,4.3,...,8.3 but none of the symmetry properties of the 3j symbol, set forth on page 1056 of Messiah, is satisfied. The subroutine generates f(L1MIN), f(L1MIN+1), ..., f(L1MAX) are defined above. The sequence f(L1) is generated by a three-term recurrence algorithm with scaling to control overflow. Both backward and forward recurrence are used to maintain numerical stability. The two recurrence sequences are matched at an interior point and are normalized from the unitary property of 3j coefficients and Wigner's phase convention. The algorithm is suited to applications in which large quantum numbers arise, such as in molecular dynamics. not applicable or not stated by selected source not a workspace argument

## 7. `THRCOF`

input-output `array` argument; Fortran declaration `REAL`, Rust ABI type `*mut f32`, and rank 1; dimensions (NDIM). OUT Set of 3j coefficients generated by evaluating the 3j symbol for all allowed values of L1.  THRCOF(I) not stated by selected source not applicable or not stated by selected source not a workspace argument

## 8. `NDIM`

input `scalar` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. IN    Declared length of THRCOF in calling program. L1MIN+1. IN    Declared length of THRCOF in calling program. L1MIN+1. not applicable or not stated by selected source not a workspace argument

## 9. `IER`

status-output `status` argument; Fortran declaration `INTEGER`, Rust ABI type `*mut crate::FortranInteger`, and scalar. OUT    Error flag. 0 No errors. 1 Either L2.LT.ABS(M2) or L3.LT.ABS(M3). integer. L1MIN not an integer. 4 L1MAX less than L1MIN. L1MIN+1. not stated by selected source not applicable or not stated by selected source not a workspace argument

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

This interface has no callback argument.

# Status and error values

The selected source has no separate status-code section. Status output arguments, if present, are identified in the argument contract; legacy SLATEC error-runtime behavior remains part of the native provider contract.

# Workspace and array requirements

- `L2`: not a workspace argument
- `L3`: not a workspace argument
- `M2`: not a workspace argument
- `M3`: not a workspace argument
- `L1MIN`: not a workspace argument
- `L1MAX`: not a workspace argument
- `THRCOF`: not a workspace argument
- `NDIM`: not a workspace argument
- `IER`: not a workspace argument

# ABI notes

- Canonical Rust path: `slatec_sys::special::rc3jj`
- Original SLATEC routine: `RC3JJ`
- Native symbol: `rc3jj_`
- ABI fingerprint: `subroutine:void(mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32,mut_f32_ptr_rank1,mut_i32,mut_i32)`
- Exact Netlib source file: [RC3JJ](https://www.netlib.org/slatec/src/rc3jj.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
