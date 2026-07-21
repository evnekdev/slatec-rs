# Manual semantic-review sample

The deterministic sample contains three routines per family, every source-hash-guarded correction, and every source/dataflow direction disagreement. `machine-semantic-audit-passed` records the required review result; human review can add external evidence without changing generated contracts.

## `ACOSH`

- Canonical path: `slatec_sys::special::acosh`
- Family: `Elementary and transcendental functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input)

## `AI`

- Canonical path: `slatec_sys::special::airy::ai`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input)

## `AIE`

- Canonical path: `slatec_sys::special::airy::aie`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input)

## `ALBETA`

- Canonical path: `slatec_sys::special::beta::albeta`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `B` (input)

## `ALGAMS`

- Canonical path: `slatec_sys::special::algams`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `ALGAM` (input), `SGNGAM` (input)

## `ALNREL`

- Canonical path: `slatec_sys::special::elementary::alnrel`
- Family: `Elementary and transcendental functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input)

## `ASINH`

- Canonical path: `slatec_sys::special::asinh`
- Family: `Elementary and transcendental functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input)

## `AVINT`

- Canonical path: `slatec_sys::quadrature::avint`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `Y` (input), `N` (input), `XLO` (input), `XUP` (input), `ANS` (output), `IERR` (output)

## `BAKVEC`

- Canonical path: `slatec_sys::linear_algebra::eigen::bakvec`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `T` (input-output), `E` (input-output), `M` (input), `Z` (input-output), `IERR` (output)

## `BALANC`

- Canonical path: `slatec_sys::linear_algebra::eigen::balanc`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input-output), `LOW` (output), `IGH` (output), `SCALE` (output)

## `BALBAK`

- Canonical path: `slatec_sys::linear_algebra::eigen::balbak`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `SCALE` (input), `M` (input), `Z` (input-output)

## `BANDR`

- Canonical path: `slatec_sys::linear_algebra::eigen::bandr`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `MB` (input), `A` (input), `D` (output), `E` (output), `E2` (output), `MATZ` (input), `Z` (output)

## `BANDV`

- Canonical path: `slatec_sys::linear_algebra::eigen::bandv`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `MBW` (input), `A` (input-output), `E21` (input), `M` (input), `W` (input-output), `Z` (input-output), `IERR` (output), `NV` (input), `RV` (output), `RV6` (output)

## `BESJ`

- Canonical path: `slatec_sys::special::bessel::besj`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `ALPHA` (input), `N` (input), `Y` (output), `NZ` (status-output)

## `BESK`

- Canonical path: `slatec_sys::special::bessel::besk`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `FNU` (input), `KODE` (input), `N` (input), `Y` (output), `NZ` (status-output)

## `BESKES`

- Canonical path: `slatec_sys::special::bessel::beskes`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XNU` (input), `X` (input), `NIN` (input), `BKE` (input-output)

## `BESKS`

- Canonical path: `slatec_sys::special::bessel::besks`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XNU` (input), `X` (input), `NIN` (input), `BK` (input-output)

## `BESY`

- Canonical path: `slatec_sys::special::bessel::besy`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `FNU` (input), `N` (input), `Y` (output)

## `BINT4`

- Canonical path: `slatec_sys::interpolation::bint4`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `Y` (input), `NDATA` (input), `IBCL` (input), `IBCR` (input), `FBCL` (input), `FBCR` (input), `KNTOPT` (input), `T` (input-output), `BCOEF` (output), `N` (output), `K` (output), `W` (workspace-output)

## `BINTK`

- Canonical path: `slatec_sys::interpolation::bintk`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `Y` (input), `T` (input), `N` (input), `K` (input), `BCOEF` (output), `Q` (workspace-output), `WORK` (workspace-output)

## `BISECT`

- Canonical path: `slatec_sys::linear_algebra::eigen::bisect`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `EPS1` (input-output), `D` (input-output), `E` (input-output), `E2` (input-output), `LB` (input), `UB` (input), `MM` (input), `M` (output), `W` (output), `IND` (status-output), `IERR` (output), `RV4` (output), `RV5` (output)

## `BLKTRI`

- Canonical path: `slatec_sys::pde::fishpack::blktri`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `IFLG` (input), `NP` (input), `N` (input), `AN` (input), `BN` (input), `CN` (input), `MP` (input), `M` (input), `AM` (input), `BM` (input), `CM` (input), `IDIMY` (input), `Y` (input-output), `IERROR` (status-output), `W` (input-output)

## `BNDACC`

- Canonical path: `slatec_sys::linear_algebra::banded::bndacc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `G` (input-output), `MDG` (input-output), `NB` (input), `IP` (input-output), `IR` (input-output), `MT` (input), `JT` (input-output)

## `BNDSOL`

- Canonical path: `slatec_sys::linear_algebra::banded::bndsol`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `MODE` (input), `G` (input-output), `MDG` (input-output), `NB` (input), `IP` (input-output), `IR` (input-output), `X` (input-output), `N` (input), `RNORM` (output)

## `BQR`

- Canonical path: `slatec_sys::linear_algebra::eigen::bqr`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `MB` (input-output), `A` (input-output), `T` (input-output), `R` (input-output), `IERR` (output), `NV` (input), `RV` (output)

## `BSKIN`

- Canonical path: `slatec_sys::special::bskin`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `N` (input), `KODE` (input-output), `M` (input), `Y` (input-output), `NZ` (status-output), `IERR` (output)

## `BSPDR`

- Canonical path: `slatec_sys::interpolation::bspdr`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `A` (input), `N` (input), `K` (input), `NDERIV` (input), `AD` (output)

## `BSPEV`

- Canonical path: `slatec_sys::interpolation::bspev`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `AD` (input), `N` (input), `K` (input), `NDERIV` (input), `X` (input), `INEV` (input-output), `SVALUE` (output), `WORK` (workspace-output)

## `BSPPP`

- Canonical path: `slatec_sys::interpolation::bsppp`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `A` (input), `N` (input), `K` (input), `LDC` (input), `C` (output), `XI` (input-output), `LXI` (output), `WORK` (workspace-output)

## `BSPVD`

- Canonical path: `slatec_sys::interpolation::bspvd`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `K` (input), `NDERIV` (input), `X` (input), `ILEFT` (input), `LDVNIK` (input), `VNIKX` (output), `WORK` (workspace-output)

## `BSPVN`

- Canonical path: `slatec_sys::interpolation::bspvn`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `JHIGH` (input), `K` (input), `INDEX` (input), `X` (input), `ILEFT` (input), `VNIKX` (output), `WORK` (workspace-output), `IWORK` (workspace-output)

## `BSQAD`

- Canonical path: `slatec_sys::quadrature::bsqad`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `BCOEF` (input), `N` (input), `K` (input), `X1` (input), `X2` (input), `BQUAD` (output), `WORK` (workspace-output)

## `BVALU`

- Canonical path: `slatec_sys::interpolation::bvalu`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `A` (input), `N` (input), `K` (input), `IDERIV` (input), `X` (input), `INBV` (input-output), `WORK` (workspace-output)

## `CAIRY`

- Canonical path: `slatec_sys::special::complex::cairy`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `Z` (input), `ID` (input), `KODE` (input), `AI` (input-output), `NZ` (status-output), `IERR` (output)

## `CAXPY`

- Canonical path: `slatec_sys::blas::level1::caxpy`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `CA` (input), `CX` (input), `INCX` (input), `CY` (input-output), `INCY` (input)

## `CBAL`

- Canonical path: `slatec_sys::linear_algebra::eigen::cbal`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `AR` (input-output), `AI` (input-output), `LOW` (output), `IGH` (output), `SCALE` (output)

## `CBESH`

- Canonical path: `slatec_sys::special::complex::cbesh`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `Z` (input), `FNU` (input), `KODE` (input), `M` (input), `N` (input), `CY` (input-output), `NZ` (status-output), `IERR` (output)

## `CBESI`

- Canonical path: `slatec_sys::special::complex::cbesi`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `Z` (input), `FNU` (input), `KODE` (input), `N` (input), `CY` (input-output), `NZ` (status-output), `IERR` (output)

## `CBESJ`

- Canonical path: `slatec_sys::special::complex::cbesj`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `Z` (input), `FNU` (input), `KODE` (input), `N` (input), `CY` (input-output), `NZ` (status-output), `IERR` (output)

## `CBESK`

- Canonical path: `slatec_sys::special::complex::cbesk`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `Z` (input), `FNU` (input), `KODE` (input), `N` (input), `CY` (input-output), `NZ` (status-output), `IERR` (output)

## `CBESY`

- Canonical path: `slatec_sys::special::complex::cbesy`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `Z` (input), `FNU` (input), `KODE` (input-output), `N` (input), `CY` (input-output), `NZ` (status-output), `CWRK` (workspace-output), `IERR` (output)

## `CBIRY`

- Canonical path: `slatec_sys::special::complex::cbiry`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `Z` (input), `ID` (input), `KODE` (input), `BI` (input-output), `IERR` (output)

## `CBLKTR`

- Canonical path: `slatec_sys::pde::fishpack::complex::cblktr`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `IFLG` (input), `NP` (input), `N` (input), `AN` (input), `BN` (input), `CN` (input), `MP` (input), `M` (input), `AM` (input), `BM` (input), `CM` (input), `IDIMY` (input), `Y` (input-output), `IERROR` (status-output), `W` (input-output)

## `CCHDC`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cchdc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `P` (input), `WORK` (workspace-output), `JPVT` (input-output), `JOB` (input), `INFO` (status-output)

## `CCHDD`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cchdd`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `R` (input-output), `LDR` (input), `P` (input), `X` (input), `Z` (input-output), `LDZ` (input), `NZ` (input), `Y` (input), `RHO` (input-output), `C` (output), `S` (output), `INFO` (status-output)

## `CCHEX`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cchex`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `R` (input-output), `LDR` (input), `P` (input), `K` (input), `L` (input), `Z` (input-output), `LDZ` (input), `NZ` (input), `C` (output), `S` (output), `JOB` (input)

## `CCHUD`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cchud`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `R` (input), `LDR` (input), `P` (input), `X` (input), `Z` (input-output), `LDZ` (input), `NZ` (input), `Y` (input), `RHO` (input-output), `C` (output), `S` (output)

## `CCOPY`

- Canonical path: `slatec_sys::blas::level1::ccopy`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `CX` (input), `INCX` (input), `CY` (input-output), `INCY` (input)

## `CDCDOT`

- Canonical path: `slatec_sys::blas::level1::cdcdot`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `CB` (input), `CX` (input), `INCX` (input), `CY` (input), `INCY` (input)

## `CFFTB1`

- Canonical path: `slatec_sys::fftpack::cfftb1`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `C` (input-output), `CH` (input), `WA` (input), `IFAC` (input)

## `CFFTF1`

- Canonical path: `slatec_sys::fftpack::cfftf1`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `C` (input-output), `CH` (input), `WA` (input), `IFAC` (input)

## `CFFTI1`

- Canonical path: `slatec_sys::fftpack::cffti1`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `WA` (output), `IFAC` (output)

## `CG`

- Canonical path: `slatec_sys::linear_algebra::eigen::cg`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `AR` (input), `AI` (input), `WR` (output), `WI` (output), `MATZ` (input), `ZR` (output), `ZI` (output), `FV1` (output), `FV2` (output), `FV3` (output), `IERR` (output)

## `CGBCO`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cgbco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `RCOND` (output), `Z` (output)

## `CGBDI`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cgbdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input), `LDA` (input), `N` (input), `ML` (input), `MU` (input), `IPVT` (input), `DET` (output)

## `CGBFA`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cgbfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `INFO` (status-output)

## `CGBMV`

- Canonical path: `slatec_sys::blas::level2::cgbmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `TRANS` (input), `M` (input), `N` (input), `KL` (input), `KU` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `CGECO`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cgeco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `IPVT` (output), `RCOND` (output), `Z` (output)

## `CGEDI`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cgedi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `IPVT` (input), `DET` (output), `WORK` (workspace-output), `JOB` (input)

## `CGEEV`

- Canonical path: `slatec_sys::linear_algebra::eigen::cgeev`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `E` (output), `V` (input-output), `LDV` (input), `WORK` (workspace-output), `JOB` (input), `INFO` (status-output)

## `CGEFA`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cgefa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `IPVT` (output), `INFO` (status-output)

## `CGEFS`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cgefs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (output), `LDA` (input), `N` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output), `IWORK` (workspace-output)

## `CGEIR`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cgeir`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `LDA` (input), `N` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output), `IWORK` (workspace-output)

## `CGEMM`

- Canonical path: `slatec_sys::blas::level3::cgemm`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `TRANSA` (input), `TRANSB` (input), `M` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input), `C` (input), `LDC` (input)

## `CGEMV`

- Canonical path: `slatec_sys::blas::level2::cgemv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `TRANS` (input), `M` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `CGERC`

- Canonical path: `slatec_sys::blas::level2::cgerc`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `M` (input), `N` (input), `ALPHA` (input), `X` (input), `INCX` (input), `Y` (input), `INCY` (input), `A` (input), `LDA` (input)

## `CGERU`

- Canonical path: `slatec_sys::blas::level2::cgeru`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `M` (input), `N` (input), `ALPHA` (input), `X` (input), `INCX` (input), `Y` (input), `INCY` (input), `A` (input), `LDA` (input)

## `CGTSL`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cgtsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `C` (input), `D` (input), `E` (input), `B` (input-output), `INFO` (status-output)

## `CH`

- Canonical path: `slatec_sys::linear_algebra::eigen::ch`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `AR` (input), `AI` (input), `W` (output), `MATZ` (input), `ZR` (output), `ZI` (output), `FV1` (output), `FV2` (output), `FM1` (output), `IERR` (output)

## `CHBMV`

- Canonical path: `slatec_sys::blas::level2::chbmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `CHEMM`

- Canonical path: `slatec_sys::blas::level3::chemm`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `SIDE` (input), `UPLO` (input), `M` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input), `C` (input), `LDC` (input)

## `CHEMV`

- Canonical path: `slatec_sys::blas::level2::chemv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `CHER`

- Canonical path: `slatec_sys::blas::level2::cher`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `X` (input), `INCX` (input), `A` (input), `LDA` (input)

## `CHER2`

- Canonical path: `slatec_sys::blas::level2::cher2`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `X` (input), `INCX` (input), `Y` (input), `INCY` (input), `A` (input), `LDA` (input)

## `CHER2K`

- Canonical path: `slatec_sys::blas::level3::cher2k`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input-output), `C` (input-output), `LDC` (input)

## `CHERK`

- Canonical path: `slatec_sys::blas::level3::cherk`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `BETA` (input), `C` (input), `LDC` (input)

## `CHFDV`

- Canonical path: `slatec_sys::interpolation::chfdv`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X1` (input), `X2` (input), `F1` (input), `F2` (input), `D1` (input), `D2` (input), `NE` (input), `XE` (input), `FE` (output), `DE` (output), `NEXT` (output), `IERR` (output)

## `CHFEV`

- Canonical path: `slatec_sys::interpolation::chfev`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X1` (input), `X2` (input), `F1` (input), `F2` (input), `D1` (input), `D2` (input), `NE` (input), `XE` (input), `FE` (output), `NEXT` (output), `IERR` (output)

## `CHICO`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::chico`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (output), `RCOND` (output), `Z` (output)

