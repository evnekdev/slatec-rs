C Packaged profile support for the selected GNU Fortran MinGW x86_64 ABI.
C This is build-layer compatibility code, not acquired SLATEC evidence.
      REAL FUNCTION R1MACH (I)
      INTEGER I
      IF (I .LT. 1 .OR. I .GT. 5) THEN
        CALL XERMSG ('SLATEC', 'R1MACH', 'I OUT OF BOUNDS', 1, 2)
        R1MACH = 0.0
        RETURN
      ENDIF
      IF (I .EQ. 1) R1MACH = TINY(0.0)
      IF (I .EQ. 2) R1MACH = HUGE(0.0)
      IF (I .EQ. 3) R1MACH = EPSILON(0.0) / REAL(RADIX(0.0))
      IF (I .EQ. 4) R1MACH = EPSILON(0.0)
      IF (I .EQ. 5) R1MACH = LOG10(REAL(RADIX(0.0)))
      RETURN
      END
