# Purpose

I. ABSTRACT ....................................................... The primary function of DDRIV3 is to solve N ordinary differential equations of the form dY(I)/dT = F(Y(I),T), given the initial conditions Y(I) = YI. The program has options to allow the solution of both stiff and non-stiff differential equations. In addition, DDRIV3 may be used to solve: 1. The initial value problem, A*dY(I)/dT = F(Y(I),T), where A is a non-singular matrix depending on Y and T. 2. The hybrid differential/algebraic initial value problem, A*dY(I)/dT = F(Y(I),T), where A is a vector (whose values may depend upon Y and T) some of whose components will be zero corresponding to those equations which are algebraic rather than differential. DDRIV3 is to be called once for each output point of T.

# Description

This canonical unsafe binding exposes original SLATEC routine `DDRIV3`. Its documented behavior is taken from the selected, source-hash-verified provider prologue; the exact implementation source is [DDRIV3](https://www.netlib.org/slatec/src/ddriv3.f).

# Arguments

## `N`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) The number of dependent functions whose solution is desired. N must not be altered during a problem.

## `T`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

The independent variable. On input for the first call, T is the initial point. On output, T is the point at which the solution is given.

## `Y`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

The vector of dependent variables. Y is used as input on the first call, to set the initial values. On output, Y is the computed solution vector. This array Y is passed in the call sequence of the user-provided routines F, JACOBN, FA, USERS, and G. Thus parameters required by those routines can be stored in this array in components N+1 and above. (Note: Changes by the user to the first N components of this array will take effect only after a restart, i.

## `F`

**Direction:** `callback`. **Fortran type:** `UNKNOWN`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

A subroutine supplied by the user. The name must be declared EXTERNAL in the user's calling program. This subroutine is of the form: SUBROUTINE F (N, T, Y, YDOT) DOUBLE PRECISION Y(*), YDOT(*) YDOT(1) =. YDOT(N) =. END (Sample) This computes YDOT = F(Y,T), the right hand side of the differential equations. Here Y is a vector of length at least N. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `NSTATE`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An integer describing the status of integration. The meaning of NSTATE is as follows: 1 (Input) Means the first call to the routine. This value must be set by the user. On all subsequent calls the value of NSTATE should be tested by the user, but must not be altered. (As a convenience to the user who may wish to put out the initial conditions, DDRIV3 can be called with NSTATE=1, and.

## `TOUT`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

T. In this case the program will return with NSTATE unchanged, i. e. , NSTATE=1. ) 2 (Output) Means a successful integration. If a normal continuation is desired (i.

## `NTASK`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) An index specifying the manner of returning the solution, according to the following: 1 Means DDRIV3 will integrate past TOUT and interpolate the solution. This is the most efficient mode. 2 Means DDRIV3 will return the solution after each internal integration step, or at TOUT, whichever comes first. In the latter case, the program integrates exactly to TOUT. 3 Means DDRIV3 will adjust its internal step to reach TOUT exactly (useful if a singularity exists beyond TOUT. ).

## `NROOT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) The number of equations whose roots are desired. If NROOT is zero, the root search is not active. This option is useful for obtaining output at points which are not known in advance, but depend upon the solution, e. g. , when some solution component takes on a specified value. The root search is carried out using the user-written function G (see description of G below.

## `EPS`

**Direction:** `input-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

On input, the requested relative accuracy in all solution components. EPS = 0 is allowed. On output, the adjusted relative accuracy if the input value was too small. The value of EPS should be set as large as is reasonable, because the amount of work done by DDRIV3 increases as EPS decreases.

## `EWT`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

(Input) Problem zero, i. e. , the smallest, nonzero, physically meaningful value for the solution. (Array, possibly of length one. See following description of is ignored. 2 Means YWT(I) = ABS(Y(I)), (Relative error control) 3 Means YWT(I) = MAX(ABS(Y(I)), EWT(1)).

## `IERROR`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

Setting EWT smaller than necessary can adversely affect the running time. (Input) Error control indicator. A value of 3 is suggested for most problems. Other choices and detailed explanations of EWT and IERROR are given below for those who may need extra flexibility. These last three input quantities EPS, EWT and IERROR control the accuracy of the computed solution. EWT and are used internally to compute an array YWT.

## `MINT`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) The integration method indicator. 1 Means the Adams methods, and is used for non-stiff problems. 2 Means the stiff methods of Gear (i. e. , the backward differentiation formulas), and is used for stiff problems. 3 Means the program dynamically selects the Adams methods when the problem is non-stiff and the Gear methods when the problem is stiff.

## `MITER`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) The iteration method indicator. 0 Means functional iteration. This value is suggested for non-stiff problems. 1 Means chord method with analytic Jacobian. In this case, the user supplies subroutine JACOBN (see description below). 2 Means chord method with Jacobian calculated internally by finite differences.

## `IMPL`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) The implicit method indicator. 0 Means solving dY(I)/dT = F(Y(I),T). 1 Means solving A*dY(I)/dT = F(Y(I),T), non- singular A (see description of FA below. ) Only MINT = 1 or 2, and MITER = 1, 2, 3, 4, or 5 are allowed for this option. 2,3 Means solving certain systems of hybrid differential/algebraic equations (see description of FA below. ) Only MINT = 2 and 0.

## `ML`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) The lower half-bandwidth in the case of a banded A or Jacobian matrix. (I. e. , maximum(R-C) for nonzero A(R,C). ).

## `MU`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) The upper half-bandwidth in the case of a banded A or Jacobian matrix. (I. e. , maximum(C-R). ) The left hand side of the I-th equation is a linear combination of dY(I-ML)/dT, dY(I-ML+1)/dT,. , dY(I)/dT,.

## `MXORD`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) The maximum order desired. This is. LE. 12 for the Adams methods and. 5 for the Gear methods. Normal value is 12 and 5, respectively.

## `HMAX`

**Direction:** `input`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** scalar.

(Input) The maximum magnitude of the step size that will be used for the problem. This is useful for ensuring that important details are not missed. If this is not the case, a large value, such as the interval length, is suggested.

## `WORK`

**Direction:** `workspace-output`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `*mut f64`. **Shape:** rank 1; dimensions (*).

is an array of LENW double precision words used internally for temporary storage. The user must allocate space for this array in the calling program by a statement such as DOUBLE PRECISION WORK(. ) The following table gives the required minimum value for the length of WORK, depending on the value of IMPL and. is an array of LENW double precision words used internally for temporary storage. The user must allocate space for this array in the calling program by a statement such as DOUBLE PRECISION WORK(...) The following table gives the required minimum value for the length of WORK, depending on the value of IMPL and

## `LENW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input).

## `IWORK`

**Direction:** `workspace-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** rank 1; dimensions (*).

is an integer array of length LENIW used internally for temporary storage. The user must allocate space for this array in the calling program by a statement such as INTEGER IWORK(. ) The length of IWORK should be at least 50 if MITER is 0 or 3, or N+50 if MITER is 1, 2, 4, or 5, or MINT is 3, and LENIW should be set to the value used. The contents of IWORK should not be disturbed between calls to DDRIV3. is an integer array of length LENIW used internally for temporary storage. The user must allocate space for this array in the calling program by a statement such as INTEGER IWORK(...) The length of IWORK should be at least 50 if MITER is 0 or 3, or N+50 if MITER is 1, 2, 4, or 5, or MINT is 3, and LENIW should be set to the value used. The contents of IWORK should not be disturbed between calls to DDRIV3.

## `LENIW`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input).

## `JACOBN`

**Direction:** `callback`. **Fortran type:** `UNKNOWN`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

A subroutine supplied by the user, if MITER is 1 or 4. If this is the case, the name must be declared EXTERNAL in the user's calling program. Given a system of N differential equations, it is meaningful to speak about the partial derivative of the I-th right hand side with respect to the J-th dependent variable. In general there are N*N such quantities. Often however the equations can be ordered so that the I-th differential equation only involves dependent variables with index near I, e. g. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `FA`

**Direction:** `callback`. **Fortran type:** `UNKNOWN`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

A subroutine supplied by the user if IMPL is not zero, and. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `NDE`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) The number of differential equations. This is required only for IMPL = 2 or 3, with NDE. LT. N.

## `MXSTEP`

**Direction:** `input`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

(Input) The maximum number of internal steps allowed on one call to DDRIV3.

## `G`

**Direction:** `callback`. **Fortran type:** `DOUBLE PRECISION`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

A double precision FORTRAN function supplied by the user if NROOT is not 0. In this case, the name must be declared EXTERNAL in the user's calling program. G is repeatedly called with different values of IROOT to obtain the value of each of the NROOT equations for which a root is desired. G is of the form: DOUBLE PRECISION FUNCTION G (N, T, Y, IROOT) DOUBLE PRECISION Y(*) GO TO (10,. ), IROOT 10 G =. END (Sample) Here, Y is a vector of length at least N, whose first N components are the solution components at the point T. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `USERS`

**Direction:** `callback`. **Fortran type:** `UNKNOWN`. **Rust ABI type:** `reviewed unsafe extern callback function pointer`. **Shape:** scalar.

A subroutine supplied by the user, if MITER is 3. If this is the case, the name must be declared EXTERNAL in the user's calling program. The routine USERS is called by DDRIV3 when certain linear systems must be solved. The user may choose any method to form, store and solve these systems in order to obtain the solution result that is returned to DDRIV3. In particular, this allows sparse matrix methods to be used. The call sequence for this routine is: SUBROUTINE USERS (Y, YH, YWT, SAVE1, SAVE2, T, H, EL, 8 IMPL, N, NDE, IFLAG) DOUBLE PRECISION Y(*), YH(*), YWT(*), SAVE1(*), 8 SAVE2(*), T, H, EL The input variable IFLAG indicates what action is to be taken. The callback is synchronous, must remain valid for the complete native call, obey the reviewed ABI and documented array extents, may not retain caller pointers, and must not unwind into Fortran.

## `IERFLG`

**Direction:** `input-output`. **Fortran type:** `INTEGER`. **Rust ABI type:** `*mut crate::FortranInteger`. **Shape:** scalar.

An error flag. The error number associated with a diagnostic message (see Section III-A below) is the same as the corresponding value of IERFLG. The meaning of 0 The routine completed successfully. (No message is issued. ) 3 (Warning) The number of steps required to reach TOUT exceeds MXSTEP. 4 (Warning) The value of EPS is too small.

# Return value

This is a Fortran subroutine and has no direct return value. Its results, status, and any persistent solver state are communicated through the documented arguments.

# Callback contract

Each callback uses its exact reviewed Rust function-pointer ABI, is invoked synchronously, must remain valid for the complete native call, must satisfy the documented scalar and array extents, must not retain caller pointers, and **must not unwind** through Fortran.

# Status and error values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IERROR` | `1` | 1. (Absolute error control) |

# Workspace and array requirements

- `Y`: not a workspace argument
- `EWT`: not a workspace argument
- `WORK`: is an array of LENW double precision words used internally for temporary storage. The user must allocate space for this array in the calling program by a statement such as DOUBLE PRECISION WORK(...) The following table gives the required minimum value for the length of WORK, depending on the value of IMPL and
- `IWORK`: is an integer array of length LENIW used internally for temporary storage. The user must allocate space for this array in the calling program by a statement such as INTEGER IWORK(...) The length of IWORK should be at least 50 if MITER is 0 or 3, or N+50 if MITER is 1, 2, 4, or 5, or MINT is 3, and LENIW should be set to the value used. The contents of IWORK should not be disturbed between calls to DDRIV3.

# ABI notes

- Canonical Rust path: `slatec_sys::ode::ddriv3`
- Original SLATEC routine: `DDRIV3`
- Native symbol: `ddriv3_`
- ABI fingerprint: `unavailable`
- Exact Netlib source file: [DDRIV3](https://www.netlib.org/slatec/src/ddriv3.f)

# Safety

Every raw pointer must be non-null unless this argument contract expressly permits null, correctly aligned, and valid for its documented readable or writable extent. Preserve Fortran column-major layout, length, leading-dimension, workspace, callback-lifetime, aliasing, and provider-runtime requirements. The native routine does not retain ordinary argument pointers beyond this call.