## `CHIDI`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::chidi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (input), `DET` (output), `INERT` (output), `WORK` (workspace-output), `JOB` (input)

## `CHIEV`

- Canonical path: `slatec_sys::linear_algebra::eigen::chiev`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `E` (output), `V` (output), `LDV` (input), `WORK` (workspace-output), `JOB` (input), `INFO` (status-output)

## `CHIFA`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::chifa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (output), `INFO` (status-output)

## `CHKDER`

- Canonical path: `slatec_sys::nonlinear::jacobian_check::chkder`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `M` (input), `N` (input), `X` (input), `FVEC` (input), `FJAC` (input), `LDFJAC` (input), `XP` (input), `FVECP` (input), `MODE` (input), `ERR` (input)

## `CHPCO`

- Canonical path: `slatec_sys::linear_algebra::packed::complex::chpco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (output), `RCOND` (output), `Z` (output)

## `CHPDI`

- Canonical path: `slatec_sys::linear_algebra::packed::complex::chpdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (input), `DET` (output), `INERT` (output), `WORK` (workspace-output), `JOB` (input)

## `CHPFA`

- Canonical path: `slatec_sys::linear_algebra::packed::complex::chpfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (output), `INFO` (status-output)

## `CHPMV`

- Canonical path: `slatec_sys::blas::level2::chpmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `AP` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `CINVIT`

- Canonical path: `slatec_sys::linear_algebra::eigen::cinvit`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `AR` (input-output), `AI` (input-output), `WR` (input), `WI` (input-output), `SELECT` (input-output), `MM` (input), `M` (output), `ZR` (output), `ZI` (output), `IERR` (output), `RM1` (output), `RM2` (output), `RV1` (output), `RV2` (output)

## `CMGNBN`

- Canonical path: `slatec_sys::pde::fishpack::complex::cmgnbn`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NPEROD` (input), `N` (input), `MPEROD` (input), `M` (input), `A` (input), `B` (input), `C` (input), `IDIMY` (input), `Y` (input-output), `IERROR` (status-output), `W` (input-output)

## `CNBCO`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cnbco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `RCOND` (output), `Z` (output)

## `CNBDI`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cnbdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input), `LDA` (input), `N` (input), `ML` (input), `MU` (input), `IPVT` (input), `DET` (output)

## `CNBFA`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cnbfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `INFO` (status-output)

## `CNBFS`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cnbfs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (output), `LDA` (input), `N` (input), `ML` (input), `MU` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output), `IWORK` (workspace-output)

## `CNBIR`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cnbir`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input), `LDA` (input), `N` (input-output), `ML` (input), `MU` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output), `IWORK` (workspace-output)

## `COMHES`

- Canonical path: `slatec_sys::linear_algebra::eigen::comhes`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input-output), `AR` (input-output), `AI` (input-output), `INT` (output)

## `COMLR`

- Canonical path: `slatec_sys::linear_algebra::eigen::comlr`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `HR` (input), `HI` (input), `WR` (output), `WI` (output), `IERR` (output)

## `COMLR2`

- Canonical path: `slatec_sys::linear_algebra::eigen::comlr2`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `INT` (input), `HR` (input), `HI` (input), `WR` (output), `WI` (output), `ZR` (output), `ZI` (output), `IERR` (output)

## `COMQR`

- Canonical path: `slatec_sys::linear_algebra::eigen::comqr`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `HR` (input), `HI` (input), `WR` (output), `WI` (output), `IERR` (output)

## `COMQR2`

- Canonical path: `slatec_sys::linear_algebra::eigen::comqr2`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `ORTR` (input), `ORTI` (input), `HR` (input), `HI` (input), `WR` (output), `WI` (output), `ZR` (output), `ZI` (output), `IERR` (output)

## `CORTB`

- Canonical path: `slatec_sys::linear_algebra::eigen::cortb`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `LOW` (input), `IGH` (input), `AR` (input), `AI` (input), `ORTR` (input), `ORTI` (input), `M` (input), `ZR` (input-output), `ZI` (input-output)

## `CORTH`

- Canonical path: `slatec_sys::linear_algebra::eigen::corth`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `AR` (input-output), `AI` (input-output), `ORTR` (output), `ORTI` (output)

## `COSQB`

- Canonical path: `slatec_sys::fftpack::cosqb`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input-output), `WSAVE` (workspace-output)

## `COSQF`

- Canonical path: `slatec_sys::fftpack::cosqf`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input-output), `WSAVE` (workspace-output)

## `COSQI`

- Canonical path: `slatec_sys::fftpack::cosqi`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `WSAVE` (workspace-output)

## `COST`

- Canonical path: `slatec_sys::fftpack::cost`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input-output), `WSAVE` (workspace-output)

## `COSTI`

- Canonical path: `slatec_sys::fftpack::costi`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `WSAVE` (workspace-output)

## `CPBCO`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cpbco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `M` (input-output), `RCOND` (output), `Z` (output), `INFO` (status-output)

## `CPBDI`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cpbdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input), `LDA` (input), `N` (input), `M` (input), `DET` (output)

## `CPBFA`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cpbfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `M` (input-output), `INFO` (status-output)

## `CPOCO`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cpoco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `RCOND` (output), `Z` (output), `INFO` (status-output)

## `CPODI`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cpodi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `DET` (output), `JOB` (input)

## `CPOFS`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cpofs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (output), `LDA` (input), `N` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output)

## `CPOIR`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cpoir`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `LDA` (input), `N` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output)

## `CPPCO`

- Canonical path: `slatec_sys::linear_algebra::packed::complex::cppco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `RCOND` (output), `Z` (output), `INFO` (status-output)

## `CPPDI`

- Canonical path: `slatec_sys::linear_algebra::packed::complex::cppdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `DET` (output), `JOB` (input)

## `CPQR79`

- Canonical path: `slatec_sys::roots::complex::cpqr79`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NDEG` (input), `COEFF` (input), `ROOT` (output), `IERR` (output), `WORK` (workspace-output)

## `CPSI`

- Canonical path: `slatec_sys::special::complex::cpsi`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ZIN` (input)

## `CPTSL`

- Canonical path: `slatec_sys::linear_algebra::banded::complex::cptsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `D` (input), `E` (input), `B` (input-output)

## `CPZERO`

- Canonical path: `slatec_sys::roots::complex::cpzero`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `IN` (input), `A` (input), `R` (input-output), `T` (input), `IFLG` (output), `S` (input-output)

## `CQRDC`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cqrdc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `LDX` (input), `N` (input), `P` (input), `QRAUX` (output), `JPVT` (input-output), `WORK` (workspace-output), `JOB` (input)

## `CQRSL`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::cqrsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `LDX` (input), `N` (input), `K` (input), `QRAUX` (input), `Y` (input), `QY` (output), `QTY` (output), `B` (output), `RSD` (output), `XB` (output), `JOB` (input), `INFO` (status-output)

## `CSICO`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::csico`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (output), `RCOND` (output), `Z` (output)

## `CSIDI`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::csidi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (input), `DET` (output), `WORK` (workspace-output), `JOB` (input)

## `CSIFA`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::csifa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (output), `INFO` (status-output)

## `CSPCO`

- Canonical path: `slatec_sys::linear_algebra::packed::complex::cspco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (output), `RCOND` (output), `Z` (output)

## `CSPDI`

- Canonical path: `slatec_sys::linear_algebra::packed::complex::cspdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (input), `DET` (output), `WORK` (workspace-output), `JOB` (input)

## `CSPFA`

- Canonical path: `slatec_sys::linear_algebra::packed::complex::cspfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (output), `INFO` (status-output)

## `CSROT`

- Canonical path: `slatec_sys::blas::level1::csrot`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `CX` (input), `INCX` (input), `CY` (input), `INCY` (input), `C` (input), `S` (input)

## `CSVDC`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::csvdc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `LDX` (input), `N` (input), `P` (input), `S` (output), `E` (output), `U` (output), `LDU` (input), `V` (output), `LDV` (input), `WORK` (workspace-output), `JOB` (input), `INFO` (status-output)

## `CSYMM`

- Canonical path: `slatec_sys::blas::level3::csymm`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `SIDE` (input), `UPLO` (input), `M` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input), `C` (input), `LDC` (input)

## `CSYR2K`

- Canonical path: `slatec_sys::blas::level3::csyr2k`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input-output), `C` (input-output), `LDC` (input)

## `CSYRK`

- Canonical path: `slatec_sys::blas::level3::csyrk`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `BETA` (input), `C` (input), `LDC` (input)

## `CTBMV`

- Canonical path: `slatec_sys::blas::level2::ctbmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `K` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input)

## `CTBSV`

- Canonical path: `slatec_sys::blas::level2::ctbsv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `K` (input), `A` (input-output), `LDA` (input), `X` (input-output), `INCX` (input)

## `CTPMV`

- Canonical path: `slatec_sys::blas::level2::ctpmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `AP` (input), `X` (input), `INCX` (input)

## `CTRCO`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::ctrco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `LDT` (input), `N` (input), `RCOND` (output), `Z` (output), `JOB` (input)

## `CTRDI`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::ctrdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input-output), `LDT` (input), `N` (input), `DET` (output), `JOB` (input), `INFO` (status-output)

## `CTRMM`

- Canonical path: `slatec_sys::blas::level3::ctrmm`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `SIDE` (input), `UPLO` (input), `TRANSA` (input), `DIAG` (input), `M` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input)

## `CTRMV`

- Canonical path: `slatec_sys::blas::level2::ctrmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input)

## `CTRSL`

- Canonical path: `slatec_sys::linear_algebra::dense::complex::ctrsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `LDT` (input), `N` (input), `B` (input-output), `JOB` (input), `INFO` (status-output)

## `CTRSV`

- Canonical path: `slatec_sys::blas::level2::ctrsv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `A` (input-output), `LDA` (input), `X` (input-output), `INCX` (input)

## `CV`

- Canonical path: `slatec_sys::statistics::cv`
- Family: `Probability and statistics`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XVAL` (input), `NDATA` (input), `NCONST` (input), `NORD` (input), `NBKPT` (input), `BKPT` (input), `W` (input-output)

## `DAVINT`

- Canonical path: `slatec_sys::quadrature::davint`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `Y` (input), `N` (input), `XLO` (input), `XUP` (input), `ANS` (output), `IERR` (output)

## `DBCG`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::dbcg`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MTTVEC` (callback), `MSOLVE` (callback), `MTSOLV` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `Z` (workspace-output), `P` (workspace-output), `RR` (workspace-output), `ZZ` (workspace-output), `PP` (workspace-output), `DZ` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DBESKS`

- Canonical path: `slatec_sys::special::bessel::dbesks`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XNU` (input), `X` (input), `NIN` (input), `BK` (input-output)

## `DBESY`

- Canonical path: `slatec_sys::special::bessel::dbesy`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `FNU` (input), `N` (input), `Y` (output)

## `DBINT4`

- Canonical path: `slatec_sys::interpolation::dbint4`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `Y` (input), `NDATA` (input), `IBCL` (input), `IBCR` (input), `FBCL` (input), `FBCR` (input), `KNTOPT` (input), `T` (input), `BCOEF` (output), `N` (output), `K` (output), `W` (workspace-output)

## `DBNDAC`

- Canonical path: `slatec_sys::linear_algebra::banded::dbndac`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `G` (input-output), `MDG` (input-output), `NB` (input), `IP` (input-output), `IR` (input-output), `MT` (input), `JT` (input-output)

## `DBNDSL`

- Canonical path: `slatec_sys::linear_algebra::banded::dbndsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `MODE` (input), `G` (input-output), `MDG` (input-output), `NB` (input), `IP` (input-output), `IR` (input-output), `X` (input-output), `N` (input), `RNORM` (output)

## `DBOCLS`

- Canonical path: `slatec_sys::approximation::dbocls`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `W` (input-output), `MDW` (input), `MCON` (input-output), `MROWS` (input-output), `NCOLS` (input-output), `BL` (input-output), `BU` (input-output), `IND` (input), `IOPT` (input-output), `X` (input-output), `RNORMC` (input-output), `RNORM` (input-output), `MODE` (output), `RW` (input-output), `IW` (input-output)

## `DBOLS`

- Canonical path: `slatec_sys::approximation::dbols`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `W` (input-output), `MDW` (input), `MROWS` (input-output), `NCOLS` (input-output), `BL` (input-output), `BU` (input-output), `IND` (input), `IOPT` (input-output), `X` (input-output), `RNORM` (input-output), `MODE` (output), `RW` (input-output), `IW` (input-output)

## `DBSK1E`

- Canonical path: `slatec_sys::special::dbsk1e`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input)

## `DBSKES`

- Canonical path: `slatec_sys::special::dbskes`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XNU` (input), `X` (input), `NIN` (input), `BKE` (input)

## `DBSKIN`

- Canonical path: `slatec_sys::special::dbskin`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `N` (input), `KODE` (input), `M` (input), `Y` (input), `NZ` (input), `IERR` (input)

## `DBSPPP`

- Canonical path: `slatec_sys::interpolation::dbsppp`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `A` (input), `N` (input), `K` (input), `LDC` (input), `C` (output), `XI` (input), `LXI` (output), `WORK` (workspace-output)

## `DCDOT`

- Canonical path: `slatec_sys::blas::level1::dcdot`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `FM` (input), `CX` (input), `INCX` (input), `CY` (input), `INCY` (input), `DCR` (output), `DCI` (output)

## `DCG`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::dcg`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MSOLVE` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `Z` (workspace-output), `P` (workspace-output), `DZ` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DCGN`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::dcgn`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MTTVEC` (callback), `MSOLVE` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `Z` (workspace-output), `P` (workspace-output), `ATP` (workspace-output), `ATZ` (workspace-output), `DZ` (workspace-output), `ATDZ` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DCGS`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::dcgs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MSOLVE` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `R0` (workspace-output), `P` (workspace-output), `Q` (workspace-output), `U` (workspace-output), `V1` (workspace-output), `V2` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DCHDC`

- Canonical path: `slatec_sys::linear_algebra::dense::dchdc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `P` (input), `WORK` (workspace-output), `JPVT` (input-output), `JOB` (input), `INFO` (status-output)

## `DCHDD`

- Canonical path: `slatec_sys::linear_algebra::dense::dchdd`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `R` (input-output), `LDR` (input), `P` (input), `X` (input), `Z` (input-output), `LDZ` (input), `NZ` (input), `Y` (input), `RHO` (input-output), `C` (output), `S` (output), `INFO` (status-output)

## `DCHEX`

- Canonical path: `slatec_sys::linear_algebra::dense::dchex`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `R` (input-output), `LDR` (input), `P` (input), `K` (input), `L` (input), `Z` (input-output), `LDZ` (input), `NZ` (input), `C` (output), `S` (output), `JOB` (input)

## `DCHFDV`

- Canonical path: `slatec_sys::interpolation::dchfdv`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X1` (input), `X2` (input), `F1` (input), `F2` (input), `D1` (input), `D2` (input), `NE` (input), `XE` (input), `FE` (output), `DE` (output), `NEXT` (output), `IERR` (output)

## `DCHFEV`

- Canonical path: `slatec_sys::interpolation::dchfev`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X1` (input), `X2` (input), `F1` (input), `F2` (input), `D1` (input), `D2` (input), `NE` (input), `XE` (input), `FE` (output), `NEXT` (output), `IERR` (output)

