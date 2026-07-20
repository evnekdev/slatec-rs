# Error handling

[All families](../routines-by-family.md) | [Alphabetical index](../routines-alphabetical.md) | [Coverage](../routine-coverage.md)

Retained routines classified by the authoritative catalogue under this mathematical family.

Retained identities: **17**. Canonical public raw routines: **0**. Secondary or terminally disposed identities: **17**.

## Public routines

Public routines are sorted by canonical SLATEC name.

<table>
<thead><tr><th>Routine</th><th>Purpose</th><th>Role</th><th>Precision</th><th>Storage/problem class</th><th>Operation</th><th>Raw API status</th><th>Safe API status</th><th>Canonical Rust path</th></tr></thead>
<tbody>
</tbody>
</table>

## Internal, support, and historical identities

These records remain part of the retained catalogue but are not additional public raw routines.

<table>
<thead><tr><th>Routine</th><th>Purpose</th><th>Role</th><th>Precision</th><th>Storage/problem class</th><th>Operation</th><th>Final disposition</th><th>Documentation quality</th></tr></thead>
<tbody>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/fdump.md"><code>FDUMP</code></a></td><td>Symbolic dump (should be locally written).</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/j4save.md"><code>J4SAVE</code></a></td><td>Save or recall global variables needed by error handling routines.</td><td><code>subsidiary</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>subsidiary_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xerbla.md"><code>XERBLA</code></a></td><td>Error handler for the Level 2 and Level 3 BLAS Routines.</td><td><code>subsidiary</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>subsidiary_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xerclr.md"><code>XERCLR</code></a></td><td>Reset current error number to zero.</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xercnt.md"><code>XERCNT</code></a></td><td>Allow user control over handling of errors.</td><td><code>subsidiary</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>subsidiary_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xerdmp.md"><code>XERDMP</code></a></td><td>Print the error tables and then clear them.</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xerhlt.md"><code>XERHLT</code></a></td><td>Abort program execution and print error message.</td><td><code>subsidiary</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>subsidiary_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xermax.md"><code>XERMAX</code></a></td><td>Set maximum number of times any error message is to be printed.</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xermsg.md"><code>XERMSG</code></a></td><td>Process error messages for SLATEC and other libraries.</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xerprn.md"><code>XERPRN</code></a></td><td>Print error messages processed by XERMSG.</td><td><code>subsidiary</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>subsidiary_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xersve.md"><code>XERSVE</code></a></td><td>Record that an error has occurred.</td><td><code>subsidiary</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>subsidiary_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xgetf.md"><code>XGETF</code></a></td><td>Return the current value of the error control flag.</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xgetua.md"><code>XGETUA</code></a></td><td>Return unit number(s) to which error messages are being sent.</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xgetun.md"><code>XGETUN</code></a></td><td>Return the (first) output file to which error messages are being sent.</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xsetf.md"><code>XSETF</code></a></td><td>Set the error control flag.</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xsetua.md"><code>XSETUA</code></a></td><td>Set logical unit numbers (up to 5) to which error messages are to be sent.</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
<tr class="routine-secondary" style="opacity:0.76"><td><a href="../routines/xsetun.md"><code>XSETUN</code></a></td><td>Set output file to which error messages are to be sent.</td><td><code>user_callable</code></td><td><code>unknown</code></td><td><code>not_applicable</code></td><td><code>utility</code></td><td><code>error-support</code></td><td><code>support_unit_minimal</code></td></tr>
</tbody>
</table>
