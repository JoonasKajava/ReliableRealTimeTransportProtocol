During development and using Mutexes to share data between threads I encountered deadlocks.

![[9acc85f0648afac0a572787dffdcbd69.gif]]
## Cause
These mutexes were shared with two threads:
![[Pasted image 20231203203134.png]]

And since some of these mutexes are locked at the same time like so:
![[Pasted image 20231203203323.png]]
And here:
![[Pasted image 20231203203445.png]]

It is very likely that given unfortunate timing, these mutexes will be cross locked by these two threads.

## Fix
I will rework this behaviour. I will most likely encapsulate these variables into a single struct and wrap that inside a Mutex.