## `DCHUD`

- Canonical path: `slatec_sys::linear_algebra::dense::dchud`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `R` (input), `LDR` (input), `P` (input), `X` (input), `Z` (input-output), `LDZ` (input), `NZ` (input), `Y` (input), `RHO` (input-output), `C` (output), `S` (output)

## `DCKDER`

- Canonical path: `slatec_sys::nonlinear::jacobian_check::dckder`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `M` (input), `N` (input), `X` (input), `FVEC` (input), `FJAC` (input), `LDFJAC` (input), `XP` (input), `FVECP` (input), `MODE` (input), `ERR` (input)

## `DCOPY`

- Canonical path: `slatec_sys::blas::level1::dcopy`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `DX` (input), `INCX` (input), `DY` (input-output), `INCY` (input)

## `DCOPYM`

- Canonical path: `slatec_sys::blas::level1::dcopym`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `DX` (input), `INCX` (input), `DY` (input-output), `INCY` (input)

## `DCOV`

- Canonical path: `slatec_sys::least_squares::dcov`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FCN` (callback), `IOPT` (input), `M` (input), `N` (input), `X` (input), `FVEC` (input), `R` (input), `LDR` (input), `INFO` (input), `WA1` (input), `WA2` (input), `WA3` (input), `WA4` (input)

## `DCV`

- Canonical path: `slatec_sys::statistics::dcv`
- Family: `Probability and statistics`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XVAL` (input), `NDATA` (input), `NCONST` (input), `NORD` (input), `NBKPT` (input), `BKPT` (input), `W` (input-output)

## `DDASSL`

- Canonical path: `slatec_sys::dassl::ddassl`
- Family: `ODE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `RES` (callback), `NEQ` (input), `T` (input-output), `Y` (input-output), `YPRIME` (input-output), `TOUT` (input), `INFO` (input), `RTOL` (input-output), `ATOL` (input-output), `IDID` (output), `RWORK` (workspace-output), `LRW` (input), `IWORK` (workspace-output), `LIW` (input), `RPAR` (input), `IPAR` (input), `JAC` (callback)

## `DDEABM`

- Canonical path: `slatec_sys::ode::callbacks::ddeabm`
- Family: `ODE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `DF` (callback), `NEQ` (input), `T` (input), `Y` (input-output), `TOUT` (input), `INFO` (input), `RTOL` (input), `ATOL` (input), `IDID` (input), `RWORK` (workspace-output), `LRW` (input-output), `IWORK` (workspace-output), `LIW` (input-output), `RPAR` (input), `IPAR` (input)

## `DDERKF`

- Canonical path: `slatec_sys::ode::callbacks::dderkf`
- Family: `ODE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `DF` (callback), `NEQ` (input), `T` (input), `Y` (input-output), `TOUT` (input), `INFO` (input), `RTOL` (input), `ATOL` (input), `IDID` (input), `RWORK` (workspace-output), `LRW` (input-output), `IWORK` (workspace-output), `LIW` (input-output), `RPAR` (input), `IPAR` (input)

## `DDRIV3`

- Canonical path: `slatec_sys::ode::ddriv3`
- Family: `ODE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `T` (input-output), `Y` (input-output), `F` (callback), `NSTATE` (input-output), `TOUT` (input-output), `NTASK` (input), `NROOT` (input), `EPS` (input-output), `EWT` (input), `IERROR` (input), `MINT` (input), `MITER` (input), `IMPL` (input), `ML` (input), `MU` (input), `MXORD` (input), `HMAX` (input), `WORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input), `JACOBN` (callback), `FA` (callback), `NDE` (input), `MXSTEP` (input), `G` (callback), `USERS` (callback), `IERFLG` (input-output)

## `DEFC`

- Canonical path: `slatec_sys::approximation::defc`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NDATA` (input-output), `XDATA` (input-output), `YDATA` (input-output), `SDDATA` (input-output), `NORD` (input-output), `NBKPT` (input-output), `BKPT` (input-output), `MDEIN` (input), `MDEOUT` (output), `COEFF` (output), `LW` (input), `W` (workspace-output)

## `DERKF`

- Canonical path: `slatec_sys::ode::callbacks::derkf`
- Family: `ODE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `NEQ` (input), `T` (input), `Y` (input-output), `TOUT` (input), `INFO` (input), `RTOL` (input), `ATOL` (input), `IDID` (input), `RWORK` (workspace-output), `LRW` (input-output), `IWORK` (workspace-output), `LIW` (input-output), `RPAR` (input), `IPAR` (input)

## `DFC`

- Canonical path: `slatec_sys::approximation::dfc`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NDATA` (input-output), `XDATA` (input-output), `YDATA` (input-output), `SDDATA` (input-output), `NORD` (input-output), `NBKPT` (input-output), `BKPT` (input-output), `NCONST` (input-output), `XCONST` (input-output), `YCONST` (input-output), `NDERIV` (input-output), `MODE` (input-output), `COEFF` (input-output), `W` (input-output), `IW` (input-output)

## `DFZERO`

- Canonical path: `slatec_sys::roots::scalar::dfzero`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `B` (input-output), `C` (input-output), `R` (input), `RE` (input), `AE` (input), `IFLAG` (status-output)

## `DGAMLM`

- Canonical path: `slatec_sys::special::dgamlm`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XMIN` (input), `XMAX` (input)

## `DGBCO`

- Canonical path: `slatec_sys::linear_algebra::banded::dgbco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `RCOND` (output), `Z` (output)

## `DGBDI`

- Canonical path: `slatec_sys::linear_algebra::banded::dgbdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input), `LDA` (input), `N` (input), `ML` (input), `MU` (input), `IPVT` (input), `DET` (output)

## `DGBFA`

- Canonical path: `slatec_sys::linear_algebra::banded::dgbfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `INFO` (status-output)

## `DGBMV`

- Canonical path: `slatec_sys::blas::level2::dgbmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `TRANS` (input), `M` (input), `N` (input), `KL` (input), `KU` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `DGECO`

- Canonical path: `slatec_sys::linear_algebra::dense::dgeco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `IPVT` (output), `RCOND` (output), `Z` (output)

## `DGEDI`

- Canonical path: `slatec_sys::linear_algebra::dense::dgedi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `IPVT` (input), `DET` (output), `WORK` (workspace-output), `JOB` (input)

## `DGEFA`

- Canonical path: `slatec_sys::linear_algebra::dense::dgefa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `IPVT` (output), `INFO` (status-output)

## `DGEFS`

- Canonical path: `slatec_sys::linear_algebra::dense::dgefs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (output), `LDA` (input), `N` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output), `IWORK` (workspace-output)

## `DGEMM`

- Canonical path: `slatec_sys::blas::level3::dgemm`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `TRANSA` (input), `TRANSB` (input), `M` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input), `C` (input), `LDC` (input)

## `DGEMV`

- Canonical path: `slatec_sys::blas::level2::dgemv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `TRANS` (input), `M` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `DGER`

- Canonical path: `slatec_sys::blas::level2::dger`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `M` (input), `N` (input), `ALPHA` (input), `X` (input), `INCX` (input), `Y` (input), `INCY` (input), `A` (input), `LDA` (input)

## `DGLSS`

- Canonical path: `slatec_sys::linear_algebra::dense::dglss`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `MDA` (input), `M` (input), `N` (input), `B` (input-output), `MDB` (input), `NB` (input), `RNORM` (input-output), `WORK` (workspace-output), `LW` (input), `IWORK` (workspace-output), `LIW` (input), `INFO` (status-output)

## `DGMRES`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::dgmres`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input-output), `ISYM` (input), `MATVEC` (callback), `MSOLVE` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `SB` (input), `SX` (input), `RGWK` (output), `LRGW` (input), `IGWK` (input-output), `LIGW` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DGTSL`

- Canonical path: `slatec_sys::linear_algebra::banded::dgtsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `C` (input), `D` (input), `E` (input), `B` (input-output), `INFO` (status-output)

## `DHFTI`

- Canonical path: `slatec_sys::linear_algebra::banded::dhfti`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `MDA` (input-output), `M` (input-output), `N` (input-output), `B` (output), `MDB` (input-output), `NB` (input-output), `TAU` (input), `KRANK` (output), `RNORM` (output), `H` (output), `G` (output), `IP` (input-output)

## `DINTP`

- Canonical path: `slatec_sys::ode::dintp`
- Family: `ODE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `Y` (input), `XOUT` (input), `YOUT` (input-output), `YPOUT` (input-output), `NEQN` (input), `KOLD` (input), `PHI` (input), `IVC` (input), `IV` (input), `KGI` (input), `GI` (input), `ALPHA` (input), `OG` (input), `OW` (input), `OX` (input), `OY` (input)

## `DINTRV`

- Canonical path: `slatec_sys::interpolation::dintrv`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XT` (input), `LXT` (input), `X` (input), `ILO` (output), `ILEFT` (output), `MFLAG` (output)

## `DIR`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::dir`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MSOLVE` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `Z` (workspace-output), `DZ` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DLGAMS`

- Canonical path: `slatec_sys::special::dlgams`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `DLGAM` (output), `SGNGAM` (output)

## `DLLSIA`

- Canonical path: `slatec_sys::linear_algebra::dense::dllsia`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `MDA` (input), `M` (input), `N` (input), `B` (input-output), `MDB` (input), `NB` (input), `RE` (input), `AE` (input), `KEY` (input), `MODE` (input), `NP` (input), `KRANK` (output), `KSURE` (output), `RNORM` (output), `W` (input-output), `LW` (input), `IWORK` (workspace-output), `LIW` (input), `INFO` (status-output)

## `DLLTI2`

- Canonical path: `slatec_sys::linear_algebra::dense::dllti2`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (output), `NEL` (input), `IEL` (input), `JEL` (input), `EL` (input), `DINV` (input)

## `DLSEI`

- Canonical path: `slatec_sys::approximation::dlsei`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `W` (input-output), `MDW` (input-output), `ME` (input), `MA` (input), `MG` (input), `N` (input), `PRGOPT` (input-output), `X` (input-output), `RNORME` (input-output), `RNORML` (output), `MODE` (output), `WS` (input-output), `IP` (input-output)

## `DNBCO`

- Canonical path: `slatec_sys::linear_algebra::banded::dnbco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `RCOND` (output), `Z` (output)

## `DNBDI`

- Canonical path: `slatec_sys::linear_algebra::banded::dnbdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input), `LDA` (input), `N` (input), `ML` (input), `MU` (input), `IPVT` (input), `DET` (output)

## `DNBFA`

- Canonical path: `slatec_sys::linear_algebra::banded::dnbfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `INFO` (status-output)

## `DNBFS`

- Canonical path: `slatec_sys::linear_algebra::banded::dnbfs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (output), `LDA` (input), `N` (input), `ML` (input), `MU` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output), `IWORK` (workspace-output)

## `DNLS1`

- Canonical path: `slatec_sys::least_squares::dnls1`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FCN` (callback), `IOPT` (input), `M` (input), `N` (input), `X` (input-output), `FVEC` (input-output), `FJAC` (input-output), `LDFJAC` (input), `FTOL` (input), `XTOL` (input), `GTOL` (input), `MAXFEV` (input), `EPSFCN` (input), `DIAG` (input), `MODE` (input), `FACTOR` (input), `NPRINT` (input), `INFO` (input), `NFEV` (input-output), `NJEV` (input-output), `IPVT` (input-output), `QTF` (input-output), `WA1` (workspace-output), `WA2` (workspace-output), `WA3` (workspace-output), `WA4` (input)

## `DNLS1E`

- Canonical path: `slatec_sys::least_squares::dnls1e`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FCN` (callback), `IOPT` (input), `M` (input), `N` (input), `X` (input), `FVEC` (input), `TOL` (input), `NPRINT` (input), `INFO` (input), `IW` (input), `WA` (input), `LWA` (input)

## `DNSQ`

- Canonical path: `slatec_sys::nonlinear::dnsq`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FCN` (callback), `JAC` (input), `IOPT` (input), `N` (input), `X` (input), `FVEC` (input), `FJAC` (input), `LDFJAC` (input), `XTOL` (input), `MAXFEV` (input), `ML` (input), `MU` (input), `EPSFCN` (input), `DIAG` (input), `MODE` (input), `FACTOR` (input), `NPRINT` (input), `INFO` (input), `NFEV` (input), `NJEV` (input), `R` (input), `LR` (input), `QTF` (input), `WA1` (input), `WA2` (input), `WA3` (input), `WA4` (input)

## `DNSQE`

- Canonical path: `slatec_sys::nonlinear::dnsqe`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FCN` (callback), `JAC` (callback), `IOPT` (input), `N` (input), `X` (input), `FVEC` (input), `TOL` (input), `NPRINT` (input), `INFO` (input-output), `WA` (input), `LWA` (input)

## `DOMN`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::domn`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MSOLVE` (callback), `NSAVE` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `Z` (workspace-output), `P` (workspace-output), `AP` (workspace-output), `EMAP` (workspace-output), `DZ` (workspace-output), `CSAV` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DP1VLU`

- Canonical path: `slatec_sys::approximation::dp1vlu`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `L` (input), `NDER` (input), `X` (input), `YFIT` (output), `YP` (output), `A` (input)

## `DPBCO`

- Canonical path: `slatec_sys::linear_algebra::banded::dpbco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `M` (input-output), `RCOND` (output), `Z` (output), `INFO` (status-output)

## `DPBDI`

- Canonical path: `slatec_sys::linear_algebra::banded::dpbdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input), `LDA` (input), `N` (input), `M` (input), `DET` (output)

## `DPBFA`

- Canonical path: `slatec_sys::linear_algebra::banded::dpbfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `M` (input-output), `INFO` (status-output)

## `DPCHBS`

- Canonical path: `slatec_sys::interpolation::dpchbs`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input-output), `D` (input-output), `INCFD` (input), `KNOTYP` (input), `NKNOTS` (input-output), `T` (input-output), `BCOEF` (output), `NDIM` (output), `KORD` (output), `IERR` (output)

## `DPCHCM`

- Canonical path: `slatec_sys::interpolation::dpchcm`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (input), `INCFD` (input), `SKIP` (input-output), `ISMON` (output), `IERR` (output)

## `DPCHFD`

- Canonical path: `slatec_sys::interpolation::dpchfd`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (input), `INCFD` (input), `SKIP` (input-output), `NE` (input), `XE` (input), `FE` (output), `DE` (output), `IERR` (output)

## `DPCHFE`

- Canonical path: `slatec_sys::interpolation::dpchfe`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (input), `INCFD` (input), `SKIP` (input-output), `NE` (input), `XE` (input), `FE` (output), `IERR` (output)

## `DPCHIA`

- Canonical path: `slatec_sys::interpolation::dpchia`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (input), `INCFD` (input), `SKIP` (output), `A` (input), `B` (input), `IERR` (output)

## `DPCHIC`

- Canonical path: `slatec_sys::interpolation::dpchic`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `IC` (input), `VC` (input), `SWITCH` (input), `N` (input), `X` (input), `F` (input), `D` (output), `INCFD` (input), `WK` (input), `NWK` (input), `IERR` (output)

## `DPCHID`

- Canonical path: `slatec_sys::interpolation::dpchid`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (input), `INCFD` (input), `SKIP` (output), `IA` (input), `IB` (input), `IERR` (output)

## `DPCHIM`

- Canonical path: `slatec_sys::interpolation::dpchim`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (output), `INCFD` (input), `IERR` (output)

## `DPCHSP`

- Canonical path: `slatec_sys::interpolation::dpchsp`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `IC` (input), `VC` (input), `N` (input), `X` (input), `F` (input), `D` (output), `INCFD` (input), `WK` (input-output), `NWK` (input), `IERR` (output)

## `DPCOEF`

- Canonical path: `slatec_sys::approximation::dpcoef`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `L` (input), `C` (input), `TC` (output), `A` (input)

## `DPOCH1`

- Canonical path: `slatec_sys::special::dpoch1`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `X` (input-output)

## `DPOCO`

- Canonical path: `slatec_sys::linear_algebra::dense::dpoco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `RCOND` (output), `Z` (output), `INFO` (status-output)

