# DSPLP

[Family: Optimization and least squares](../families/optimization-and-least-squares.md) | [All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

## Purpose

Solve linear programming problems involving at most a few thousand constraints and variables. Takes advantage of sparsity in the constraint matrix.

## Description

These are the short usage instructions; for details about other features, options and methods for defining the matrix A, see the extended usage instructions which are contained in the Long Description section below. |Introduction| The subprogram DSPLP( ) solves a linear optimization problem. The problem statement is as follows minimize (transpose of costs)*x subject to A*x=w. The entries of the unknowns x and w may have simple lower or upper bounds (or both), or be free to take on any value. By setting the bounds for x and w, the user is imposing the con- straints of the problem. The matrix A has MRELAS rows and NVARS columns. The vectors costs, x, and w respectively have NVARS, NVARS, and MRELAS number of entries. The input for the problem includes the problem dimensions, MRELAS and NVARS, the array COSTS(*), data for the matrix A, and the bound information for the unknowns x and w, BL(*), BU(*), and IND(*). Only the nonzero entries of the matrix A are passed to DSPLP( ). The output from the problem (when output flag INFO=1) includes optimal values for x and w in PRIMAL(*), optimal values for dual variables of the equations A*x=w and the simple bounds on x in DUALS(*), and the indices of the basic columns,

## Classification

- Historical role: `user_callable`
- Program-unit kind: `subroutine`
- Identity kind: `subroutine`
- Identity status: `retained_verified_program_unit`
- Precision: `f64`
- Scalar kind: `real`
- Primary family: `Optimization and least squares`
- Mathematical domain: `optimization`
- Package provenance: `unknown`
- GAMS classifications: `G2A2`
- Family evidence: `netlib_gams` (`verified`)

## Project coverage

- Source status: `canonical_verified`
- Raw-binding status: `not_bound`
- Build/profile status: `available_but_unselected`
- Audit status: `deeply_audited`
- Safe-API status: `safe_public`
- Implementation status: `safe_api_available`
- Safe Rust paths: `slatec::linear_programming::LinearProgram::<f64>::solve`

## Providers

- Canonical provider: `main-src/src/dsplp.f` (`pinned-reproducible-subset`)
- Alternate providers:
  - `src/dsplp.f` (`live-main-source`)

## Official references

- [Netlib source](https://www.netlib.org/slatec/src/dsplp.f) — `verified_cached`
- [Netlib full source](https://www.netlib.org/cgi-bin/netlibfiles.pl?filename=/slatec/src/dsplp.f) — `verified_cached`
- [Netlib directory entry](https://www.netlib.org/slatec/src/) — `verified_cached`
- [Netlib TOC](https://www.netlib.org/slatec/toc) — `verified_cached`

## Evidence notes

Description selected from `canonical_source_prologue` using `PURPOSE`; confidence: `high`. External-reference statuses are generated offline from separately cached source files, directory indexes, and TOC evidence.

<!-- release-readiness:start -->
## Interface documentation quality

- Documentation work status: `complete-semantic-contract`
- Documentation evidence: bounded selected-source prologue evidence
- Exact Netlib source: [DSPLP](https://www.netlib.org/slatec/src/dsplp.f)

### Arguments

| # | Argument | Direction | Role | Fortran type | Rust raw type | Shape | Contract |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `DUSRMT` | `callback` | `callback` | `REAL` | `reviewed unsafe extern callback function pointer` | scalar | This is the name of a specific subprogram in the DSPLP( ) package used to define the matrix A. In this usage mode of DSPLP( ) the user places the nonzero entries of A in the array DATTRV(*) as given in the description of that parameter. The name DUSRMT must appear in a Fortran EXTERNAL statement. that is used to define the matrix entries when this data is passed to DSPLP( ) as a linear array. In this usage mode of DSPLP( ) the user gives information about the nonzero entries of A in DATTRV(*) as given under the description of that parameter. Users who are passing the matrix data with DUSRMT( ) can skip directly to the description of the input parameter DATTRV(*). |
| 2 | `MRELAS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | These parameters are respectively the number of constraints (the linear relations A*x=w that the unknowns x and w are to satisfy) and the number of entries in the vector x. Both must be. GE. 1. Other values are errors. number of constraint equations. |
| 3 | `NVARS` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | These parameters are respectively the number of constraints (the linear relations A*x=w that the unknowns x and w are to satisfy) and the number of entries in the vector x. Both must be. GE. 1. Other values are errors. number of dependent variables. |
| 4 | `COSTS` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | The NVARS entries of this array are the coefficients of the linear objective function. The value COSTS(J) is the multiplier for variable J of the unknown vector x. Each entry of this array must be defined. entry of this array must be defined. This array can be changed by the user between restarts. See options with KEY=55,57 for details of checkpointing and restarting. |
| 5 | `PRGOPT` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | This array is used to redefine various parameters within DSPLP( ). Frequently, perhaps most of the time, a user will be satisfied and obtain the solutions with no changes to any of these parameters. To try this, simply set PRGOPT(1)=1. D0. For users with more sophisticated needs, DSPLP( ) provides several options that may be used to take advantage of more detailed knowledge of the problem or satisfy other utilitarian needs. The complete description of how to use this option array to utilize additional subprogram features is found under the heading of DSPLP( ) Subprogram Options in the Extended Usage Instructions. |
| 6 | `DATTRV` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | The array DATTRV(*) contains data for the matrix A as follows: Each column (numbered J) requires (floating point) data con- sisting of the value (-J) followed by pairs of values. Each pair consists of the row index immediately followed by the value of the matrix at that entry. A value of J=0 signals that there are no more columns. The required length of is 2*no. of nonzeros + NVARS + 1. 69 Pass an absolute tolerance to use for the feasibility test when the usual relative error test indicates infeasibility. |
| 7 | `BL` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | The values of IND(*) are input parameters that define the form of the bounds for the unknowns x and w. The values for the bounds are found in the arrays BL(*) and BU(*) as follows. For values of J between 1 and NVARS, if IND(J)=1, then X(J). GE. BL(J); BU(J) is not used. if IND(J)=2, then X(J). |
| 8 | `BU` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | The values of IND(*) are input parameters that define the form of the bounds for the unknowns x and w. The values for the bounds are found in the arrays BL(*) and BU(*) as follows. For values of J between 1 and NVARS, if IND(J)=1, then X(J). GE. BL(J); BU(J) is not used. if IND(J)=2, then X(J). |
| 9 | `IND` | `input` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | The values of IND(*) are input parameters that define the form of the bounds for the unknowns x and w. The values for the bounds are found in the arrays BL(*) and BU(*) as follows. For values of J between 1 and NVARS, if IND(J)=1, then X(J). GE. BL(J); BU(J) is not used. if IND(J)=2, then X(J). |
| 10 | `INFO` | `status-output` | `status` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The integer flag INFO indicates why DSPLP( ) has returned to the user. If INFO=1 the solution has been computed. In this case X(J)=PRIMAL(J) and W(I)=PRIMAL(I+NVARS). The dual variables for the equations A*x=w are in the array DUALS(I)=dual for equation number I. The dual value for the component X(J) that has an upper or lower bound (or both) is returned in CALCULATE VAL, THE MINIMAL VALUE OF THE OBJECTIVE FUNCTION. VAL=DDOT(NVARS,COSTS,1,PRIMAL,1) STOP END \|End of Example of Usage \| \|Usage of DSPLP( ) Subprogram Options. |
| 11 | `PRIMAL` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | The integer flag INFO indicates why DSPLP( ) has returned to the user. If INFO=1 the solution has been computed. In this case X(J)=PRIMAL(J) and W(I)=PRIMAL(I+NVARS). The dual variables for the equations A*x=w are in the array DUALS(I)=dual for equation number I. The dual value for the component X(J) that has an upper or lower bound (or both) is returned in EXTERNAL DUSRMT MRELAS=3 NVARS=3 DEFINE THE ARRAY COSTS(*) FOR THE OBJECTIVE FUNCTION. COSTS(01)=1. |
| 12 | `DUALS` | `input-output` | `array` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | The integer flag INFO indicates why DSPLP( ) has returned to the user. If INFO=1 the solution has been computed. In this case X(J)=PRIMAL(J) and W(I)=PRIMAL(I+NVARS). The dual variables for the equations A*x=w are in the array DUALS(I)=dual for equation number I. The dual value for the component X(J) that has an upper or lower bound (or both) is returned in The only other values for INFO are. LT. |
| 13 | `IBASIS` | `input-output` | `array` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | \|Fortran Declarations Required:\| DIMENSION COSTS(NVARS),PRGOPT(*),DATTRV(*), EXTERNAL DUSRMT MRELAS=3 NVARS=3 DEFINE THE ARRAY COSTS(*) FOR THE OBJECTIVE FUNCTION. COSTS(01)=1. COSTS(02)=1. COSTS(03)=1. PLACE THE NONZERO INFORMATION ABOUT THE MATRIX IN DATTRV(*). DEFINE COL. |
| 14 | `WORK` | `workspace-output` | `workspace` | `DOUBLE PRECISION` | `*mut f64` | rank 1; dimensions (*) | EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. The exact lengths will be determined by user-required options and data transferred to the subprogram DUSRMT( ). The values of LW and LIW, the lengths of the arrays WORK(*) and IWORK(*), must satisfy the inequalities MRELAS=3 NVARS=3 DEFINE THE ARRAY COSTS(*) FOR THE OBJECTIVE FUNCTION. COSTS(01)=1. COSTS(02)=1. COSTS(03)=1. |
| 15 | `LW` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | GE. 4*NVARS+ 8*MRELAS+LAMAT+ LBM LIW. NVARS+11*MRELAS+LAMAT+2*LBM It is an error if they do not both satisfy these inequalities. (The subprogram will inform the user of the required lengths if either LW or LIW is wrong. ) The values of LAMAT and LBM nominally are LAMAT=4*NVARS+7 and LBM =8*MRELAS LAMAT determines the length of the sparse matrix storage area. The value of LBM determines the amount of storage available to decompose and update the active basis matrix. |
| 16 | `IWORK` | `workspace-output` | `workspace` | `INTEGER` | `*mut crate::FortranInteger` | rank 1; dimensions (*) | EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. The exact lengths will be determined by user-required options and data transferred to the subprogram DUSRMT( ). The values of LW and LIW, the lengths of the arrays WORK(*) and IWORK(*), must satisfy the inequalities The arrays WORK(*) and IWORK(*) are respectively floating point and type INTEGER working arrays for DSPLP( ) and its subprograms. The lengths of these arrays are respectively LW and LIW. These parameters must satisfy the inequalities noted above under the heading "Fortran Declarations Required:" It is an error if either value is too small. \|Input/Output files required:\| Fortran unit 1 is used by DSPLP( ) to store the sparse matrix A out of high-speed memory. |
| 17 | `LIW` | `input-output` | `scalar` | `INTEGER` | `*mut crate::FortranInteger` | scalar | The arrays WORK(*) and IWORK(*) are respectively floating point and type INTEGER working arrays for DSPLP( ) and its subprograms. The lengths of these arrays are respectively LW and LIW. These parameters must satisfy the inequalities noted above under the heading "Fortran Declarations Required:" It is an error if either value is too small. \|Input/Output files required:\| Fortran unit 1 is used by DSPLP( ) to store the sparse matrix A out of high-speed memory. A crude upper bound for the amount of information written on unit 1 is 6*nz, where nz is the number of nonzero entries in A. noted above under the heading "Fortran Declarations Required. |

The authoritative public-documentation inventory records argument evidence ranges, nullability, shapes, relationships, leading dimensions, option values, and overwrite behavior. Native code does not retain ordinary argument pointers.

### Return value

This is a Fortran subroutine and has no direct return value; outputs are documented in its argument contract.

### Callback contract

Callback arguments must use the exact reviewed callback ABI, remain valid for the entire native call, satisfy their documented storage contract, and never unwind through Fortran.

### Error and status values

| Status | Value | Meaning |
| --- | ---: | --- |
| `IND` | `1` | , then X(J) .GE. BL(J); BU(J) is not used. |
| `IND` | `2` | , then X(J) .LE. BU(J); BL(J) is not used. |
| `IND` | `3` | , then BL(J) .LE. X(J) .LE. BU(J),(BL(J)=BU(J) ok) |
| `IND` | `4` | , then X(J) is free to have any value, |
| `IND` | `3` | , BL(I) must be .LE. BU(I). The condition BL(I).GT. BU(I) indicates infeasibility and is an error. These arrays are output parameters only under the (unusual) circumstances where the stated problem is infeasible, has an unbounded optimum value, or both. These respective conditions |
| `IND` | `-1` | ,-2 or -3. See the Extended Usage Instructions for further details. |
| `IND` | `1` | ,...,MRELAS This array contains the indices of the variables that are ). A value of IBASIS(I) between 1 and NVARS corresponds to the variable X(IBASIS(I)). A value of IBASIS(I) between NVARS+1 and NVARS+ MRELAS corresponds to the variable W(IBASIS(I)-NVARS). The values of IND(*) are input parameters that define the form of the bounds for the unknowns x and w. The values for the bounds are found in the arrays BL(*) and BU(*) as follows. For values of J between 1 and NVARS, , then X(J) .GE. BL(J); BU(J) is not used. |
| `IND` | `2` | , then X(J) .LE. BU(J); BL(J) is not used. |
| `IND` | `3` | , then BL(J) .LE. X(J) .LE. BU(J),(BL(J)=BU(J) ok) |
| `IND` | `4` | , then X(J) is free to have any value, |
| `IND` | `3` | , BL(I) must be .LE. BU(I). The condition BL(I).GT. BU(I) indicates infeasibility and is an error. These arrays can be changed by the user between restarts. See |
| `IND` | `55` | ,57 for details of checkpointing and restarting. These arrays are output parameters only under the (unusual) circumstances where the stated problem is infeasible, has an unbounded optimum value, or both. These respective conditions |
| `IND` | `-1` | ,-2 or -3. For INFO=-1 or -3 certain comp- onents of the vectors x or w will not satisfy the input bounds. If component J of X or component I of W does not satisfy its input |
| `IND` | `-4` | or IND(I+NVARS)=-4, |
| `IND` | `-2` | or -3 certain components of the vector x could not be used as basic variables because the objective function would have become unbounded. In particular if component J of x corresponds to such a variable, |
| `IND` | `-3` | . Further, if the input value of IND(J) |
| `IND` | `1` | , then BU(J)=BL(J); |
| `IND` | `2` | , then BL(J)=BU(J); |
| `IND` | `4` | , then BL(J)=0.,BU(J)=0. (The J-th variable in x has been restricted to an appropriate feasible value.) The negative output value for IND(*) allows the user to identify those constraints that are not satisfied or those variables that would cause unbounded values of the objective function. Note that the absolute value of IND(*), together with BL(*) and BU(*), are valid input to DSPLP( ). In the case of infeasibility the sum of magnitudes of the infeasible values is minimized. Thus one could reenter DSPLP( ) with these components of x or w now fixed at their present values. This involves setting |
| `IND` | `3` | 3, and BL(*) = BU(*). |
| `IND` | `1` | ,...,MRELAS This array contains the indices of the variables that are ). A value of IBASIS(I) between 1 and NVARS corresponds to the variable X(IBASIS(I)). A value of IBASIS(I) between NVARS+1 and NVARS+ MRELAS corresponds to the variable W(IBASIS(I)-NVARS). Computing with the Matrix A after Calling DSPLP( ) Following the return from DSPLP( ), nonzero entries of the MRELAS by NVARS matrix A are available for usage by the user. The method for obtaining the next nonzero in column J with a row index strictly greater than I in value, is completed by executing CALL DPNNZR(I,AIJ,IPLACE,WORK,IWORK,J) The value of I is also an output parameter. If I.LE.0 on output, |
| `IND` | `>0` | , the output value for component number I of column J is in AIJ. The parameters WORK(*) and IWORK(*) are the same arguments as in the call to DSPLP( ). The parameter IPLACE is a single INTEGER working variable. The data structure used for storage of the matrix A within DSPLP() corresponds to sequential storage by columns as defined in SAND78-0785. Note that the names of the subprograms LNNZRS(), LCHNGS(),LINITM(),LLOC(),LRWPGE(), and LRWVIR() have been changed to DPNNZR(),DPCHNG(),PINITM(),IPLOC(),DPRWPG(), and DPRWVR() respectively. The error processing subprogram LERROR() is no longer used; XERMSG() is used instead. \|Subprograms Required by DSPLP( )\| Called by DSPLP() are DPLPMN(),DPLPUP(),DPINIT(),DPOPT(), DPLPDM(),DPLPCE(),DPINCW(),DPLPFL(), DPLPFE(),DPLPMU(). Error Processing Subprograms XERMSG(),I1MACH(),D1MACH() Sparse Matrix Subprograms DPNNZR(),DPCHNG(),DPRWPG(),DPRWVR(), Mass Storage File Subprograms SOPENM(),SCLOSM(),DREADP(),DWRITP() Basic Linear Algebra Subprograms DCOPY(),DASUM(),DDOT() Sparse Matrix Basis Handling Subprograms LA05AD(),LA05BD(), LA05CD(),LA05ED(),MC20AD() Vector Output Subprograms DVOUT(),IVOUT() Machine-sensitive Subprograms I1MACH( ),D1MACH( ), SOPENM(),SCLOSM(),DREADP(),DWRITP(). COMMON Block Used /LA05DD/ SMALL,LP,LENL,LENU,NCP,LROW,LCOL See the document AERE-R8269 for further details. \|Example of DSPLP( ) Usage\| PROGRAM LPEX THE OPTIMIZATION PROBLEM IS TO FIND X1, X2, X3 THAT MINIMIZE X1 + X2 + X3, X1.GE.0, X2.GE.0, X3 UNCONSTRAINED. THE UNKNOWNS X1,X2,X3 ARE TO SATISFY CONSTRAINTS |
| `IND` | `5` | 5 X1 -2*X2 .LE.3 2*X2 - X3.GE.4 WE FIRST DEFINE THE DEPENDENT VARIABLES W1=X1 -3*X2 +4*X3 W2=X1- 2*X2 |
| `IND` | `2` | 2*X2 -X3 WE NOW SHOW HOW TO USE DSPLP( ) TO SOLVE THIS LINEAR OPTIMIZATION PROBLEM. EACH REQUIRED STEP WILL BE SHOWN IN THIS EXAMPLE. DIMENSION COSTS(03),PRGOPT(01),DATTRV(18),BL(06),BU(06),IND(06), |
| `IND` | `0` | . |
| `IND` | `5` | ,W2.LE.3, AND W3.GE.4. |
| `IND` | `3` | . |
| `IND` | `4` | . |
| `IND` | `1` | INDICATE THAT NO MODIFICATIONS TO OPTIONS ARE IN USE. DEFINE THE WORKING ARRAY LENGTHS. |
| `IND` | `103` | CALL DSPLP(DUSRMT,MRELAS,NVARS,COSTS,PRGOPT,DATTRV, |
| `INFO` | `1` | the solution has been computed. In this case X(J)=PRIMAL(J) and W(I)=PRIMAL(I+NVARS). The dual variables for the equations A*x=w are in the array DUALS(I)=dual for equation number I. The dual value for the component X(J) that has an upper or lower bound (or both) is returned in The integer flag INFO indicates why DSPLP( ) has returned to the CALCULATE VAL, THE MINIMAL VALUE OF THE OBJECTIVE FUNCTION. VAL=DDOT(NVARS,COSTS,1,PRIMAL,1) STOP END \|End of Example of Usage \| \|Usage of DSPLP( ) Subprogram Options.\| Users frequently have a large variety of requirements for linear optimization software. Allowing for these varied requirements is at cross purposes with the desire to keep the usage of DSPLP( ) as simple as possible. One solution to this dilemma is as follows. (1) Provide a version of DSPLP( ) that solves a wide class of problems and is easy to use. (2) Identify parameters within DSPLP() that certain users may want to change. (3) Provide a means of changing any selected number of these parameters that does not require changing all of them. Changing selected parameters is done by requiring that the user provide an option array, PRGOPT(*), to DSPLP( ). The contents of PRGOPT(*) inform DSPLP( ) of just those options that are going to be modified within the total set of possible parameters that can be modified. The array PRGOPT(*) is a linked list consisting of groups of data of the following form LINK KEY SWITCH data set that describe the desired options. The parameters LINK, KEY and switch are each one word and are always required. The data set can be comprised of several words or can be empty. The number of words in the data set for each option depends on the value of the parameter KEY. The value of LINK points to the first entry of the next group of data within PRGOPT(*). The exception is when there are no more and the values for KEY, SWITCH and data set are not referenced. The general layout of |
| `INFO` | `-17` | . |
| `INFO` | `0` | option is off; no checking of the data matrix is done |
| `INFO` | `1` | option is on; checking is done. data set =ASMALL ABIG |
| `INFO` | `5` | checking if the residuals are feasible. Normally, TOLLS=RELPR, where RELPR is the machine precision. |
| `INFO` | `0` | option is off; TOLLS=RELPR. |
| `INFO` | `1` | option is on. data set =TOLLS |
| `INFO` | `4` | columns to enter the basis. Normally, DSPLP( ) uses the steepest edge pricing strategy which is the best local move. The steepest edge pricing strategy generally uses fewer iterations than the minimum reduced cost pricing, but each iteration costs more in the number of calculations done. The steepest edge pricing is considered to be more efficient. However, this is very problem dependent. That is why DSPLP( ) provides the option of either pricing strategy. |
| `INFO` | `0` | option is off; steepest option edge pricing is used. |
| `INFO` | `1` | option is on; minimum reduced cost pricing is used. data set =empty |
| `INFO` | `3` | recalculating the error in the primal solution. Normally, MXITBR is set to 10. The error in the primal solution is used to monitor the error in solving the linear system. This is an expensive calculation and every tenth iteration is generally often enough. |
| `INFO` | `0` | option is off; MXITBR=10. |
| `INFO` | `1` | option is on. data set =MXITBR |
| `INFO` | `4` | (at most) to be found at each iteration of choosing a variable to enter the basis. Normally NPP is set to NVARS which implies that all of the reduced costs are computed at each such step. This "partial pricing" may very well increase the total number of iterations required. However it decreases the number of calculations at each iteration. therefore the effect on overall efficiency is quite problem-dependent. |
| `INFO` | `0` | option is off; NPP=NVARS |
| `INFO` | `1` | option is on. data set =NPP |
| `INFO` | `4` | error estimates for the primal and dual linear algebraic systems |
| `INFO` | `1` | 1.D0, but in some environments it may be necessary to reset PHI to the range 0.001-0.01. This is particularly important for machines with short word lengths. |
| `INFO` | `0` | 0 option is off; PHI=1.D0. |
| `INFO` | `1` | 1 option is on. Data Set = PHI |
| `INFO` | `4` | with the DSPLP() package, for passing a standard Fortran two- dimensional array containing the constraint matrix. Thus the sub- program DFULMT must be declared in a Fortran EXTERNAL statement. The two-dimensional array is passed as the argument DATTRV. The information about the array and problem dimensions are passed in the option array PRGOPT(*). It is an error if DFULMT() is used and this information is not passed in PRGOPT(*). |
| `INFO` | `0` | 0 option is off; this is an error is DFULMT() is used. |
| `INFO` | `1` | 1 option is on. Data Set = IA = row dimension of two-dimensional array. |

### Storage and workspace requirements

`WORK`: EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. The exact lengths will be determined by user-required options and data transferred to the subprogram DUSRMT( ). The values of LW and LIW, the lengths of the arrays WORK(*) and IWORK(*), must satisfy the inequalities MRELAS=3 NVARS=3 DEFINE THE ARRAY COSTS(*) FOR THE OBJECTIVE FUNCTION. COSTS(01)=1. COSTS(02)=1. COSTS(03)=1. PLACE THE NONZERO INFORMATION ABOUT THE MATRIX IN DATTRV(*). DEFINE COL. 1: DATTRV(01)=-1 DATTRV(02)=1 DATTRV(03)=1. DATTRV(04)=2 DATTRV(05)=1. DEFINE COL. 2: DATTRV(06)=-2 DATTRV(07)=1 DATTRV(08)=-3. DATTRV(09)=2 DATTRV(10)=-2. DATTRV(11)=3 DATTRV(12)=2. DEFINE COL. 3: DATTRV(13)=-3 DATTRV(14)=1 DATTRV(15)=4. DATTRV(16)=3 DATTRV(17)=-1. DATTRV(18)=0 CONSTRAIN X1,X2 TO BE NONNEGATIVE. LET X3 HAVE NO BOUNDS. BL(1)=0. IND(1)=1 BL(2)=0. IND(2)=1 IND(3)=4 CONSTRAIN W1=5,W2.LE.3, AND W3.GE.4. BL(4)=5. BU(4)=5. IND(4)=3 BU(5)=3. IND(5)=2 BL(6)=4. IND(6)=1 INDICATE THAT NO MODIFICATIONS TO OPTIONS ARE IN USE. PRGOPT(01)=1 DEFINE THE WORKING ARRAY LENGTHS. LW=079 LIW=103 CALL DSPLP(DUSRMT,MRELAS,NVARS,COSTS,PRGOPT,DATTRV, CALCULATE VAL, THE MINIMAL VALUE OF THE OBJECTIVE FUNCTION. VAL=DDOT(NVARS,COSTS,1,PRIMAL,1) STOP END |End of Example of Usage | |Usage of DSPLP( ) Subprogram Options.| Users frequently have a large variety of requirements for linear optimization software. Allowing for these varied requirements is at cross purposes with the desire to keep the usage of DSPLP( ) as simple as possible. One solution to this dilemma is as follows. (1) Provide a version of DSPLP( ) that solves a wide class of problems and is easy to use. (2) Identify parameters within DSPLP() that certain users may want to change. (3) Provide a means of changing any selected number of these parameters that does not require changing all of them. Changing selected parameters is done by requiring that the user provide an option array, PRGOPT(*), to DSPLP( ). The contents of PRGOPT(*) inform DSPLP( ) of just those options that are going to be modified within the total set of possible parameters that can be modified. The array PRGOPT(*) is a linked list consisting of groups of data of the following form LINK KEY SWITCH data set that describe the desired options. The parameters LINK, KEY and switch are each one word and are always required. The data set can be comprised of several words or can be empty. The number of words in the data set for each option depends on the value of the parameter KEY. The value of LINK points to the first entry of the next group of data within PRGOPT(*). The exception is when there are no more options to change. In that case, LINK=1 and the values for KEY, SWITCH and data set are not referenced. The general layout of

`IWORK`: EXTERNAL DUSRMT The dimensions of PRGOPT(*) and DATTRV(*) must be at least 1. The exact lengths will be determined by user-required options and data transferred to the subprogram DUSRMT( ). The values of LW and LIW, the lengths of the arrays WORK(*) and IWORK(*), must satisfy the inequalities The arrays WORK(*) and IWORK(*) are respectively floating point and type INTEGER working arrays for DSPLP( ) and its subprograms. The lengths of these arrays are respectively LW and LIW. These parameters must satisfy the inequalities noted above under the heading "Fortran Declarations Required:" It is an error if either value is too small. |Input/Output files required:| Fortran unit 1 is used by DSPLP( ) to store the sparse matrix A out of high-speed memory. A crude upper bound for the amount of information written on unit 1 is 6*nz, where nz is the number of nonzero entries in A. noted above under the heading "Fortran Declarations Required." out of high-speed memory. This direct access file is opened within the package under the following two conditions. 1. When the Save/Restore feature is used. 2. When the constraint matrix is so large that storage out of high-speed memory is required. The user may need to close unit 1 (with deletion from the job step) in the main program unit when several calls are made to DSPLP( ). A crude The unit number may be redefined to any other positive value by means of input in the option array PRGOPT(*). Fortran unit 2 is used by DSPLP( ) only when the Save/Restore feature is desired. Normally this feature is not used. It is activated by means of input in the option array PRGOPT(*). On some computer systems the user may need to open unit 2 before executing a call to DSPLP( ). This file is type sequential and is unformatted. Fortran unit=I1MACH(2) (check local setting) is used by DSPLP( ) when the printed output feature (KEY=51) is used. Normally this feature is not used. It is activated by input in the options array PRGOPT(*). For many computer systems I1MACH(2)=6. MRELAS=3 NVARS=3 DEFINE THE ARRAY COSTS(*) FOR THE OBJECTIVE FUNCTION. COSTS(01)=1. COSTS(02)=1. COSTS(03)=1. PLACE THE NONZERO INFORMATION ABOUT THE MATRIX IN DATTRV(*). DEFINE COL. 1: DATTRV(01)=-1 DATTRV(02)=1 DATTRV(03)=1. DATTRV(04)=2 DATTRV(05)=1. DEFINE COL. 2: DATTRV(06)=-2 DATTRV(07)=1 DATTRV(08)=-3. DATTRV(09)=2 DATTRV(10)=-2. DATTRV(11)=3 DATTRV(12)=2. DEFINE COL. 3: DATTRV(13)=-3 DATTRV(14)=1 DATTRV(15)=4. DATTRV(16)=3 DATTRV(17)=-1. DATTRV(18)=0 CONSTRAIN X1,X2 TO BE NONNEGATIVE. LET X3 HAVE NO BOUNDS. BL(1)=0. IND(1)=1 BL(2)=0. IND(2)=1 IND(3)=4 CONSTRAIN W1=5,W2.LE.3, AND W3.GE.4. BL(4)=5. BU(4)=5. IND(4)=3 BU(5)=3. IND(5)=2 BL(6)=4. IND(6)=1 INDICATE THAT NO MODIFICATIONS TO OPTIONS ARE IN USE. PRGOPT(01)=1 DEFINE THE WORKING ARRAY LENGTHS. LW=079 LIW=103 CALL DSPLP(DUSRMT,MRELAS,NVARS,COSTS,PRGOPT,DATTRV, CALCULATE VAL, THE MINIMAL VALUE OF THE OBJECTIVE FUNCTION. VAL=DDOT(NVARS,COSTS,1,PRIMAL,1) STOP END |End of Example of Usage | |Usage of DSPLP( ) Subprogram Options.| Users frequently have a large variety of requirements for linear optimization software. Allowing for these varied requirements is at cross purposes with the desire to keep the usage of DSPLP( ) as simple as possible. One solution to this dilemma is as follows. (1) Provide a version of DSPLP( ) that solves a wide class of problems and is easy to use. (2) Identify parameters within DSPLP() that certain users may want to change. (3) Provide a means of changing any selected number of these parameters that does not require changing all of them. Changing selected parameters is done by requiring that the user provide an option array, PRGOPT(*), to DSPLP( ). The contents of PRGOPT(*) inform DSPLP( ) of just those options that are going to be modified within the total set of possible parameters that can be modified. The array PRGOPT(*) is a linked list consisting of groups of data of the following form LINK SWITCH data set that describe the desired options. The parameters LINK, KEY and switch are each one word and are always required. The data set can be comprised of several words or can be empty. The number of words in the data set for each option depends on the value of the parameter KEY. The value of LINK points to the first entry of the next group of data within PRGOPT(*). The exception is when there are no more options to change. In that case, LINK=1 and the values for KEY, SWITCH and data set are not referenced. The general layout of

### Provider, ABI, and safety

Canonical Rust path: `slatec_sys::linear_programming::dsplp`. Native symbol: `dsplp_`. Declaration feature: `optimization-linear-programming-in-memory`. Provider feature: `optimization-linear-programming-in-memory`. ABI fingerprint: `unavailable`.

# Safety

Every pointer must be non-null unless its argument record explicitly permits null, correctly aligned, and valid for its documented readable or writable extent. Callers must preserve Fortran column-major layout, dimensions, leading dimensions, workspace capacity, callback lifetime, and the selected provider's runtime serialization requirements. Mutable arguments may not alias in a way the native routine does not permit.
<!-- release-readiness:end -->

<!-- raw-api-status:start -->
## Raw Rust API status

This generated status is evidence only; see the [authoritative inventory](../../../generated/raw-api/routine-status.json).

- Public raw API status: `canonical-public`
- ABI validation: `pending`
- Canonical Rust path: `slatec_sys::linear_programming::dsplp`
- Public declaration feature: `optimization-linear-programming-in-memory`
- `all`-feature reachability: `transitively_enabled_by_all`
- Provider-backed callable symbol: `yes` (`observed_exactly_once`)
- Documentation status: `complete_authored`
- Compile-test status: `compiler_observed`
- Link-test status: `passed`
- Runtime validation: `passed`
- Safe-wrapper status: `slatec::linear_programming::LinearProgram::<f64>::solve`
- Safe-facade link test: `not_recorded`
- Safe-facade runtime test: `not_recorded`
- Exclusion or deferment reason: `none`
<!-- raw-api-status:end -->
