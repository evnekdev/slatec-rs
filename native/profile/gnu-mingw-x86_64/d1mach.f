C Profile support for the selected GNU Fortran MinGW x86_64 ABI.
C This is build-layer compatibility code, not acquired SLATEC evidence.
      DOUBLE PRECISION FUNCTION D1MACH (I)
      INTEGER I
      IF (I .LT. 1 .OR. I .GT. 5) THEN
        CALL XERMSG ('SLATEC', 'D1MACH', 'I OUT OF BOUNDS', 1, 2)
        D1MACH = 0.0D0
        RETURN
      ENDIF
      IF (I .EQ. 1) D1MACH = TINY(0.0D0)
      IF (I .EQ. 2) D1MACH = HUGE(0.0D0)
      IF (I .EQ. 3) D1MACH = EPSILON(0.0D0) /
     +                            DBLE(RADIX(0.0D0))
      IF (I .EQ. 4) D1MACH = EPSILON(0.0D0)
      IF (I .EQ. 5) D1MACH = LOG10(DBLE(RADIX(0.0D0)))
      RETURN
      END