## `DPODI`

- Canonical path: `slatec_sys::linear_algebra::dense::dpodi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `DET` (output), `JOB` (input)

## `DPOFS`

- Canonical path: `slatec_sys::linear_algebra::dense::dpofs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (output), `LDA` (input), `N` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output)

## `DPOLCF`

- Canonical path: `slatec_sys::interpolation::dpolcf`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XX` (input), `N` (input), `X` (input), `C` (input), `D` (input-output), `WORK` (workspace-output)

## `DPOLFT`

- Canonical path: `slatec_sys::approximation::dpolft`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input-output), `X` (input), `Y` (input), `W` (input-output), `MAXDEG` (input-output), `NDEG` (output), `EPS` (input-output), `R` (output), `IERR` (output), `A` (output)

## `DPOLVL`

- Canonical path: `slatec_sys::interpolation::dpolvl`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NDER` (input), `XX` (input), `YFIT` (output), `YP` (input-output), `N` (input), `X` (input), `C` (input), `WORK` (workspace-output), `IERR` (output)

## `DPPCO`

- Canonical path: `slatec_sys::linear_algebra::packed::dppco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `RCOND` (output), `Z` (output), `INFO` (status-output)

## `DPPDI`

- Canonical path: `slatec_sys::linear_algebra::packed::dppdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `DET` (output), `JOB` (input)

## `DPPQAD`

- Canonical path: `slatec_sys::quadrature::dppqad`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `LDC` (input), `C` (input), `XI` (input-output), `LXI` (input), `K` (input), `X1` (input), `X2` (input), `PQUAD` (output)

## `DPSIFN`

- Canonical path: `slatec_sys::special::dpsifn`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `N` (input), `KODE` (input), `M` (input), `ANS` (input), `NZ` (input), `IERR` (input)

## `DPTSL`

- Canonical path: `slatec_sys::linear_algebra::banded::dptsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `D` (input), `E` (input), `B` (input-output)

## `DQAG`

- Canonical path: `slatec_sys::quadrature::dqag`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `EPSABS` (input), `EPSREL` (input), `KEY` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMIT` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `DQAGI`

- Canonical path: `slatec_sys::quadrature::dqagi`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `BOUND` (input), `INF` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMIT` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `DQAGIE`

- Canonical path: `slatec_sys::quadrature::callbacks::dqagie`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `BOUND` (input), `INF` (input), `EPSABS` (input), `EPSREL` (input), `LIMIT` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `ALIST` (output), `BLIST` (output), `RLIST` (output), `ELIST` (output), `IORD` (output), `LAST` (output)

## `DQAGP`

- Canonical path: `slatec_sys::quadrature::dqagp`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `NPTS2` (input), `POINTS` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LENIW` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `DQAGPE`

- Canonical path: `slatec_sys::quadrature::callbacks::dqagpe`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `NPTS2` (input), `POINTS` (input), `EPSABS` (input), `EPSREL` (input), `LIMIT` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `ALIST` (output), `BLIST` (output), `RLIST` (output), `ELIST` (output), `PTS` (output), `IORD` (output), `LEVEL` (output), `NDIN` (output), `LAST` (output)

## `DQAGS`

- Canonical path: `slatec_sys::quadrature::dqags`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMIT` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `DQAGSE`

- Canonical path: `slatec_sys::quadrature::callbacks::dqagse`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `EPSABS` (input), `EPSREL` (input), `LIMIT` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `ALIST` (output), `BLIST` (output), `RLIST` (output), `ELIST` (output), `IORD` (output), `LAST` (output)

## `DQAWC`

- Canonical path: `slatec_sys::quadrature::dqawc`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `C` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMIT` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `DQAWCE`

- Canonical path: `slatec_sys::quadrature::callbacks::dqawce`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `C` (input), `EPSABS` (input), `EPSREL` (input), `LIMIT` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `ALIST` (output), `BLIST` (output), `RLIST` (output), `ELIST` (output), `IORD` (output), `LAST` (output)

## `DQAWF`

- Canonical path: `slatec_sys::quadrature::dqawf`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `OMEGA` (input), `INTEGR` (input), `EPSABS` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMLST` (input), `LST` (output), `LENIW` (input), `MAXP1` (input), `LENW` (input), `IWORK` (workspace-output), `WORK` (workspace-output)

## `DQAWO`

- Canonical path: `slatec_sys::quadrature::dqawo`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `OMEGA` (input), `INTEGR` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LENIW` (input), `MAXP1` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `DQAWS`

- Canonical path: `slatec_sys::quadrature::dqaws`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `ALFA` (input), `BETA` (input), `INTEGR` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMIT` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `DQAWSE`

- Canonical path: `slatec_sys::quadrature::callbacks::dqawse`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `ALFA` (input), `BETA` (input), `INTEGR` (input), `EPSABS` (input), `EPSREL` (input), `LIMIT` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `ALIST` (output), `BLIST` (output), `RLIST` (output), `ELIST` (output), `IORD` (output), `LAST` (output)

## `DQDOTA`

- Canonical path: `slatec_sys::blas::level1::dqdota`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `DB` (input), `QC` (input-output), `DX` (input), `INCX` (input), `DY` (input), `INCY` (input)

## `DQK15`

- Canonical path: `slatec_sys::quadrature::callbacks::dqk15`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `DQK15I`

- Canonical path: `slatec_sys::quadrature::callbacks::dqk15i`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `BOUN` (input), `INF` (input), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `DQK15W`

- Canonical path: `slatec_sys::quadrature::callbacks::dqk15w`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `W` (callback), `P1` (input), `P2` (input), `P3` (input), `P4` (input), `KP` (input), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `DQK21`

- Canonical path: `slatec_sys::quadrature::callbacks::dqk21`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `DQK31`

- Canonical path: `slatec_sys::quadrature::callbacks::dqk31`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `DQK41`

- Canonical path: `slatec_sys::quadrature::callbacks::dqk41`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `DQK51`

- Canonical path: `slatec_sys::quadrature::callbacks::dqk51`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `DQK61`

- Canonical path: `slatec_sys::quadrature::callbacks::dqk61`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `DQNC79`

- Canonical path: `slatec_sys::quadrature::dqnc79`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FUN` (callback), `A` (input), `B` (input), `ERR` (input), `ANS` (output), `IERR` (output), `K` (output)

## `DQNG`

- Canonical path: `slatec_sys::quadrature::dqng`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output)

## `DQRDC`

- Canonical path: `slatec_sys::linear_algebra::dense::dqrdc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `LDX` (input), `N` (input), `P` (input), `QRAUX` (output), `JPVT` (input-output), `WORK` (workspace-output), `JOB` (input)

## `DQRSL`

- Canonical path: `slatec_sys::linear_algebra::dense::dqrsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `LDX` (input), `N` (input), `K` (input), `QRAUX` (input), `Y` (input), `QY` (output), `QTY` (output), `B` (output), `RSD` (output), `XB` (output), `JOB` (input), `INFO` (status-output)

## `DRC`

- Canonical path: `slatec_sys::special::drc`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `Y` (output), `IER` (input)

## `DRC3JJ`

- Canonical path: `slatec_sys::special::drc3jj`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `L2` (input), `L3` (input), `M2` (input), `M3` (input), `L1MIN` (input-output), `L1MAX` (input-output), `THRCOF` (output), `NDIM` (input), `IER` (status-output)

## `DRC3JM`

- Canonical path: `slatec_sys::special::drc3jm`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `L1` (input), `L2` (input), `L3` (input), `M1` (input), `M2MIN` (input-output), `M2MAX` (input-output), `THRCOF` (output), `NDIM` (input), `IER` (status-output)

## `DRC6J`

- Canonical path: `slatec_sys::special::drc6j`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `L2` (input), `L3` (input), `L4` (input), `L5` (input), `L6` (input), `L1MIN` (input-output), `L1MAX` (input-output), `SIXCOF` (output), `NDIM` (input), `IER` (status-output)

## `DRD`

- Canonical path: `slatec_sys::special::drd`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `Y` (input-output), `Z` (output), `IER` (input)

## `DRF`

- Canonical path: `slatec_sys::special::drf`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `Y` (input-output), `Z` (output), `IER` (input)

## `DRJ`

- Canonical path: `slatec_sys::special::drj`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `Y` (input-output), `Z` (input-output), `P` (output), `IER` (input)

## `DROTG`

- Canonical path: `slatec_sys::blas::level1::drotg`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `DA` (input-output), `DB` (input-output), `DC` (output), `DS` (output)

## `DROTMG`

- Canonical path: `slatec_sys::blas::level1::drotmg`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `DD1` (input-output), `DD2` (input-output), `DX1` (input-output), `DY1` (input), `DPARAM` (input)

## `DS2LT`

- Canonical path: `slatec_sys::linear_algebra::dense::ds2lt`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `NEL` (output), `IEL` (output), `JEL` (output), `EL` (output)

## `DSBMV`

- Canonical path: `slatec_sys::blas::level2::dsbmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `DSD2S`

- Canonical path: `slatec_sys::linear_algebra::dense::dsd2s`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `DINV` (output)

## `DSDBCG`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsdbcg`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (output)

## `DSDCG`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsdcg`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSDCGN`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsdcgn`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSDCGS`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsdcgs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSDGMR`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsdgmr`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input-output), `ISYM` (input), `NSAVE` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSDOMN`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsdomn`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `NSAVE` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSDS`

- Canonical path: `slatec_sys::linear_algebra::dense::dsds`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `DINV` (output)

## `DSDSCL`

- Canonical path: `slatec_sys::linear_algebra::dense::dsdscl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `X` (input-output), `B` (input-output), `DINV` (output), `JOB` (input), `ITOL` (input)

## `DSGS`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsgs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSICCG`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsiccg`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSICO`

- Canonical path: `slatec_sys::linear_algebra::dense::dsico`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (output), `RCOND` (output), `Z` (output)

## `DSICS`

- Canonical path: `slatec_sys::linear_algebra::dense::dsics`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `NEL` (output), `IEL` (output), `JEL` (output), `EL` (output), `D` (output), `R` (workspace-output), `IWARN` (output)

## `DSIDI`

- Canonical path: `slatec_sys::linear_algebra::dense::dsidi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (input), `DET` (output), `INERT` (output), `WORK` (workspace-output), `JOB` (input)

## `DSIFA`

- Canonical path: `slatec_sys::linear_algebra::dense::dsifa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (output), `INFO` (status-output)

## `DSILUR`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsilur`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSILUS`

- Canonical path: `slatec_sys::linear_algebra::dense::dsilus`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `NL` (output), `IL` (output), `JL` (output), `L` (output), `DINV` (input-output), `NU` (output), `IU` (output), `JU` (output), `U` (output), `NROW` (workspace-output), `NCOL` (workspace-output)

## `DSJAC`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsjac`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSLI`

- Canonical path: `slatec_sys::linear_algebra::dense::dsli`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DSLI2`

- Canonical path: `slatec_sys::linear_algebra::dense::dsli2`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (output), `NEL` (input), `IEL` (input), `JEL` (input), `EL` (input)

## `DSLLTI`

- Canonical path: `slatec_sys::linear_algebra::dense::dsllti`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DSLUBC`

- Canonical path: `slatec_sys::linear_algebra::sparse::dslubc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSLUCN`

- Canonical path: `slatec_sys::linear_algebra::sparse::dslucn`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSLUCS`

- Canonical path: `slatec_sys::linear_algebra::dense::dslucs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSLUGM`

- Canonical path: `slatec_sys::linear_algebra::sparse::dslugm`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input-output), `ISYM` (input), `NSAVE` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSLUI`

- Canonical path: `slatec_sys::linear_algebra::dense::dslui`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DSLUI2`

- Canonical path: `slatec_sys::linear_algebra::dense::dslui2`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (output), `IL` (input), `JL` (input), `L` (input), `DINV` (input), `IU` (input), `JU` (input), `U` (input)

## `DSLUI4`

- Canonical path: `slatec_sys::linear_algebra::dense::dslui4`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (output), `IL` (input), `JL` (input), `L` (input), `DINV` (input), `IU` (input), `JU` (input), `U` (input)

## `DSLUOM`

- Canonical path: `slatec_sys::linear_algebra::sparse::dsluom`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `NSAVE` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `DSLUTI`

- Canonical path: `slatec_sys::linear_algebra::dense::dsluti`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DSMMI2`

- Canonical path: `slatec_sys::linear_algebra::dense::dsmmi2`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (output), `IL` (input), `JL` (input), `L` (input), `DINV` (input), `IU` (input), `JU` (input), `U` (input)

## `DSMMTI`

- Canonical path: `slatec_sys::linear_algebra::dense::dsmmti`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `DSMTV`

- Canonical path: `slatec_sys::blas::level1::dsmtv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `Y` (output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input)

## `DSMV`

- Canonical path: `slatec_sys::blas::level1::dsmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `Y` (output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input)

## `DSOS`

- Canonical path: `slatec_sys::nonlinear::systems::dsos`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FNC` (callback), `NEQ` (input), `X` (output), `RTOLX` (input), `ATOLX` (input), `TOLF` (input), `IFLAG` (status-output), `RW` (input), `LRW` (input), `IW` (input), `LIW` (input)

## `DSPCO`

- Canonical path: `slatec_sys::linear_algebra::packed::dspco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (output), `RCOND` (output), `Z` (output)

## `DSPDI`

- Canonical path: `slatec_sys::linear_algebra::packed::dspdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (input), `DET` (output), `INERT` (output), `WORK` (workspace-output), `JOB` (input)

## `DSPFA`

- Canonical path: `slatec_sys::linear_algebra::packed::dspfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (output), `INFO` (status-output)

## `DSPLP`

- Canonical path: `slatec_sys::linear_programming::dsplp`
- Family: `Optimization and least squares`
- Review result: `machine-semantic-audit-passed`
- Arguments: `DUSRMT` (callback), `MRELAS` (input-output), `NVARS` (input-output), `COSTS` (input-output), `PRGOPT` (input-output), `DATTRV` (input-output), `BL` (input-output), `BU` (input-output), `IND` (input), `INFO` (status-output), `PRIMAL` (input-output), `DUALS` (input-output), `IBASIS` (input-output), `WORK` (workspace-output), `LW` (input-output), `IWORK` (workspace-output), `LIW` (input-output)

## `DSPMV`

