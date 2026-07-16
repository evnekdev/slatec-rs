C GNU MinGW profile site hook.  Preserve fatal termination, but use a
C deterministic nonzero status so a parent process can distinguish failure.
      SUBROUTINE XERHLT (MESSG)
      CHARACTER*(*) MESSG
      STOP 70
      END
