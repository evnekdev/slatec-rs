program slatec_runtime_profile_probe
  implicit none
  character(len=48) :: mode

  call get_command_argument(1, mode)
  select case (trim(mode))
  case ('constants')
    call probe_constants()
  case ('compiler')
    call probe_compiler()
  case ('initds')
    call probe_initds()
  case ('inits')
    call probe_inits()
  case ('dgamlm')
    call probe_dgamlm()
  case ('gamlim')
    call probe_gamlim()
  case ('ai')
    call probe_ai()
  case ('ei')
    call probe_ei()
  case ('reciprocal-gamma')
    call probe_reciprocal_gamma()
  case ('degree')
    call probe_degree()
  case ('warning-once')
    call xermsg('SLATEC', 'PROFILE', 'WARNING ONCE', 11, -1)
    write (*, '(A)') 'RETURNED'
  case ('warning')
    call xermsg('SLATEC', 'PROFILE', 'WARNING', 12, 0)
    write (*, '(A)') 'RETURNED'
  case ('recoverable-controlled')
    call xsetf(1)
    call xermsg('SLATEC', 'PROFILE', 'RECOVERABLE', 13, 1)
    write (*, '(A)') 'RETURNED'
  case ('recoverable-default')
    call xsetf(2)
    call xermsg('SLATEC', 'PROFILE', 'RECOVERABLE', 14, 1)
    write (*, '(A)') 'UNEXPECTED_RETURN'
  case ('fatal')
    call xsetf(0)
    call xermsg('SLATEC', 'PROFILE', 'FATAL', 15, 2)
    write (*, '(A)') 'UNEXPECTED_RETURN'
  case ('error-state')
    call probe_error_state()
  case ('invalid-machine-selector')
    call probe_invalid_machine_selector()
  case default
    error stop 64
  end select
contains
  subroutine probe_constants()
    integer :: i
    integer :: i1mach
    real :: r1mach
    double precision :: d1mach
    do i = 1, 16
      write (*, '(A,1X,I0,1X,I0)') 'I1MACH', i, i1mach(i)
    end do
    do i = 1, 5
      write (*, '(A,1X,I0,1X,ES25.17E3)') 'R1MACH', i, r1mach(i)
      write (*, '(A,1X,I0,1X,ES25.17E3)') 'D1MACH', i, d1mach(i)
    end do
  end subroutine

  subroutine probe_compiler()
    use iso_fortran_env, only: input_unit, output_unit, error_unit
    write (*, '(A,1X,I0)') 'INPUT_UNIT', input_unit
    write (*, '(A,1X,I0)') 'OUTPUT_UNIT', output_unit
    write (*, '(A,1X,I0)') 'ERROR_UNIT', error_unit
    write (*, '(A,1X,I0)') 'INTEGER_STORAGE_BITS', storage_size(0)
    write (*, '(A,1X,I0)') 'INTEGER_RADIX', radix(0)
    write (*, '(A,1X,I0)') 'INTEGER_DIGITS', digits(0)
    write (*, '(A,1X,I0)') 'INTEGER_HUGE', huge(0)
    write (*, '(A,1X,I0)') 'REAL_RADIX', radix(0.0)
    write (*, '(A,1X,I0)') 'REAL_DIGITS', digits(0.0)
    write (*, '(A,1X,I0)') 'REAL_MINEXPONENT', minexponent(0.0)
    write (*, '(A,1X,I0)') 'REAL_MAXEXPONENT', maxexponent(0.0)
    write (*, '(A,1X,ES25.17E3)') 'REAL_TINY', tiny(0.0)
    write (*, '(A,1X,ES25.17E3)') 'REAL_HUGE', huge(0.0)
    write (*, '(A,1X,ES25.17E3)') 'REAL_EPSILON', epsilon(0.0)
    write (*, '(A,1X,I0)') 'DOUBLE_RADIX', radix(0.0d0)
    write (*, '(A,1X,I0)') 'DOUBLE_DIGITS', digits(0.0d0)
    write (*, '(A,1X,I0)') 'DOUBLE_MINEXPONENT', minexponent(0.0d0)
    write (*, '(A,1X,I0)') 'DOUBLE_MAXEXPONENT', maxexponent(0.0d0)
    write (*, '(A,1X,ES25.17E3)') 'DOUBLE_TINY', tiny(0.0d0)
    write (*, '(A,1X,ES25.17E3)') 'DOUBLE_HUGE', huge(0.0d0)
    write (*, '(A,1X,ES25.17E3)') 'DOUBLE_EPSILON', epsilon(0.0d0)
  end subroutine

  subroutine probe_initds()
    double precision :: coefficients(3)
    integer :: initds
    coefficients = [1.0d0, 0.1d0, 0.01d0]
    write (*, '(A,1X,I0)') 'INITDS', initds(coefficients, 3, 0.05)
  end subroutine

  subroutine probe_inits()
    real :: coefficients(3)
    integer :: inits
    coefficients = [1.0, 0.1, 0.01]
    write (*, '(A,1X,I0)') 'INITS', inits(coefficients, 3, 0.05)
  end subroutine

  subroutine probe_dgamlm()
    double precision :: xmin, xmax
    call dgamlm(xmin, xmax)
    write (*, '(A,2(1X,ES25.17E3))') 'DGAMLM', xmin, xmax
  end subroutine

  subroutine probe_gamlim()
    real :: xmin, xmax
    call gamlim(xmin, xmax)
    write (*, '(A,2(1X,ES25.17E3))') 'GAMLIM', xmin, xmax
  end subroutine

  subroutine probe_ai()
    real :: ai
    write (*, '(A,1X,ES25.17E3)') 'AI', ai(0.0)
  end subroutine

  subroutine probe_ei()
    real :: ei
    write (*, '(A,1X,ES25.17E3)') 'EI', ei(1.0)
  end subroutine

  subroutine probe_reciprocal_gamma()
    double precision :: dgamr
    write (*, '(A,1X,ES25.17E3)') 'DGAMR1', dgamr(1.0d0)
    write (*, '(A,1X,ES25.17E3)') 'DGAMRHALF', dgamr(0.5d0)
  end subroutine

  subroutine probe_degree()
    real :: sindg, cosdg
    write (*, '(A,1X,ES25.17E3)') 'SINDG30', sindg(30.0)
    write (*, '(A,1X,ES25.17E3)') 'COSDG60', cosdg(60.0)
  end subroutine

  subroutine probe_invalid_machine_selector()
    integer :: i1mach
    write (*, '(A,1X,I0)') 'INVALID', i1mach(0)
  end subroutine

  subroutine probe_error_state()
    integer :: control, before_clear, after_clear
    call xsetf(1)
    call xgetf(control)
    call xermsg('SLATEC', 'PROFILE', 'STATE', 19, 1)
    call numxer(before_clear)
    call xerclr()
    call numxer(after_clear)
    write (*, '(A,3(1X,I0))') 'ERROR_STATE', control, before_clear, after_clear
  end subroutine
end program