- Canonical path: `slatec_sys::blas::level2::dspmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `AP` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `DSVDC`

- Canonical path: `slatec_sys::linear_algebra::dense::dsvdc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `LDX` (input), `N` (input), `P` (input), `S` (output), `E` (output), `U` (output), `LDU` (input), `V` (output), `LDV` (input), `WORK` (workspace-output), `JOB` (input), `INFO` (status-output)

## `DSYMM`

- Canonical path: `slatec_sys::blas::level3::dsymm`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `SIDE` (input), `UPLO` (input), `M` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input), `C` (input), `LDC` (input)

## `DSYMV`

- Canonical path: `slatec_sys::blas::level2::dsymv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `DSYR`

- Canonical path: `slatec_sys::blas::level2::dsyr`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `X` (input), `INCX` (input), `A` (input), `LDA` (input)

## `DSYR2`

- Canonical path: `slatec_sys::blas::level2::dsyr2`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `X` (input), `INCX` (input), `Y` (input), `INCY` (input), `A` (input), `LDA` (input)

## `DSYR2K`

- Canonical path: `slatec_sys::blas::level3::dsyr2k`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input-output), `C` (input-output), `LDC` (input)

## `DSYRK`

- Canonical path: `slatec_sys::blas::level3::dsyrk`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `BETA` (input), `C` (input), `LDC` (input)

## `DTBMV`

- Canonical path: `slatec_sys::blas::level2::dtbmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `K` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input)

## `DTBSV`

- Canonical path: `slatec_sys::blas::level2::dtbsv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `K` (input), `A` (input-output), `LDA` (input), `X` (input-output), `INCX` (input)

## `DTPMV`

- Canonical path: `slatec_sys::blas::level2::dtpmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `AP` (input), `X` (input), `INCX` (input)

## `DTRCO`

- Canonical path: `slatec_sys::linear_algebra::dense::dtrco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `LDT` (input), `N` (input), `RCOND` (output), `Z` (output), `JOB` (input)

## `DTRDI`

- Canonical path: `slatec_sys::linear_algebra::dense::dtrdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input-output), `LDT` (input), `N` (input), `DET` (output), `JOB` (input), `INFO` (status-output)

## `DTRMM`

- Canonical path: `slatec_sys::blas::level3::dtrmm`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `SIDE` (input), `UPLO` (input), `TRANSA` (input), `DIAG` (input), `M` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input)

## `DTRMV`

- Canonical path: `slatec_sys::blas::level2::dtrmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input)

## `DTRSL`

- Canonical path: `slatec_sys::linear_algebra::dense::dtrsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `LDT` (input), `N` (input), `B` (input-output), `JOB` (input), `INFO` (status-output)

## `DTRSV`

- Canonical path: `slatec_sys::blas::level2::dtrsv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `A` (input-output), `LDA` (input), `X` (input-output), `INCX` (input)

## `DULSIA`

- Canonical path: `slatec_sys::linear_algebra::dense::dulsia`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `MDA` (input), `M` (input), `N` (input), `B` (input-output), `MDB` (input), `NB` (input), `RE` (input), `AE` (input), `KEY` (input), `MODE` (input), `NP` (input), `KRANK` (output), `KSURE` (output), `RNORM` (output), `W` (input-output), `LW` (input), `IWORK` (workspace-output), `LIW` (input), `INFO` (status-output)

## `DWNNLS`

- Canonical path: `slatec_sys::approximation::dwnnls`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `W` (input-output), `MDW` (input-output), `ME` (input), `MA` (input), `N` (input), `L` (input), `PRGOPT` (input-output), `X` (input-output), `RNORM` (output), `MODE` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `DXLEGF`

- Canonical path: `slatec_sys::special::dxlegf`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `DNU1` (input), `NUDIFF` (input), `MU1` (input), `MU2` (input), `THETA` (input), `ID` (input), `PQA` (input-output), `IPQA` (input-output), `IERROR` (status-output)

## `DXNRMP`

- Canonical path: `slatec_sys::special::dxnrmp`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NU` (input), `MU1` (input), `MU2` (input), `DARG` (input), `MODE` (input), `DPN` (output), `IPN` (input), `ISIG` (output), `IERROR` (status-output)

## `EFC`

- Canonical path: `slatec_sys::approximation::efc`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NDATA` (input-output), `XDATA` (input-output), `YDATA` (input-output), `SDDATA` (input-output), `NORD` (input-output), `NBKPT` (input-output), `BKPT` (input-output), `MDEIN` (input), `MDEOUT` (output), `COEFF` (output), `LW` (input), `W` (workspace-output)

## `ELTRAN`

- Canonical path: `slatec_sys::linear_algebra::eigen::eltran`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `A` (input), `INT` (input), `Z` (output)

## `EXINT`

- Canonical path: `slatec_sys::special::exint`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `N` (input), `KODE` (input), `M` (input), `TOL` (input), `EN` (output), `NZ` (status-output), `IERR` (output)

## `EZFFTB`

- Canonical path: `slatec_sys::fftpack::ezfftb`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `R` (output), `AZERO` (input), `A` (input), `B` (input), `WSAVE` (workspace-output)

## `EZFFTF`

- Canonical path: `slatec_sys::fftpack::ezfftf`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `R` (input), `AZERO` (output), `A` (output), `B` (output), `WSAVE` (workspace-output)

## `FC`

- Canonical path: `slatec_sys::approximation::fc`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NDATA` (input-output), `XDATA` (input-output), `YDATA` (input-output), `SDDATA` (input-output), `NORD` (input-output), `NBKPT` (input-output), `BKPT` (input-output), `NCONST` (input-output), `XCONST` (input-output), `YCONST` (input-output), `NDERIV` (input-output), `MODE` (input-output), `COEFF` (input-output), `W` (input-output), `IW` (input-output)

## `FIGI`

- Canonical path: `slatec_sys::linear_algebra::eigen::figi`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `T` (input-output), `D` (output), `E` (output), `E2` (output), `IERR` (output)

## `FIGI2`

- Canonical path: `slatec_sys::linear_algebra::eigen::figi2`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `T` (input-output), `D` (output), `E` (output), `Z` (output), `IERR` (output)

## `FZERO`

- Canonical path: `slatec_sys::roots::scalar::fzero`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `B` (input-output), `C` (input-output), `R` (output), `RE` (input), `AE` (input), `IFLAG` (status-output)

## `GAMLIM`

- Canonical path: `slatec_sys::special::gamlim`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XMIN` (input), `XMAX` (input)

## `GAUS8`

- Canonical path: `slatec_sys::quadrature::gaus8`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FUN` (input), `A` (input-output), `B` (input), `ERR` (input-output), `ANS` (output), `IERR` (output)

## `GENBUN`

- Canonical path: `slatec_sys::pde::fishpack::genbun`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NPEROD` (input), `N` (input), `MPEROD` (input), `M` (input), `A` (input), `B` (input), `C` (input), `IDIMY` (input), `Y` (input-output), `IERROR` (status-output), `W` (input-output)

## `HFTI`

- Canonical path: `slatec_sys::linear_algebra::dense::hfti`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `MDA` (input-output), `M` (input-output), `N` (input-output), `B` (output), `MDB` (input-output), `NB` (input-output), `TAU` (input), `KRANK` (output), `RNORM` (output), `H` (output), `G` (output), `IP` (input-output)

## `HQR`

- Canonical path: `slatec_sys::linear_algebra::eigen::hqr`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `H` (input), `WR` (output), `WI` (output), `IERR` (output)

## `HQR2`

- Canonical path: `slatec_sys::linear_algebra::eigen::hqr2`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `H` (input), `WR` (output), `WI` (output), `Z` (input-output), `IERR` (output)

## `HSTCRT`

- Canonical path: `slatec_sys::pde::fishpack::hstcrt`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `B` (input), `M` (input), `MBDCND` (input), `BDA` (input), `BDB` (input), `C` (input), `D` (input), `N` (input), `NBDCND` (input), `BDC` (input), `BDD` (input), `ELMBDA` (input), `F` (input-output), `IDIMF` (input), `PERTRB` (output), `IERROR` (status-output), `W` (input-output)

## `HSTCSP`

- Canonical path: `slatec_sys::pde::fishpack::hstcsp`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `INTL` (input), `A` (input), `B` (input), `M` (input), `MBDCND` (input), `BDA` (input), `BDB` (input), `C` (input), `D` (input), `N` (input), `NBDCND` (input), `BDC` (input), `BDD` (input), `ELMBDA` (input), `F` (input-output), `IDIMF` (input), `PERTRB` (output), `IERROR` (status-output), `W` (input-output)

## `HSTCYL`

- Canonical path: `slatec_sys::pde::fishpack::hstcyl`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `B` (input), `M` (input), `MBDCND` (input), `BDA` (input), `BDB` (input), `C` (input), `D` (input), `N` (input), `NBDCND` (input), `BDC` (input), `BDD` (input), `ELMBDA` (input), `F` (input-output), `IDIMF` (input), `PERTRB` (output), `IERROR` (status-output), `W` (input-output)

## `HSTPLR`

- Canonical path: `slatec_sys::pde::fishpack::hstplr`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `B` (input), `M` (input), `MBDCND` (input), `BDA` (input), `BDB` (input), `C` (input), `D` (input), `N` (input), `NBDCND` (input), `BDC` (input), `BDD` (input), `ELMBDA` (input), `F` (input-output), `IDIMF` (input), `PERTRB` (output), `IERROR` (status-output), `W` (input-output)

## `HSTSSP`

- Canonical path: `slatec_sys::pde::fishpack::hstssp`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `B` (input), `M` (input), `MBDCND` (input), `BDA` (input), `BDB` (input), `C` (input), `D` (input), `N` (input), `NBDCND` (input), `BDC` (input), `BDD` (input), `ELMBDA` (input), `F` (input-output), `IDIMF` (input), `PERTRB` (output), `IERROR` (status-output), `W` (input-output)

## `HTRID3`

- Canonical path: `slatec_sys::linear_algebra::eigen::htrid3`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input-output), `D` (output), `E` (output), `E2` (output), `TAU` (output)

## `HTRIDI`

- Canonical path: `slatec_sys::linear_algebra::eigen::htridi`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `AR` (input-output), `AI` (input-output), `D` (output), `E` (output), `E2` (output), `TAU` (output)

## `HW3CRT`

- Canonical path: `slatec_sys::pde::fishpack::hw3crt`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XS` (input), `XF` (input), `L` (input), `LBDCND` (input), `BDXS` (input), `BDXF` (input), `YS` (input), `YF` (input), `M` (input), `MBDCND` (input), `BDYS` (input), `BDYF` (input), `ZS` (input), `ZF` (input), `N` (input), `NBDCND` (input), `BDZS` (input), `BDZF` (input), `ELMBDA` (input), `LDIMF` (input), `MDIMF` (input), `F` (input-output), `PERTRB` (output), `IERROR` (status-output), `W` (input)

## `HWSCRT`

- Canonical path: `slatec_sys::pde::fishpack::hwscrt`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `B` (input), `M` (input), `MBDCND` (input), `BDA` (input), `BDB` (input), `C` (input), `D` (input), `N` (input), `NBDCND` (input), `BDC` (input), `BDD` (input), `ELMBDA` (input), `F` (input-output), `IDIMF` (input), `PERTRB` (output), `IERROR` (status-output), `W` (input-output)

## `HWSCSP`

- Canonical path: `slatec_sys::pde::fishpack::hwscsp`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `INTL` (input), `TS` (input), `TF` (input), `M` (input), `MBDCND` (input), `BDTS` (input), `BDTF` (input), `RS` (input), `RF` (input), `N` (input), `NBDCND` (input), `BDRS` (input), `BDRF` (input), `ELMBDA` (input), `F` (input-output), `IDIMF` (input), `PERTRB` (output), `IERROR` (status-output), `W` (input-output)

## `HWSCYL`

- Canonical path: `slatec_sys::pde::fishpack::hwscyl`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `B` (input), `M` (input), `MBDCND` (input), `BDA` (input), `BDB` (input), `C` (input), `D` (input), `N` (input), `NBDCND` (input), `BDC` (input), `BDD` (input), `ELMBDA` (input), `F` (input-output), `IDIMF` (input), `PERTRB` (output), `IERROR` (status-output), `W` (input-output)

## `HWSPLR`

- Canonical path: `slatec_sys::pde::fishpack::hwsplr`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `B` (input), `M` (input), `MBDCND` (input), `BDA` (input), `BDB` (input), `C` (input), `D` (input), `N` (input), `NBDCND` (input), `BDC` (input), `BDD` (input), `ELMBDA` (input), `F` (input-output), `IDIMF` (input), `PERTRB` (output), `IERROR` (status-output), `W` (input-output)

## `HWSSSP`

- Canonical path: `slatec_sys::pde::fishpack::hwsssp`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `TS` (input), `TF` (input), `M` (input), `MBDCND` (input), `BDTS` (input), `BDTF` (input), `PS` (input), `PF` (input), `N` (input), `NBDCND` (input), `BDPS` (input), `BDPF` (input), `ELMBDA` (input), `F` (input-output), `IDIMF` (input), `PERTRB` (output), `IERROR` (status-output), `W` (input-output)

## `ICOPY`

- Canonical path: `slatec_sys::blas::level1::icopy`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `IX` (input), `INCX` (input), `IY` (input-output), `INCY` (input)

## `IMTQL1`

- Canonical path: `slatec_sys::linear_algebra::eigen::imtql1`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `D` (input-output), `E` (input), `IERR` (output)

## `IMTQL2`

- Canonical path: `slatec_sys::linear_algebra::eigen::imtql2`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `D` (input-output), `E` (input), `Z` (input-output), `IERR` (output)

## `IMTQLV`

- Canonical path: `slatec_sys::linear_algebra::eigen::imtqlv`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `D` (input-output), `E` (input-output), `E2` (input), `W` (output), `IND` (status-output), `IERR` (output), `RV1` (output)

## `INVIT`

- Canonical path: `slatec_sys::linear_algebra::eigen::invit`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input-output), `WR` (input), `WI` (input-output), `SELECT` (input), `MM` (input), `M` (output), `Z` (output), `IERR` (output), `RM1` (output), `RV1` (output), `RV2` (output)

## `LLSIA`

- Canonical path: `slatec_sys::linear_algebra::dense::llsia`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `MDA` (input), `M` (input), `N` (input), `B` (input-output), `MDB` (input), `NB` (input), `RE` (input), `AE` (input), `KEY` (input), `MODE` (input), `NP` (input), `KRANK` (output), `KSURE` (output), `RNORM` (output), `W` (input-output), `LW` (input), `IWORK` (workspace-output), `LIW` (input), `INFO` (status-output)

## `LSEI`

- Canonical path: `slatec_sys::approximation::lsei`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `W` (input-output), `MDW` (input-output), `ME` (input), `MA` (input), `MG` (input), `N` (input), `PRGOPT` (input-output), `X` (input-output), `RNORME` (input-output), `RNORML` (output), `MODE` (output), `WS` (input-output), `IP` (input-output)

## `MINFIT`

- Canonical path: `slatec_sys::linear_algebra::dense::minfit`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `M` (input), `N` (input), `A` (input), `W` (output), `IP` (input), `B` (input), `IERR` (output), `RV1` (output)

## `ORTBAK`

- Canonical path: `slatec_sys::linear_algebra::eigen::ortbak`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `LOW` (input), `IGH` (input), `A` (input), `ORT` (input), `M` (input), `Z` (input-output)

## `ORTHES`

