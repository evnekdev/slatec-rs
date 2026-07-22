C Packaged profile support for the selected GNU Fortran Linux x86_64 ABI.
C This is build-layer compatibility code, not acquired SLATEC evidence.
      INTEGER FUNCTION I1MACH (I)
      USE ISO_FORTRAN_ENV, ONLY: INPUT_UNIT, OUTPUT_UNIT, ERROR_UNIT
      INTEGER I, VALUES(16)
      VALUES(1) = INPUT_UNIT
      VALUES(2) = OUTPUT_UNIT
      VALUES(3) = 0
      VALUES(4) = ERROR_UNIT
      VALUES(5) = BIT_SIZE(0)
      VALUES(6) = STORAGE_SIZE(0) / 8
      VALUES(7) = RADIX(0)
      VALUES(8) = DIGITS(0)
      VALUES(9) = HUGE(0)
      VALUES(10) = RADIX(0.0)
      VALUES(11) = DIGITS(0.0)
      VALUES(12) = MINEXPONENT(0.0)
      VALUES(13) = MAXEXPONENT(0.0)
      VALUES(14) = DIGITS(0.0D0)
      VALUES(15) = MINEXPONENT(0.0D0)
      VALUES(16) = MAXEXPONENT(0.0D0)
      IF (I .LT. 1 .OR. I .GT. 16) THEN
        CALL XERMSG ('SLATEC', 'I1MACH', 'I OUT OF BOUNDS', 1, 2)
        I1MACH = 0
        RETURN
      ENDIF
      I1MACH = VALUES(I)
      RETURN
      END