- Canonical path: `slatec_sys::linear_algebra::eigen::orthes`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `A` (input-output), `ORT` (output)

## `ORTRAN`

- Canonical path: `slatec_sys::linear_algebra::eigen::ortran`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `LOW` (input), `IGH` (input), `A` (input), `ORT` (input), `Z` (output)

## `PCHBS`

- Canonical path: `slatec_sys::interpolation::pchbs`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input-output), `D` (input-output), `INCFD` (input), `KNOTYP` (input), `NKNOTS` (input-output), `T` (input-output), `BCOEF` (input-output), `NDIM` (output), `KORD` (output), `IERR` (output)

## `PCHCM`

- Canonical path: `slatec_sys::interpolation::pchcm`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (input), `INCFD` (input), `SKIP` (input-output), `ISMON` (output), `IERR` (output)

## `PCHFD`

- Canonical path: `slatec_sys::interpolation::pchfd`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (input), `INCFD` (input), `SKIP` (input-output), `NE` (input), `XE` (input), `FE` (output), `DE` (output), `IERR` (output)

## `PCHFE`

- Canonical path: `slatec_sys::interpolation::pchfe`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (input), `INCFD` (input), `SKIP` (input-output), `NE` (input), `XE` (input), `FE` (output), `IERR` (output)

## `PCHIA`

- Canonical path: `slatec_sys::interpolation::pchia`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (input), `INCFD` (input), `SKIP` (output), `A` (input), `B` (input), `IERR` (output)

## `PCHIC`

- Canonical path: `slatec_sys::interpolation::pchic`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `IC` (input), `VC` (input), `SWITCH` (input), `N` (input), `X` (input), `F` (input), `D` (output), `INCFD` (input), `WK` (input), `NWK` (input), `IERR` (output)

## `PCHID`

- Canonical path: `slatec_sys::interpolation::pchid`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (input), `INCFD` (input), `SKIP` (output), `IA` (input), `IB` (input), `IERR` (output)

## `PCHIM`

- Canonical path: `slatec_sys::interpolation::pchim`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `F` (input), `D` (output), `INCFD` (input), `IERR` (output)

## `PCHSP`

- Canonical path: `slatec_sys::interpolation::pchsp`
- Family: `PCHIP`
- Review result: `machine-semantic-audit-passed`
- Arguments: `IC` (input), `VC` (input), `N` (input), `X` (input), `F` (input), `D` (output), `INCFD` (input), `WK` (input-output), `NWK` (input), `IERR` (output)

## `PCOEF`

- Canonical path: `slatec_sys::approximation::pcoef`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `L` (input), `C` (input), `TC` (output), `A` (input)

## `POCH1`

- Canonical path: `slatec_sys::special::poch1`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `X` (input-output)

## `POIS3D`

- Canonical path: `slatec_sys::pde::fishpack::pois3d`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `LPEROD` (input), `L` (input), `C1` (input-output), `MPEROD` (input), `M` (input), `C2` (input), `NPEROD` (input), `N` (input), `A` (input), `B` (input), `C` (input), `LDIMF` (input), `MDIMF` (input), `F` (input-output), `IERROR` (status-output), `W` (input)

## `POISTG`

- Canonical path: `slatec_sys::pde::fishpack::poistg`
- Family: `FISHPACK elliptic PDE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NPEROD` (input), `N` (input), `MPEROD` (input), `M` (input), `A` (input-output), `B` (input), `C` (input), `IDIMY` (input), `Y` (input-output), `IERROR` (status-output), `W` (input-output)

## `POLCOF`

- Canonical path: `slatec_sys::interpolation::polcof`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `XX` (input), `N` (input), `X` (input), `C` (input), `D` (input-output), `WORK` (workspace-output)

## `POLFIT`

- Canonical path: `slatec_sys::approximation::polfit`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input-output), `X` (input), `Y` (input), `W` (input-output), `MAXDEG` (input-output), `NDEG` (output), `EPS` (input-output), `R` (output), `IERR` (output), `A` (output)

## `POLYVL`

- Canonical path: `slatec_sys::interpolation::polyvl`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NDER` (input), `XX` (input), `YFIT` (output), `YP` (input-output), `N` (input), `X` (input), `C` (input), `WORK` (workspace-output), `IERR` (output)

## `PPQAD`

- Canonical path: `slatec_sys::quadrature::ppqad`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `LDC` (input), `C` (input), `XI` (input-output), `LXI` (input), `K` (input), `X1` (input), `X2` (input), `PQUAD` (output)

## `PPVAL`

- Canonical path: `slatec_sys::interpolation::ppval`
- Family: `Interpolation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `LDC` (input), `C` (input), `XI` (input), `LXI` (input), `K` (input), `IDERIV` (input), `X` (input), `INPPV` (input-output)

## `PSIFN`

- Canonical path: `slatec_sys::special::psifn`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `N` (input), `KODE` (input), `M` (input), `ANS` (input-output), `NZ` (status-output), `IERR` (output)

## `PVALUE`

- Canonical path: `slatec_sys::approximation::pvalue`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `L` (input), `NDER` (input), `X` (input), `YFIT` (output), `YP` (output), `A` (input)

## `QAG`

- Canonical path: `slatec_sys::quadrature::qag`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `EPSABS` (input), `EPSREL` (input), `KEY` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMIT` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `QAGI`

- Canonical path: `slatec_sys::quadrature::qagi`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `BOUND` (input), `INF` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMIT` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `QAGIE`

- Canonical path: `slatec_sys::quadrature::callbacks::qagie`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `BOUND` (input), `INF` (input), `EPSABS` (input), `EPSREL` (input), `LIMIT` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `ALIST` (output), `BLIST` (output), `RLIST` (output), `ELIST` (output), `IORD` (output), `LAST` (output)

## `QAGP`

- Canonical path: `slatec_sys::quadrature::qagp`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `NPTS2` (input), `POINTS` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LENIW` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `QAGPE`

- Canonical path: `slatec_sys::quadrature::callbacks::qagpe`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `NPTS2` (input), `POINTS` (input), `EPSABS` (input), `EPSREL` (input), `LIMIT` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `ALIST` (output), `BLIST` (output), `RLIST` (output), `ELIST` (output), `PTS` (output), `IORD` (output), `LEVEL` (output), `NDIN` (output), `LAST` (output)

## `QAGS`

- Canonical path: `slatec_sys::quadrature::qags`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMIT` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `QAGSE`

- Canonical path: `slatec_sys::quadrature::callbacks::qagse`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `EPSABS` (input), `EPSREL` (input), `LIMIT` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `ALIST` (output), `BLIST` (output), `RLIST` (output), `ELIST` (output), `IORD` (output), `LAST` (output)

## `QAWC`

- Canonical path: `slatec_sys::quadrature::qawc`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `C` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMIT` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `QAWCE`

- Canonical path: `slatec_sys::quadrature::callbacks::qawce`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `C` (input), `EPSABS` (input), `EPSREL` (input), `LIMIT` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `ALIST` (output), `BLIST` (output), `RLIST` (output), `ELIST` (output), `IORD` (output), `LAST` (output)

## `QAWF`

- Canonical path: `slatec_sys::quadrature::qawf`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `OMEGA` (input), `INTEGR` (input), `EPSABS` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMLST` (input), `LST` (output), `LENIW` (input), `MAXP1` (input), `LENW` (input), `IWORK` (workspace-output), `WORK` (workspace-output)

## `QAWO`

- Canonical path: `slatec_sys::quadrature::qawo`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `OMEGA` (input), `INTEGR` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LENIW` (input), `MAXP1` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `QAWS`

- Canonical path: `slatec_sys::quadrature::qaws`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `ALFA` (input), `BETA` (input), `INTEGR` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `LIMIT` (input), `LENW` (input), `LAST` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `QAWSE`

- Canonical path: `slatec_sys::quadrature::callbacks::qawse`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `ALFA` (input), `BETA` (input), `INTEGR` (input), `EPSABS` (input), `EPSREL` (input), `LIMIT` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output), `ALIST` (output), `BLIST` (output), `RLIST` (output), `ELIST` (output), `IORD` (output), `LAST` (output)

## `QK15`

- Canonical path: `slatec_sys::quadrature::callbacks::qk15`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `QK15I`

- Canonical path: `slatec_sys::quadrature::callbacks::qk15i`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `BOUN` (input), `INF` (input), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `QK15W`

- Canonical path: `slatec_sys::quadrature::callbacks::qk15w`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `W` (callback), `P1` (input), `P2` (input), `P3` (input), `P4` (input), `KP` (input), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `QK21`

- Canonical path: `slatec_sys::quadrature::callbacks::qk21`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `QK31`

- Canonical path: `slatec_sys::quadrature::callbacks::qk31`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `QK41`

- Canonical path: `slatec_sys::quadrature::callbacks::qk41`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `QK51`

- Canonical path: `slatec_sys::quadrature::callbacks::qk51`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `QK61`

- Canonical path: `slatec_sys::quadrature::callbacks::qk61`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `RESULT` (output), `ABSERR` (output), `RESABS` (output), `RESASC` (output)

## `QNC79`

- Canonical path: `slatec_sys::quadrature::qnc79`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FUN` (callback), `A` (input), `B` (input), `ERR` (input), `ANS` (output), `IERR` (output), `K` (output)

## `QNG`

- Canonical path: `slatec_sys::quadrature::qng`
- Family: `Numerical quadrature`
- Review result: `machine-semantic-audit-passed`
- Arguments: `F` (callback), `A` (input), `B` (input), `EPSABS` (input), `EPSREL` (input), `RESULT` (output), `ABSERR` (output), `NEVAL` (output), `IER` (status-output)

## `QZHES`

- Canonical path: `slatec_sys::linear_algebra::eigen::qzhes`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input), `B` (input), `MATZ` (input), `Z` (output)

## `QZIT`

- Canonical path: `slatec_sys::linear_algebra::eigen::qzit`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input), `B` (input-output), `EPS1` (input), `MATZ` (input), `Z` (input-output), `IERR` (output)

## `QZVAL`

- Canonical path: `slatec_sys::linear_algebra::eigen::qzval`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input), `B` (input-output), `ALFR` (output), `ALFI` (output), `BETA` (output), `MATZ` (input), `Z` (output)

## `QZVEC`

- Canonical path: `slatec_sys::linear_algebra::eigen::qzvec`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input-output), `B` (input), `ALFR` (input-output), `ALFI` (input-output), `BETA` (input-output), `Z` (input-output)

## `RAND`

- Canonical path: `slatec_sys::statistics::rand`
- Family: `Probability and statistics`
- Review result: `machine-semantic-audit-passed`
- Arguments: `R` (input)

## `RATQR`

- Canonical path: `slatec_sys::linear_algebra::eigen::ratqr`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `EPS1` (input-output), `D` (input-output), `E` (input-output), `E2` (input), `M` (input-output), `W` (output), `IND` (status-output), `BD` (output), `TYPE` (input), `IDEF` (input), `IERR` (output)

## `RC`

- Canonical path: `slatec_sys::special::rc`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `Y` (output), `IER` (input)

## `RC3JJ`

- Canonical path: `slatec_sys::special::rc3jj`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `L2` (input), `L3` (input), `M2` (input), `M3` (input), `L1MIN` (input-output), `L1MAX` (input-output), `THRCOF` (output), `NDIM` (input), `IER` (status-output)

## `RC3JM`

- Canonical path: `slatec_sys::special::rc3jm`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `L1` (input), `L2` (input), `L3` (input), `M1` (input), `M2MIN` (input-output), `M2MAX` (input-output), `THRCOF` (output), `NDIM` (input), `IER` (status-output)

## `RC6J`

- Canonical path: `slatec_sys::special::rc6j`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `L2` (input), `L3` (input), `L4` (input), `L5` (input), `L6` (input), `L1MIN` (input-output), `L1MAX` (input-output), `SIXCOF` (output), `NDIM` (input), `IER` (status-output)

## `RD`

- Canonical path: `slatec_sys::special::rd`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `Y` (input-output), `Z` (output), `IER` (input)

## `RF`

- Canonical path: `slatec_sys::special::rf`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `Y` (input-output), `Z` (output), `IER` (input)

## `RFFTB1`

- Canonical path: `slatec_sys::fftpack::rfftb1`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `C` (input), `CH` (input), `WA` (input), `IFAC` (input)

## `RFFTI1`

- Canonical path: `slatec_sys::fftpack::rffti1`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `WA` (input), `IFAC` (input)

## `RG`

- Canonical path: `slatec_sys::linear_algebra::eigen::rg`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input), `WR` (output), `WI` (output), `MATZ` (input), `Z` (output), `IV1` (output), `FV1` (output), `IERR` (output)

## `RGG`

- Canonical path: `slatec_sys::linear_algebra::eigen::rgg`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input), `B` (input), `ALFR` (output), `ALFI` (output), `BETA` (output), `MATZ` (input), `Z` (output), `IERR` (output)

## `RJ`

- Canonical path: `slatec_sys::special::rj`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `Y` (input-output), `Z` (input-output), `P` (output), `IER` (input)

## `RPQR79`

- Canonical path: `slatec_sys::roots::complex::rpqr79`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NDEG` (input), `COEFF` (input), `ROOT` (output), `IERR` (output), `WORK` (workspace-output)

## `RPZERO`

- Canonical path: `slatec_sys::roots::complex::rpzero`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `A` (input), `R` (input-output), `T` (input), `IFLG` (output), `S` (input-output)

## `RS`

- Canonical path: `slatec_sys::linear_algebra::eigen::rs`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input-output), `W` (output), `MATZ` (input), `Z` (output), `FV1` (output), `FV2` (output), `IERR` (output)

## `RSB`

- Canonical path: `slatec_sys::linear_algebra::eigen::rsb`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `MB` (input), `A` (input), `W` (output), `MATZ` (input), `Z` (output), `FV1` (output), `FV2` (output), `IERR` (output)

## `RSG`

- Canonical path: `slatec_sys::linear_algebra::eigen::rsg`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input), `B` (input), `W` (output), `MATZ` (input), `Z` (output), `FV1` (output), `FV2` (output), `IERR` (output)

## `RSGAB`

- Canonical path: `slatec_sys::linear_algebra::eigen::rsgab`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input), `B` (input), `W` (output), `MATZ` (input), `Z` (output), `FV1` (output), `FV2` (output), `IERR` (output)

## `RSGBA`

- Canonical path: `slatec_sys::linear_algebra::eigen::rsgba`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input), `B` (input), `W` (output), `MATZ` (input), `Z` (output), `FV1` (output), `FV2` (output), `IERR` (output)

## `RSP`

- Canonical path: `slatec_sys::linear_algebra::eigen::rsp`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `NV` (input), `A` (input), `W` (output), `MATZ` (input), `Z` (output), `FV1` (output), `FV2` (output), `IERR` (output)

## `RST`

- Canonical path: `slatec_sys::linear_algebra::eigen::rst`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `W` (input-output), `E` (input), `MATZ` (input), `Z` (output), `IERR` (output)

## `RT`

- Canonical path: `slatec_sys::linear_algebra::eigen::rt`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input), `W` (output), `MATZ` (input), `Z` (output), `FV1` (output), `IERR` (output)

## `RUNIF`

- Canonical path: `slatec_sys::statistics::runif`
- Family: `Probability and statistics`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `N` (input)

## `SBCG`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::sbcg`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MTTVEC` (callback), `MSOLVE` (callback), `MTSOLV` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `Z` (workspace-output), `P` (workspace-output), `RR` (workspace-output), `ZZ` (workspace-output), `PP` (workspace-output), `DZ` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SBOCLS`

- Canonical path: `slatec_sys::approximation::sbocls`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `W` (input-output), `MDW` (input), `MCON` (input-output), `MROWS` (input-output), `NCOLS` (input-output), `BL` (input-output), `BU` (input-output), `IND` (input), `IOPT` (input-output), `X` (input-output), `RNORMC` (input-output), `RNORM` (input-output), `MODE` (output), `RW` (input-output), `IW` (input-output)

## `SBOLS`

- Canonical path: `slatec_sys::approximation::sbols`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `W` (input-output), `MDW` (input), `MROWS` (input-output), `NCOLS` (input-output), `BL` (input-output), `BU` (input-output), `IND` (input), `IOPT` (input-output), `X` (input-output), `RNORM` (input-output), `MODE` (output), `RW` (input-output), `IW` (input-output)

## `SCG`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::scg`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MSOLVE` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `Z` (workspace-output), `P` (workspace-output), `DZ` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SCGN`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::scgn`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MTTVEC` (callback), `MSOLVE` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `Z` (workspace-output), `P` (workspace-output), `ATP` (workspace-output), `ATZ` (workspace-output), `DZ` (workspace-output), `ATDZ` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SCGS`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::scgs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MSOLVE` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `R0` (workspace-output), `P` (workspace-output), `Q` (workspace-output), `U` (workspace-output), `V1` (workspace-output), `V2` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SCHDC`

- Canonical path: `slatec_sys::linear_algebra::dense::schdc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `P` (input), `WORK` (workspace-output), `JPVT` (input-output), `JOB` (input), `INFO` (status-output)

## `SCHDD`

- Canonical path: `slatec_sys::linear_algebra::dense::schdd`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `R` (input-output), `LDR` (input), `P` (input), `X` (input), `Z` (input-output), `LDZ` (input), `NZ` (input), `Y` (input), `RHO` (input-output), `C` (output), `S` (output), `INFO` (status-output)

## `SCHEX`

- Canonical path: `slatec_sys::linear_algebra::dense::schex`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `R` (input-output), `LDR` (input), `P` (input), `K` (input), `L` (input), `Z` (input-output), `LDZ` (input), `NZ` (input), `C` (output), `S` (output), `JOB` (input)

## `SCHUD`

- Canonical path: `slatec_sys::linear_algebra::dense::schud`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `R` (input), `LDR` (input), `P` (input), `X` (input), `Z` (input-output), `LDZ` (input), `NZ` (input), `Y` (input), `RHO` (input-output), `C` (output), `S` (output)

## `SCOPY`

- Canonical path: `slatec_sys::blas::level1::scopy`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `SX` (input), `INCX` (input), `SY` (input-output), `INCY` (input)

## `SCOPYM`

- Canonical path: `slatec_sys::blas::level1::scopym`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `SX` (input), `INCX` (input), `SY` (input-output), `INCY` (input)

## `SCOV`

- Canonical path: `slatec_sys::least_squares::scov`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FCN` (callback), `IOPT` (input), `M` (input), `N` (input), `X` (input), `FVEC` (input), `R` (input), `LDR` (input), `INFO` (input), `WA1` (input), `WA2` (input), `WA3` (input), `WA4` (input)

## `SDASSL`

- Canonical path: `slatec_sys::dassl::sdassl`
- Family: `ODE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `RES` (callback), `NEQ` (input), `T` (input-output), `Y` (input-output), `YPRIME` (input-output), `TOUT` (input), `INFO` (input), `RTOL` (input-output), `ATOL` (input-output), `IDID` (output), `RWORK` (workspace-output), `LRW` (input), `IWORK` (workspace-output), `LIW` (input), `RPAR` (input), `IPAR` (input), `JAC` (callback)

## `SDRIV3`

- Canonical path: `slatec_sys::ode::sdriv3`
- Family: `ODE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `T` (input-output), `Y` (input-output), `F` (callback), `NSTATE` (input-output), `TOUT` (input-output), `NTASK` (input), `NROOT` (input), `EPS` (input-output), `EWT` (input), `IERROR` (input), `MINT` (input), `MITER` (input), `IMPL` (input), `ML` (input), `MU` (input), `MXORD` (input), `HMAX` (input), `WORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input), `JACOBN` (callback), `FA` (callback), `NDE` (input), `MXSTEP` (input), `G` (callback), `USERS` (callback), `IERFLG` (input-output)

## `SGBCO`

- Canonical path: `slatec_sys::linear_algebra::banded::sgbco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `RCOND` (output), `Z` (output)

## `SGBDI`

- Canonical path: `slatec_sys::linear_algebra::banded::sgbdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input), `LDA` (input), `N` (input), `ML` (input), `MU` (input), `IPVT` (input), `DET` (output)

## `SGBFA`

- Canonical path: `slatec_sys::linear_algebra::banded::sgbfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `INFO` (status-output)

## `SGBMV`

- Canonical path: `slatec_sys::blas::level2::sgbmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `TRANS` (input), `M` (input), `N` (input), `KL` (input), `KU` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `SGECO`

- Canonical path: `slatec_sys::linear_algebra::dense::sgeco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `IPVT` (output), `RCOND` (output), `Z` (output)

## `SGEDI`

- Canonical path: `slatec_sys::linear_algebra::dense::sgedi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `IPVT` (input), `DET` (output), `WORK` (workspace-output), `JOB` (input)

## `SGEEV`

- Canonical path: `slatec_sys::linear_algebra::eigen::sgeev`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `E` (output), `V` (input-output), `LDV` (input), `WORK` (workspace-output), `JOB` (input), `INFO` (status-output)

## `SGEFA`

- Canonical path: `slatec_sys::linear_algebra::dense::sgefa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `IPVT` (output), `INFO` (status-output)

## `SGEFS`

- Canonical path: `slatec_sys::linear_algebra::dense::sgefs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (output), `LDA` (input), `N` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output), `IWORK` (workspace-output)

## `SGEIR`

- Canonical path: `slatec_sys::linear_algebra::dense::sgeir`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `LDA` (input), `N` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output), `IWORK` (workspace-output)

## `SGEMM`

- Canonical path: `slatec_sys::blas::level3::sgemm`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `TRANSA` (input), `TRANSB` (input), `M` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input), `C` (input), `LDC` (input)

## `SGEMV`

- Canonical path: `slatec_sys::blas::level2::sgemv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `TRANS` (input), `M` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `SGER`

- Canonical path: `slatec_sys::blas::level2::sger`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `M` (input), `N` (input), `ALPHA` (input), `X` (input), `INCX` (input), `Y` (input), `INCY` (input), `A` (input), `LDA` (input)

## `SGLSS`

- Canonical path: `slatec_sys::linear_algebra::dense::sglss`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `MDA` (input), `M` (input), `N` (input), `B` (input-output), `MDB` (input), `NB` (input), `RNORM` (input-output), `WORK` (workspace-output), `LW` (input), `IWORK` (workspace-output), `LIW` (input), `INFO` (status-output)

## `SGMRES`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::sgmres`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input-output), `ISYM` (input), `MATVEC` (callback), `MSOLVE` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `SB` (input), `SX` (input), `RGWK` (output), `LRGW` (input), `IGWK` (input-output), `LIGW` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SGTSL`

- Canonical path: `slatec_sys::linear_algebra::banded::sgtsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `C` (input), `D` (input), `E` (input), `B` (input-output), `INFO` (status-output)

## `SINQB`

- Canonical path: `slatec_sys::fftpack::sinqb`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input-output), `WSAVE` (workspace-output)

## `SINQF`

- Canonical path: `slatec_sys::fftpack::sinqf`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input-output), `WSAVE` (workspace-output)

## `SINTI`

- Canonical path: `slatec_sys::fftpack::sinti`
- Family: `FFTPACK transforms`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `WSAVE` (workspace-output)

## `SINTRP`

- Canonical path: `slatec_sys::ode::sintrp`
- Family: `ODE solvers`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `Y` (input), `XOUT` (input), `YOUT` (input-output), `YPOUT` (input-output), `NEQN` (input), `KOLD` (input), `PHI` (input), `IVC` (input), `IV` (input), `KGI` (input), `GI` (input), `ALPHA` (input), `OG` (input), `OW` (input), `OX` (input), `OY` (input)

## `SIR`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::sir`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MSOLVE` (callback), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `Z` (workspace-output), `DZ` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SLLTI2`

- Canonical path: `slatec_sys::linear_algebra::dense::sllti2`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (output), `NEL` (input), `IEL` (input), `JEL` (input), `EL` (input), `DINV` (input)

## `SNBCO`

- Canonical path: `slatec_sys::linear_algebra::banded::snbco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `RCOND` (output), `Z` (output)

## `SNBDI`

- Canonical path: `slatec_sys::linear_algebra::banded::snbdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input), `LDA` (input), `N` (input), `ML` (input), `MU` (input), `IPVT` (input), `DET` (output)

## `SNBFA`

- Canonical path: `slatec_sys::linear_algebra::banded::snbfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input-output), `LDA` (input), `N` (input), `ML` (input-output), `MU` (input-output), `IPVT` (output), `INFO` (status-output)

## `SNBFS`

- Canonical path: `slatec_sys::linear_algebra::banded::snbfs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (output), `LDA` (input), `N` (input), `ML` (input), `MU` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output), `IWORK` (workspace-output)

## `SNBIR`

- Canonical path: `slatec_sys::linear_algebra::banded::snbir`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABE` (input), `LDA` (input), `N` (input-output), `ML` (input), `MU` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output), `IWORK` (workspace-output)

## `SNLS1`

- Canonical path: `slatec_sys::least_squares::snls1`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FCN` (callback), `IOPT` (input), `M` (input), `N` (input), `X` (input-output), `FVEC` (input-output), `FJAC` (input-output), `LDFJAC` (input), `FTOL` (input), `XTOL` (input), `GTOL` (input), `MAXFEV` (input), `EPSFCN` (input), `DIAG` (input), `MODE` (input), `FACTOR` (input), `NPRINT` (input), `INFO` (input), `NFEV` (input-output), `NJEV` (input-output), `IPVT` (input-output), `QTF` (input-output), `WA1` (workspace-output), `WA2` (workspace-output), `WA3` (workspace-output), `WA4` (input)

## `SNLS1E`

- Canonical path: `slatec_sys::least_squares::snls1e`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FCN` (callback), `IOPT` (input), `M` (input), `N` (input), `X` (input), `FVEC` (input), `TOL` (input), `NPRINT` (input), `INFO` (input), `IW` (input), `WA` (input), `LWA` (input)

## `SNSQ`

- Canonical path: `slatec_sys::nonlinear::snsq`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FCN` (callback), `JAC` (input), `IOPT` (input), `N` (input), `X` (input-output), `FVEC` (input-output), `FJAC` (input), `LDFJAC` (input), `XTOL` (input), `MAXFEV` (input), `ML` (input), `MU` (input), `EPSFCN` (input), `DIAG` (input), `MODE` (input), `FACTOR` (input), `NPRINT` (input), `INFO` (input), `NFEV` (input), `NJEV` (input), `R` (input), `LR` (input), `QTF` (input), `WA1` (input), `WA2` (input), `WA3` (input), `WA4` (input)

## `SNSQE`

- Canonical path: `slatec_sys::nonlinear::snsqe`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FCN` (callback), `JAC` (callback), `IOPT` (input), `N` (input), `X` (input), `FVEC` (input), `TOL` (input), `NPRINT` (input), `INFO` (input-output), `WA` (input), `LWA` (input)

## `SOMN`

- Canonical path: `slatec_sys::linear_algebra::sparse::callbacks::somn`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `MATVEC` (callback), `MSOLVE` (callback), `NSAVE` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `R` (workspace-output), `Z` (workspace-output), `P` (workspace-output), `AP` (workspace-output), `EMAP` (workspace-output), `DZ` (workspace-output), `CSAV` (workspace-output), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SOS`

- Canonical path: `slatec_sys::nonlinear::systems::sos`
- Family: `Nonlinear equations`
- Review result: `machine-semantic-audit-passed`
- Arguments: `FNC` (callback), `NEQ` (input), `X` (output), `RTOLX` (input), `ATOLX` (input), `TOLF` (input), `IFLAG` (status-output), `RW` (input), `LRW` (input), `IW` (input), `LIW` (input)

## `SPBCO`

- Canonical path: `slatec_sys::linear_algebra::banded::spbco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `M` (input-output), `RCOND` (output), `Z` (output), `INFO` (status-output)

## `SPBDI`

- Canonical path: `slatec_sys::linear_algebra::banded::spbdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input), `LDA` (input), `N` (input), `M` (input), `DET` (output)

## `SPBFA`

- Canonical path: `slatec_sys::linear_algebra::banded::spbfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ABD` (input-output), `LDA` (input), `N` (input), `M` (input-output), `INFO` (status-output)

## `SPLP`

- Canonical path: `slatec_sys::linear_programming::splp`
- Family: `Optimization and least squares`
- Review result: `machine-semantic-audit-passed`
- Arguments: `USRMAT` (callback), `MRELAS` (input-output), `NVARS` (input-output), `COSTS` (input-output), `PRGOPT` (input-output), `DATTRV` (input-output), `BL` (input-output), `BU` (input-output), `IND` (input), `INFO` (status-output), `PRIMAL` (input-output), `DUALS` (input-output), `IBASIS` (input-output), `WORK` (workspace-output), `LW` (input-output), `IWORK` (workspace-output), `LIW` (input-output)

## `SPOCO`

- Canonical path: `slatec_sys::linear_algebra::dense::spoco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `RCOND` (output), `Z` (output), `INFO` (status-output)

## `SPODI`

- Canonical path: `slatec_sys::linear_algebra::dense::spodi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `DET` (output), `JOB` (input)

## `SPOFS`

- Canonical path: `slatec_sys::linear_algebra::dense::spofs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (output), `LDA` (input), `N` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output)

## `SPOIR`

- Canonical path: `slatec_sys::linear_algebra::dense::spoir`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input), `LDA` (input), `N` (input), `V` (output), `ITASK` (input), `IND` (input), `WORK` (workspace-output)

## `SPPCO`

- Canonical path: `slatec_sys::linear_algebra::packed::sppco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `RCOND` (output), `Z` (output), `INFO` (status-output)

## `SPPDI`

- Canonical path: `slatec_sys::linear_algebra::packed::sppdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `DET` (output), `JOB` (input)

## `SPTSL`

- Canonical path: `slatec_sys::linear_algebra::banded::sptsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `D` (input), `E` (input), `B` (input-output)

## `SQRDC`

- Canonical path: `slatec_sys::linear_algebra::dense::sqrdc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input-output), `LDX` (input), `N` (input), `P` (input), `QRAUX` (output), `JPVT` (input-output), `WORK` (workspace-output), `JOB` (input)

## `SQRSL`

- Canonical path: `slatec_sys::linear_algebra::dense::sqrsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `LDX` (input), `N` (input), `K` (input), `QRAUX` (input), `Y` (input), `QY` (output), `QTY` (output), `B` (output), `RSD` (output), `XB` (output), `JOB` (input), `INFO` (status-output)

## `SROTG`

- Canonical path: `slatec_sys::blas::level1::srotg`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `SA` (input-output), `SB` (input-output), `SC` (output), `SS` (output)

## `SROTMG`

- Canonical path: `slatec_sys::blas::level1::srotmg`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `SD1` (input-output), `SD2` (input-output), `SX1` (input-output), `SY1` (input), `SPARAM` (input)

## `SS2LT`

- Canonical path: `slatec_sys::linear_algebra::dense::ss2lt`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `NEL` (output), `IEL` (output), `JEL` (output), `EL` (output)

## `SSBMV`

- Canonical path: `slatec_sys::blas::level2::ssbmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `SSD2S`

- Canonical path: `slatec_sys::linear_algebra::dense::ssd2s`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `DINV` (output)

## `SSDBCG`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssdbcg`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (output)

## `SSDCG`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssdcg`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSDCGN`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssdcgn`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSDCGS`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssdcgs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSDGMR`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssdgmr`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input-output), `ISYM` (input), `NSAVE` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSDOMN`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssdomn`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `NSAVE` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSDS`

- Canonical path: `slatec_sys::linear_algebra::dense::ssds`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `DINV` (output)

## `SSDSCL`

- Canonical path: `slatec_sys::linear_algebra::dense::ssdscl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `X` (input-output), `B` (input-output), `DINV` (output), `JOB` (input), `ITOL` (input)

## `SSGS`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssgs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSICCG`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssiccg`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSICO`

- Canonical path: `slatec_sys::linear_algebra::dense::ssico`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (output), `RCOND` (output), `Z` (output)

## `SSICS`

- Canonical path: `slatec_sys::linear_algebra::dense::ssics`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `NEL` (output), `IEL` (output), `JEL` (output), `EL` (output), `D` (output), `R` (workspace-output), `IWARN` (output)

## `SSIDI`

- Canonical path: `slatec_sys::linear_algebra::dense::ssidi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (input), `DET` (output), `INERT` (output), `WORK` (workspace-output), `JOB` (input)

## `SSIEV`

- Canonical path: `slatec_sys::linear_algebra::eigen::ssiev`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `E` (output), `WORK` (workspace-output), `JOB` (input), `INFO` (status-output)

## `SSIFA`

- Canonical path: `slatec_sys::linear_algebra::dense::ssifa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `LDA` (input), `N` (input), `KPVT` (output), `INFO` (status-output)

## `SSILUR`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssilur`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSILUS`

- Canonical path: `slatec_sys::linear_algebra::dense::ssilus`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `NL` (output), `IL` (output), `JL` (output), `L` (output), `DINV` (input-output), `NU` (output), `IU` (output), `JU` (output), `U` (output), `NROW` (workspace-output), `NCOL` (workspace-output)

## `SSJAC`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssjac`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSLI`

- Canonical path: `slatec_sys::linear_algebra::dense::ssli`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SSLI2`

- Canonical path: `slatec_sys::linear_algebra::dense::ssli2`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (output), `NEL` (input), `IEL` (input), `JEL` (input), `EL` (input)

## `SSLLTI`

- Canonical path: `slatec_sys::linear_algebra::dense::ssllti`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SSLUBC`

- Canonical path: `slatec_sys::linear_algebra::sparse::sslubc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSLUCN`

- Canonical path: `slatec_sys::linear_algebra::sparse::sslucn`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSLUCS`

- Canonical path: `slatec_sys::linear_algebra::dense::sslucs`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSLUGM`

- Canonical path: `slatec_sys::linear_algebra::sparse::sslugm`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input), `JA` (input), `A` (input-output), `ISYM` (input), `NSAVE` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSLUI`

- Canonical path: `slatec_sys::linear_algebra::dense::sslui`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SSLUI2`

- Canonical path: `slatec_sys::linear_algebra::dense::sslui2`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (output), `IL` (input), `JL` (input), `L` (input), `DINV` (input), `IU` (input), `JU` (input), `U` (input)

## `SSLUI4`

- Canonical path: `slatec_sys::linear_algebra::dense::sslui4`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (output), `IL` (input), `JL` (input), `L` (input), `DINV` (input), `IU` (input), `JU` (input), `U` (input)

## `SSLUOM`

- Canonical path: `slatec_sys::linear_algebra::sparse::ssluom`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input-output), `NELT` (input), `IA` (input-output), `JA` (input-output), `A` (input-output), `ISYM` (input), `NSAVE` (input), `ITOL` (input), `TOL` (input-output), `ITMAX` (input), `ITER` (output), `ERR` (output), `IERR` (output), `IUNIT` (input), `RWORK` (workspace-output), `LENW` (input), `IWORK` (workspace-output), `LENIW` (input)

## `SSLUTI`

- Canonical path: `slatec_sys::linear_algebra::dense::ssluti`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SSMMI2`

- Canonical path: `slatec_sys::linear_algebra::dense::ssmmi2`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (output), `IL` (input), `JL` (input), `L` (input), `DINV` (input), `IU` (input), `JU` (input), `U` (input)

## `SSMMTI`

- Canonical path: `slatec_sys::linear_algebra::dense::ssmmti`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `B` (input), `X` (input), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input), `RWORK` (workspace-output), `IWORK` (workspace-output)

## `SSMTV`

- Canonical path: `slatec_sys::blas::level1::ssmtv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `Y` (output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input)

## `SSMV`

- Canonical path: `slatec_sys::blas::level1::ssmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `X` (input), `Y` (output), `NELT` (input), `IA` (input), `JA` (input), `A` (input), `ISYM` (input)

## `SSPCO`

- Canonical path: `slatec_sys::linear_algebra::packed::sspco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (output), `RCOND` (output), `Z` (output)

## `SSPDI`

- Canonical path: `slatec_sys::linear_algebra::packed::sspdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (input), `DET` (output), `INERT` (output), `WORK` (workspace-output), `JOB` (input)

## `SSPEV`

- Canonical path: `slatec_sys::linear_algebra::eigen::sspev`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `N` (input), `E` (output), `V` (output), `LDV` (input), `WORK` (workspace-output), `JOB` (input), `INFO` (status-output)

## `SSPFA`

- Canonical path: `slatec_sys::linear_algebra::packed::sspfa`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `AP` (input-output), `N` (input), `KPVT` (output), `INFO` (status-output)

## `SSPMV`

- Canonical path: `slatec_sys::blas::level2::sspmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `AP` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `SSVDC`

- Canonical path: `slatec_sys::linear_algebra::dense::ssvdc`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `X` (input), `LDX` (input), `N` (input), `P` (input), `S` (output), `E` (output), `U` (output), `LDU` (input), `V` (output), `LDV` (input), `WORK` (workspace-output), `JOB` (input), `INFO` (status-output)

## `SSYMM`

- Canonical path: `slatec_sys::blas::level3::ssymm`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `SIDE` (input), `UPLO` (input), `M` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input), `C` (input), `LDC` (input)

## `SSYMV`

- Canonical path: `slatec_sys::blas::level2::ssymv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input), `BETA` (input), `Y` (input), `INCY` (input)

## `SSYR`

- Canonical path: `slatec_sys::blas::level2::ssyr`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `X` (input), `INCX` (input), `A` (input), `LDA` (input)

## `SSYR2`

- Canonical path: `slatec_sys::blas::level2::ssyr2`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `N` (input), `ALPHA` (input), `X` (input), `INCX` (input), `Y` (input), `INCY` (input), `A` (input), `LDA` (input)

## `SSYR2K`

- Canonical path: `slatec_sys::blas::level3::ssyr2k`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input), `BETA` (input-output), `C` (input-output), `LDC` (input)

## `SSYRK`

- Canonical path: `slatec_sys::blas::level3::ssyrk`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `N` (input), `K` (input), `ALPHA` (input), `A` (input), `LDA` (input), `BETA` (input), `C` (input), `LDC` (input)

## `STBMV`

- Canonical path: `slatec_sys::blas::level2::stbmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `K` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input)

## `STBSV`

- Canonical path: `slatec_sys::blas::level2::stbsv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `K` (input), `A` (input-output), `LDA` (input), `X` (input-output), `INCX` (input)

## `STPMV`

- Canonical path: `slatec_sys::blas::level2::stpmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `AP` (input), `X` (input), `INCX` (input)

## `STRCO`

- Canonical path: `slatec_sys::linear_algebra::dense::strco`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `LDT` (input), `N` (input), `RCOND` (output), `Z` (output), `JOB` (input)

## `STRDI`

- Canonical path: `slatec_sys::linear_algebra::dense::strdi`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input-output), `LDT` (input), `N` (input), `DET` (output), `JOB` (input), `INFO` (status-output)

## `STRMM`

- Canonical path: `slatec_sys::blas::level3::strmm`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `SIDE` (input), `UPLO` (input), `TRANSA` (input), `DIAG` (input), `M` (input), `N` (input), `ALPHA` (input), `A` (input), `LDA` (input), `B` (input), `LDB` (input)

## `STRMV`

- Canonical path: `slatec_sys::blas::level2::strmv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `A` (input), `LDA` (input), `X` (input), `INCX` (input)

## `STRSL`

- Canonical path: `slatec_sys::linear_algebra::dense::strsl`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `T` (input), `LDT` (input), `N` (input), `B` (input-output), `JOB` (input), `INFO` (status-output)

## `STRSV`

- Canonical path: `slatec_sys::blas::level2::strsv`
- Family: `Linear algebra kernels`
- Review result: `machine-semantic-audit-passed`
- Arguments: `UPLO` (input), `TRANS` (input), `DIAG` (input), `N` (input), `A` (input-output), `LDA` (input), `X` (input-output), `INCX` (input)

## `TINVIT`

- Canonical path: `slatec_sys::linear_algebra::eigen::tinvit`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `D` (input), `E` (input), `E2` (input), `M` (input), `W` (input), `IND` (input), `Z` (output), `IERR` (output), `RV1` (output), `RV2` (output), `RV3` (output), `RV4` (output), `RV6` (output)

## `TQL1`

- Canonical path: `slatec_sys::linear_algebra::eigen::tql1`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `D` (input-output), `E` (input), `IERR` (output)

## `TQL2`

- Canonical path: `slatec_sys::linear_algebra::eigen::tql2`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `D` (input-output), `E` (input), `Z` (input-output), `IERR` (output)

## `TQLRAT`

- Canonical path: `slatec_sys::linear_algebra::eigen::tqlrat`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `D` (input-output), `E2` (input), `IERR` (output)

## `TRED1`

- Canonical path: `slatec_sys::linear_algebra::eigen::tred1`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input-output), `D` (output), `E` (output), `E2` (output)

## `TRED2`

- Canonical path: `slatec_sys::linear_algebra::eigen::tred2`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `A` (input), `D` (output), `E` (output), `Z` (output)

## `TRED3`

- Canonical path: `slatec_sys::linear_algebra::eigen::tred3`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `NV` (input), `A` (input-output), `D` (output), `E` (output), `E2` (output)

## `TRIDIB`

- Canonical path: `slatec_sys::linear_algebra::eigen::tridib`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `N` (input), `EPS1` (input-output), `D` (input-output), `E` (input-output), `E2` (input-output), `LB` (output), `UB` (output), `M11` (input), `M` (input), `W` (output), `IND` (status-output), `IERR` (output), `RV4` (output), `RV5` (output)

## `TSTURM`

- Canonical path: `slatec_sys::linear_algebra::eigen::tsturm`
- Family: `Eigenvalue problems`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NM` (input), `N` (input), `EPS1` (input-output), `D` (input-output), `E` (input-output), `E2` (input), `LB` (input), `UB` (input), `MM` (input), `M` (output), `W` (output), `Z` (output), `IERR` (output), `RV1` (output), `RV2` (output), `RV3` (output), `RV4` (output), `RV5` (output), `RV6` (output)

## `ULSIA`

- Canonical path: `slatec_sys::linear_algebra::dense::ulsia`
- Family: `Dense linear algebra`
- Review result: `machine-semantic-audit-passed`
- Arguments: `A` (input-output), `MDA` (input), `M` (input), `N` (input), `B` (input-output), `MDB` (input), `NB` (input), `RE` (input), `AE` (input), `KEY` (input), `MODE` (input), `NP` (input), `KRANK` (output), `KSURE` (output), `RNORM` (output), `W` (input-output), `LW` (input), `IWORK` (workspace-output), `LIW` (input), `INFO` (status-output)

## `WNNLS`

- Canonical path: `slatec_sys::approximation::wnnls`
- Family: `Approximation`
- Review result: `machine-semantic-audit-passed`
- Arguments: `W` (input-output), `MDW` (input-output), `ME` (input), `MA` (input), `N` (input), `L` (input), `PRGOPT` (input-output), `X` (input-output), `RNORM` (output), `MODE` (output), `IWORK` (workspace-output), `WORK` (workspace-output)

## `XLEGF`

- Canonical path: `slatec_sys::special::xlegf`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `DNU1` (input), `NUDIFF` (input), `MU1` (input), `MU2` (input), `THETA` (input), `ID` (input), `PQA` (input-output), `IPQA` (input-output), `IERROR` (status-output)

## `XNRMP`

- Canonical path: `slatec_sys::special::xnrmp`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `NU` (input), `MU1` (input), `MU2` (input), `SARG` (input), `MODE` (input), `SPN` (output), `IPN` (input), `ISIG` (output), `IERROR` (status-output)

## `ZAIRY`

- Canonical path: `slatec_sys::special::zairy`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ZR` (input), `ZI` (input), `ID` (input), `KODE` (input), `AIR` (output), `AII` (output), `NZ` (status-output), `IERR` (output)

## `ZBESH`

- Canonical path: `slatec_sys::special::bessel::zbesh`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ZR` (input), `ZI` (input), `FNU` (input), `KODE` (input), `M` (input), `N` (input), `CYR` (output), `CYI` (output), `NZ` (status-output), `IERR` (output)

## `ZBESI`

- Canonical path: `slatec_sys::special::bessel::zbesi`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ZR` (input), `ZI` (input), `FNU` (input), `KODE` (input), `N` (input), `CYR` (output), `CYI` (output), `NZ` (status-output), `IERR` (output)

## `ZBESJ`

- Canonical path: `slatec_sys::special::bessel::zbesj`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ZR` (input), `ZI` (input), `FNU` (input), `KODE` (input), `N` (input), `CYR` (output), `CYI` (output), `NZ` (status-output), `IERR` (output)

## `ZBESK`

- Canonical path: `slatec_sys::special::bessel::zbesk`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ZR` (input), `ZI` (input), `FNU` (input), `KODE` (input), `N` (input), `CYR` (output), `CYI` (output), `NZ` (status-output), `IERR` (output)

## `ZBESY`

- Canonical path: `slatec_sys::special::bessel::zbesy`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ZR` (input), `ZI` (input), `FNU` (input), `KODE` (input-output), `N` (input), `CYR` (output), `CYI` (output), `NZ` (status-output), `CWRKR` (input), `CWRKI` (input), `IERR` (output)

## `ZBIRY`

- Canonical path: `slatec_sys::special::zbiry`
- Family: `Special functions`
- Review result: `machine-semantic-audit-passed`
- Arguments: `ZR` (input), `ZI` (input), `ID` (input), `KODE` (input), `BIR` (output), `BII` (output), `IERR` (output)
